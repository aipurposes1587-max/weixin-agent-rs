# wechat-rs-sdk

![CI](https://github.com/tianrking/weixin-agent-rs/actions/workflows/ci.yml/badge.svg)
![Rust](https://img.shields.io/badge/Rust-1.78%2B-orange)
![License](https://img.shields.io/badge/License-MIT-blue)
![Version](https://img.shields.io/badge/Version-v0.0.1-brightgreen)

一个现代化的 Rust WeChat iLink Bot SDK，支持可插拔 Agent，并内置统一启动器 `wechat-agent`。

语言版本：
- 中文（默认）：`README.md`
- English: [README.en.md](./README.en.md)
- Español: [README.es.md](./README.es.md)

## 功能概览

- 扫码登录（`get_bot_qrcode`、`get_qrcode_status`）
- 长轮询收消息（`getupdates`）+ `get_updates_buf` 断点续拉
- 文本发送（`sendmessage`）
- 媒体上传（`getuploadurl` + CDN）
- 媒体下载解密（AES-128-ECB）
- 打字状态（`getconfig`、`sendtyping`）
- 会话过期处理（`errcode = -14`）
- 多账号本地凭据持久化
- Agent 抽象层，可接 OpenAI / Anthropic / ACP / 自研后端

## 快速开始

```bash
cd /Volumes/ok/Linux_dev_rewrite/wechat_dev/wechat-rs-sdk
cargo check --all-targets

# 一条命令（登录 + 启动 Codex ACP）
cargo run --bin wechat-agent -- --login --agent codex
```

## 一条命令接入本地 Agent

```bash
# Claude Code ACP
cargo run --bin wechat-agent -- --login --agent claude

# Codex ACP
cargo run --bin wechat-agent -- --login --agent codex

# OpenClaw ACP
cargo run --bin wechat-agent -- --login --agent openclaw
```

强制指定账号（多账号场景强烈建议）：

```bash
cargo run --bin wechat-agent -- --agent claude --account <account_id>
```

`account_id` 可从登录输出获得，例如：`login success: xxx-im-bot`。

## 其他 Agent

```bash
# OpenAI
OPENAI_API_KEY=... cargo run --bin wechat-agent -- --agent openai

# Anthropic
ANTHROPIC_API_KEY=... cargo run --bin wechat-agent -- --agent anthropic
```

## 运行时行为

- 入站日志会打印：发送方、消息类型、文本预览。
- 出站日志会打印：回复类型（`text` / `media` / `fallback`）。
- 当 Agent 返回空响应时，会自动发送兜底文本：
  - `（模型本轮未返回内容，请再发一次）`

## 排障

- `session expired (errcode -14)`：token 过期，请重新登录，或强制指定账号：
  - `cargo run --bin wechat-agent -- --agent claude --account <account_id>`
- 多账号场景下建议始终加 `--account <account_id>`，避免命中旧 token。

## 示例

- `echo`：最小回声机器人
- `openai`：OpenAI Chat Completions
- `anthropic`：Anthropic Messages API
- `acp`：ACP 子进程适配（Claude / Codex / Kimi 等）

## CI

已包含 GitHub Actions 三平台 CI：

- Ubuntu
- Windows
- macOS

配置文件：`.github/workflows/ci.yml`
