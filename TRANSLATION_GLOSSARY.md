# LiquidLauncher 汉化术语表（必须严格遵守）

## 硬性规则

1. **只改用户可见的英文文本**。绝不修改：CSS 类名、变量名、函数名、import 路径、URL、
   枚举值（`value:` 字段）、图标名（`icon="..."`）、`filters`、事件名、`id`、`slot`。
2. `{ value: "automatic", text: "Automatic" }` → **只翻译 `text:`**，`value:` 保持原样。
3. 模板里的插值 `{...}` 原样保留，不要动里面的表达式。
   例：`title="Link worlds ({count})"` → `title="关联存档 ({count})"`。
4. HTML 实体保留：`&hellip;`、`&bull;` 等不要改写成字符。
5. **不要修改 `<style>` 块和 `<script>` 块里的代码**，但 `<script>` 里的
   `alert("...")` / `confirm("...")` / `prompt("...")` 文案要翻译。
6. 保持 Svelte 语法完整、引号配对、转义正确。改完必须仍是合法 Svelte。
7. 中文与英文/数字之间加一个半角空格（如 `启动 Minecraft`、`1.7 版本`）。
8. 标点用中文全角（，。：？！），但括号视语境可用半角。
9. **不翻译的专有名词**：LiquidBounce、Minecraft、Nextgen、ScriptAPI、JVM、Discord、
   GitHub、Twitter、Forum、Cloudflare WARP、GraalVM、Eclipse Temurin、Azul Zulu、
   以及所有 URL、版本号。
10. 语气：简洁、面向普通玩家，不要机翻腔。

## Rust 侧额外规则

- **只翻译会显示给用户的字符串**：`report(&app, ProgressUpdate::set_label(...))` 里的进度文案、
  `dialog().message(...)` / `.title(...)` 的弹窗、会冒泡到 UI 的错误消息。
- **`info!` / `debug!` / `trace!` / `warn!` / `error!` 这些 tracing 日志宏里的内容保持英文**，
  它们只写进 launcher.log，翻成中文反而妨碍排查问题。

## 统一译名

| 英文 | 中文 |
|---|---|
| Launch | 启动 |
| Terminate | 结束进程 |
| Settings | 设置 |
| Account | 账户 |
| Log in / Login | 登录 |
| Logout / Sign out | 退出登录 |
| Username | 用户名 |
| Offline login | 离线登录 |
| Version | 版本 |
| Build | 构建 |
| Latest | 最新 |
| Nightly builds | 每日构建 |
| Recommended mods | 推荐模组 |
| Additional mods | 附加模组 |
| Custom mods | 自定义模组 |
| Install | 安装 |
| Download | 下载 |
| Downloading... | 正在下载... |
| Memory | 内存 |
| Concurrent Downloads | 并发下载数 |
| Keep launcher running | 保持启动器运行 |
| Data Location | 数据目录 |
| Minecraft Directory | Minecraft 目录 |
| Auto-detect | 自动检测 |
| Link worlds | 关联存档 |
| Resource packs | 资源包 |
| Shader packs | 光影包 |
| JVM Distribution | JVM 发行版 |
| Automatic / Manual / Custom | 自动 / 手动 / 自定义 |
| Custom JVM Path | 自定义 JVM 路径 |
| Premium | 高级版 |
| Skip Advertisements | 跳过广告 |
| Cape | 披风 |
| Copy / Copied | 复制 / 已复制 |
| Cancel | 取消 |
| Continue | 继续 |
| Confirm | 确认 |
| Client log | 客户端日志 |
| Upload log | 上传日志 |
| Auto scroll | 自动滚动 |
| Security Warning | 安全警告 |
| Error Occurred | 发生错误 |
| Technical Details | 技术细节 |
| Quick Help | 快速帮助 |
| Contact Support | 联系支持 |
| Waiting for confirmation | 等待确认 |
| Scan the code | 扫码登录 |
| Quick Start Guide | 快速入门指南 |
| Unsupported version | 不受支持的版本 |
| Show nightly builds | 显示每日构建 |
| Select version | 选择版本 |
| User hash | 用户标识 |
| Paste | 粘贴 |
