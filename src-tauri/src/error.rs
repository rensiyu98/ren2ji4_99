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
use anyhow::{anyhow, Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LauncherError {
    #[error("无效的版本配置：{0}")]
    InvalidVersionProfile(String),
    #[error("未知的模板参数：{0}")]
    UnknownTemplateParameter(String),
}

pub fn map_into_connection_error(e: Error) -> Error {
    anyhow!(
        "文件下载失败。这可能是网络连接问题导致的。请尝试使用 Cloudflare WARP 等 VPN。\n\n错误：{}",
        e
    )
}