/*
 * This file is part of LiquidLauncher (https://github.com/CCBlueX/LiquidLauncher)
 *
 * Copyright (c) 2015 - 2024 CCBlueX
 *
 * LiquidLauncher is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * LiquidLauncher is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with LiquidLauncher. If not, see <https://www.gnu.org/licenses/>.
 */
use tracing::info;

/// 汉化版改动：自动更新已被禁用。
///
/// 上游实现会在每次启动时向 CCBlueX 的更新接口查询新版本，并静默下载安装
/// 官方签名的构建包。那会把本地的汉化版整个覆盖回英文原版，因此这里保留
/// 命令本身（前端 `src/lib/Window.svelte` 仍会调用它），但不再做任何网络请求。
#[tauri::command]
pub(crate) async fn check_for_updates() -> Result<(), String> {
    info!("Automatic updates are disabled in this localized build, skipping update check");
    Ok(())
}
