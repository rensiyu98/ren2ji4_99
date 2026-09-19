use anyhow::{Context, Result};
use futures::{stream, StreamExt};
use path_absolutize::Absolutize;
use std::fmt::Write;
use std::{collections::HashSet, path::Path};
use tokio::fs::OpenOptions;
use backon::ExponentialBuilder;
use backon::Retryable;

use crate::{
    error::LauncherError,
    minecraft::{
        progress::{ProgressReceiver, ProgressUpdate, ProgressUpdateSteps},
        rule_interpreter,
        version::{LibraryDownloadInfo, VersionProfile},
    },
    utils::{zip_extract, OS},
};

use super::{LauncherData, StartParameter};

pub async fn setup_libraries<D: Send + Sync>(
    libraries_folder: &Path,
    natives_folder: &Path,
    version_profile: &VersionProfile,
    launching_parameter: &StartParameter,
    launcher_data: &LauncherData<D>,
    features: &HashSet<String>,
    class_path: &mut String,
) -> Result<()> {
    let libraries_to_download = version_profile
        .libraries
        .iter()
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    let libraries_max = libraries_to_download.len() as u64;

    launcher_data.progress_update(ProgressUpdate::set_label("正在检查依赖库..."));
    launcher_data.progress_update(ProgressUpdate::set_for_step(
        ProgressUpdateSteps::DownloadLibraries,
        0,
        libraries_max,
    ));

    let class_paths: Vec<Result<Option<String>>> =
        stream::iter(libraries_to_download.into_iter().filter_map(|library| {
            // let download_count = libraries_downloaded.clone();
            let folder_clone = libraries_folder.to_path_buf();
            let native_clone = natives_folder.to_path_buf();

            if !rule_interpreter::check_condition(&library.rules, &features).unwrap_or(false) {
                return None;
            }

            Some(async move {
                if let Some(natives) = &library.natives {
                    if let Some(required_natives) = natives.get(OS.get_simple_name()?) {
                        if let Some(classifiers) = library
                            .downloads
                            .as_ref()
                            .and_then(|x| x.classifiers.as_ref())
                        {
                            if let Some(artifact) = classifiers
                                .get(required_natives)
                                .map(LibraryDownloadInfo::from)
                            {
                                let path = (|| async { artifact.download(&library.name, folder_clone.clone(), launcher_data).await })
                                    .retry(ExponentialBuilder::default())
                                    .notify(|err, dur| {
                                        launcher_data.log(&format!(
                                            "Failed to download native library: {}. Retrying in {:?}. Error: {}",
                                            &library.name, dur, err
                                        ));
                                    })
                                    .await
                                    .with_context(|| {
                                        format!(
                                            "下载 native 库失败：{}",
                                            &library.name
                                        )
                                    })?;

                                launcher_data.progress_update(ProgressUpdate::set_label(
                                    "正在解压 natives...",
                                ));
                                let file = OpenOptions::new()
                                    .read(true)
                                    .open(path)
                                    .await
                                    .context("打开 native 库失败")?;
                                zip_extract(file, &native_clone)
                                    .await
                                    .context("解压 native 库失败")?;
                            }
                        } else {
                            return Err(LauncherError::InvalidVersionProfile(
                                "缺少 classifiers，但需要 natives。".to_string(),
                            )
                            .into());
                        }
                    }

                    return Ok(None);
                }



                // Download regular artifact
                let artifact = library.get_library_download()?;

                let path = (|| async {  artifact.download(&library.name, folder_clone.clone(), launcher_data).await })
                    .retry(ExponentialBuilder::default())
                    .notify(|err, dur| {
                        launcher_data.log(&format!(
                            "Failed to download library: {}. Retrying in {:?}. Error: {}",
                            &library.name, dur, err
                        ));
                    })
                    .await
                    .with_context(|| format!("下载依赖库失败：{}", &library.name))?;

                // Natives are not included in the classpath
                return if library.natives.is_none() {
                    return Ok(path.absolutize()?.to_str().map(|x| x.to_string()));
                } else {
                    Ok(None)
                };
            })
        }))
        .buffer_unordered(launching_parameter.concurrent_downloads as usize)
        .collect()
        .await;

    // Join class paths
    for x in class_paths {
        if let Some(library_path) = x? {
            write!(class_path, "{}{}", &library_path, OS.get_path_separator()?)?;
        }
    }

    launcher_data.progress_update(ProgressUpdate::set_for_step(
        ProgressUpdateSteps::DownloadLibraries,
        libraries_max,
        libraries_max,
    ));

    Ok(())
}
