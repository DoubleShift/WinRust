# rust-taskbar —— WinRust 任务栏定制版

基于 [Lemxy/WinRust](https://github.com/Lemxy/WinRust) v0.3.0 的任务栏（topbar）深度定制。上游是一套完整的 Windows shell 替换（安装器 + 搜索 + 任务管理器 + 文件管理器等），本仓库只保留任务栏本体并做了大幅改进，其余组件已移除。

![logo](assets/logo.svg)

## 功能

- Windows 11 浅色任务栏风格（默认浅色，右键菜单可切深色）
- 中文界面
- Win 徽标置顶头；固定与运行中图标**拖动排序**，像素级跟手，顺序持久化
- 多实例应用：点任务栏按钮 = 该应用的所有窗口按层叠顺序全部拉到前台
- 开始面板：XP 双栏风——左栏搜索 + 固定项 + 开始菜单文件夹（点击展开），右栏 此电脑 / 文档 / 图片 / 下载 / 网络 / 设置
- 停靠位置可设：下 / 上 / 左 / 右（左右为竖排栏）
- 屏幕右缘"显示桌面"占位符：迷你 Win 徽标，单击回桌面
- 全屏应用自动让位；桌面与普通窗口下常驻不隐藏
- 托盘图标全部直接显示，不折叠

## 构建

需要 Rust stable 1.85+（edition 2024），二选一：

- MSVC 工具链：`cargo build --release -p topbar`
- mingw-w64 交叉编译（可在 Linux 云机）：`rustup target add x86_64-pc-windows-gnu` 后 `cargo build --release -p topbar --target x86_64-pc-windows-gnu`

产物 `target\release\topbar.exe`（约 750 KB），单文件直接运行；参数 `--restore` 恢复 Windows 任务栏。

## 配置

全部配置在**一个文件**里：exe 同层的 `taskbar.json`（首次运行时自动从旧版分文件迁移）。

```json
{
  "theme": "light",          // 浅色 / 深色（"dark"）
  "dock": "bottom",          // 停靠位置：bottom / top / left / right
  "order": [ "…lnk 路径…" ],  // 图标拖动后的固定顺序
  "start": [ "…路径…" ],      // 开始面板固定项
  "usage": { "名字": 次数 }   // 启动频率（搜索排序用）
}
```

改动立即生效（停靠/深浅色在右键菜单里改也行）。语言：%ProgramData%\WinRust\lang.txt 写 zh（中文）/ en / ru。

## 右键菜单

在栏空白处右键：Windows 设置、任务管理器、停靠位置（下/上/左/右）、深色主题、浅色主题、关闭此栏。

## 致谢

基于 [Lemxy/WinRust](https://github.com/Lemxy/WinRust)——上游包含完整的 shell 替换套件（搜索、任务管理器、文件管理器、磁盘工具等），本仓库是其任务栏部分的定制分支。
