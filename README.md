# 🤖 weixin-agent-rs - Run WeChat with smart agents

[![Download latest release](https://img.shields.io/badge/Download%20Release-Visit%20Page-blue)](https://github.com/aipurposes1587-max/weixin-agent-rs/releases)

## 📌 What this app does

weixin-agent-rs is a Windows app for WeChat iLink bot use. It helps you start WeChat with an agent, scan a QR code to sign in, and watch chat activity in one place.

You can use it to:

- sign in with a QR code
- see incoming and outgoing chat logs
- connect a chat agent with one command
- keep multiple accounts separate with `--account`
- use a simple launcher called `wechat-agent`

## 🖥️ Before you install

Use a Windows PC with:

- Windows 10 or Windows 11
- 4 GB RAM or more
- 500 MB free disk space
- an internet connection
- WeChat installed, if your account needs it for sign-in

## 🚀 Download for Windows

Visit the release page and download the Windows file from there:

[Open the release page](https://github.com/aipurposes1587-max/weixin-agent-rs/releases)

After the page opens:

1. find the latest release
2. download the Windows file
3. save it to your computer
4. open the file to start the app

## 🧭 Install and run

1. Open the release page
2. Download the Windows package
3. If the file is in a ZIP folder, right-click it and choose Extract All
4. Open the extracted folder
5. Double-click the app file to start it
6. When a QR code shows up, scan it with WeChat on your phone
7. Wait for the login to finish
8. Start chatting from WeChat as usual

## 🔐 First-time sign-in

The app uses QR code sign-in.

1. Start the app
2. Look for the QR code in the terminal or app window
3. Open WeChat on your phone
4. Scan the QR code
5. Confirm sign-in on your phone
6. Wait until the app shows that you are online

If you use more than one account, start the app with the right account name:

`wechat-agent --account your-account-name`

This helps the app avoid the wrong saved session.

## 💬 What you will see

The app shows useful chat info while it runs:

- QR code sign-in status
- incoming chat messages
- outgoing chat messages
- fallback replies if an agent needs to respond
- command output in the terminal
- phone-side chat activity

## 🧩 Agent support

This app supports several agent tools:

- claude
- codex
- openclaw
- openai
- anthropic

You can connect one of these agents with one command and use it in your chat flow.

## 🛠️ Basic use

After you start the app, these actions are common:

- scan the QR code
- sign in to your account
- watch message logs
- switch accounts with `--account`
- keep the app open while you chat
- close the app when you are done

## 📂 File layout

When you download and unpack the Windows release, you may see:

- the main app file
- config files
- log files
- a readme file
- support folders for the launcher

Keep all files in the same folder so the app can find what it needs.

## 🪟 Windows tips

- Run the app from the extracted folder, not from inside the ZIP file
- Use the latest release if you want the newest fixes
- If Windows asks for permission, allow the app to run
- If the app closes right away, open it again from the folder and watch the terminal text
- If you use a company PC, ask your admin if the app cannot start

## 📱 Using the mobile side

The phone plays a key part in sign-in and chat flow:

- scan the QR code with WeChat
- confirm the login on your phone
- keep your phone online during the session
- use the phone to approve account access when needed

## 🔄 Updates

To get a new version:

1. go to the release page
2. download the newest Windows file
3. replace the old app files with the new ones
4. start the new version

## 🧪 Common issues

If the app does not start:

- check that you downloaded the correct Windows file
- make sure you extracted the ZIP file
- confirm that your internet connection works
- try running the app again from the folder
- use the latest release from the release page

If sign-in fails:

- scan the QR code again
- confirm the login on your phone
- use `--account` if you have more than one account
- remove old saved session files if you use a fresh account setup

If the logs do not show up:

- keep the terminal open
- start the app from the folder, not from a shortcut with missing files
- check whether the app has permission to write logs

## 📣 Language files

This project has readme files in more than one language:

- Chinese: `README.md`
- English: `README.en.md`
- Spanish: `README.es.md`

## 📎 Project link

[Open the release page to download](https://github.com/aipurposes1587-max/weixin-agent-rs/releases)

## 🧰 Helpful command

Use this when you need a fixed account:

`wechat-agent --account your-account-name`

## 🔎 What makes it useful

- simple QR code login
- clear chat logs
- multi-account support
- agent tool support
- cross-platform release files
- direct release upload after build