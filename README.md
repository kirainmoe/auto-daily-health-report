# XMU Daily Health Report - Rust CLI & SDK

> [旧版 auto-daily-health-report，点这里](https://github.com/kirainmoe/auto-daily-health-report/tree/legacy)  
> Python 实现的 auto-daily-health-report 后续将不再维护，请使用 Rust 版 CLI 程序打卡，或使用 Rust 版本 SDK 开发程序。

一个完成 [厦门大学·每日健康打卡 (Daily Health Report)](https://xmuxg.xmu.edu.cn/app/214) 的命令行工具和 Rust 语言 SDK.

# 使用方法

Daily Health Report 需要 Rust + Cargo 环境。请参照 [Rust 官网入门教程](https://www.rust-lang.org/zh-CN/learn/get-started) 安装 Rust 环境。

如果你在下载过程中遇到问题，请考虑更换 [RsProxy 源](https://rsproxy.cn/).

## 作为命令行工具 (CLI) 使用

安装 `CLI`：

```shell
$ cargo install xmu-health-report-rust-sdk
```

使用：

```shell
$ xmu-health-report-rust-sdk

XMU Daily Health Report Rust CLI - 厦门大学每日健康打卡工具 1.0

USAGE:
    xmu-health-report-rust-sdk [SUBCOMMAND]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help        Print this message or the help of the given subcommand(s)
    pipeline    Start a health report pipeline - 启动健康打卡流程
    query       Query today's health report status - 查询今日打卡状态
    report      Make health report request - 发送健康打卡请求
```

## 作为项目依赖使用

在 `Cargo.toml` 中添加：

```toml
[depenencies]
xmu-health-report-rust-sdk = "0.1.7"
```

另请参考 [docs.rs](https://docs.rs/xmu-health-report-rust-sdk/latest/xmu_health_report_rust_sdk/) 中 crate 的接口定义。

# License

MIT 

