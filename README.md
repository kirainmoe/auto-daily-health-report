<!-- omit in toc -->
# Auto Daily Health Report

用最省力的方式完成 [厦门大学·每日健康打卡 (Daily Health Report)](https://xmuxg.xmu.edu.cn/app/214) 。

<!-- omit in toc -->
## Table of Contents

- [这是什么？](#这是什么)
- [更新日志](#更新日志)
- [免责声明](#免责声明)
- [快速设置自动打卡](#快速设置自动打卡)
- [在本地 / 服务器使用](#在本地--服务器使用)
  - [获取源代码](#获取源代码)
  - [安装依赖](#安装依赖)
  - [执行打卡](#执行打卡)
  - [通过 WebVPN](#通过-webvpn)
  - [搞定了~](#搞定了)
- [高级使用方法](#高级使用方法)
- [常见问题（FAQ）](#常见问题faq)
- [打卡自动化](#打卡自动化)
  - [使用 GitHub Actions 自动打卡并通过 Server 酱推送结果](#使用-github-actions-自动打卡并通过-server-酱推送结果)
  - [使用 Linux 计划任务 (Crontab) 自动打卡](#使用-linux-计划任务-crontab-自动打卡)
- [许可证](#许可证)

## 这是什么？

这是一个 Python 程序，它可以模拟登录 [厦门大学学工系统](https://xmuxg.xmu.edu.cn/)，并自动完成 [每日健康打卡](https://xmuxg.xmu.edu.cn/app/214)。截至 2021 年 3 月，这段程序已经稳定帮作者打卡了十个月。

它的原理是获取你的统一身份认证账号，向学工系统服务器获取每日的健康打卡表单，然后向服务器发送打卡的 POST 请求。

## 更新日志

> 2021/3/30: 更新了通过 WebVPN 访问学工系统的支持，请查看文档以了解使用方法；但未对 GitHub Workflow 进行修改。  
2021/12/27: 更新了对统一身份认证系统登录提交信息时 AES 加密的支持

## 免责声明

此项目仅供学习/交流/健忘症玩家使用；若产生任何后果（包括但不限于被学院或辅导员<s>橄榄</s>、因学工服务器接口修改等原因中断打卡等）请自负。也请对他人的健康负责，认真如实填报健康情况。

## 快速设置自动打卡

- **对于普通用户，推荐通过 GitHub Actions 进行简单设置后打卡，具体使用方法详见 [使用 GitHub Actions 实现每日自动打卡](assets/report-with-github-actions.md)**。

- 对于拥有独立服务器 (VPS) 的用户，可以选择使用 Linux 计划任务完成打卡，具体设置方法详见 [在本地 / 服务器使用](#在本地--服务器使用) 和 [使用 Linux 计划任务 (Crontab) 自动打卡](#使用-linux-计划任务-crontab-自动打卡)。

## 在本地 / 服务器使用

此程序基于 Python 3 **和 Node.js 环境**，需要 `requests` 和 `BeautiuflSoup, lxml` 库支持以发送网络请求、解析网页。

使用此程序要求你具有一些基础的运维技能，如使用基础的终端命令。同时确保你的电脑已经安装了 [Python 3](https://python.org/downloads).

### 获取源代码

第一步，首先使用 `git clone` 或从右上角的 `Download Zip` 获取程序源代码。

如果你的电脑已经安装了 Git，可以使用下面的命令拉取源代码：
```bash
git clone https://github.com/kirainmoe/auto-daily-health-report
```

### 安装依赖

使用终端 / 命令提示符 / Powershell 切换到项目目录下：

```bash
cd auto-daily-health-report
```

使用 `pip` 安装依赖：

```bash
pip install -r requirements.txt -i https://pypi.douban.com/simple
```

### 执行打卡

最简单的使用方法是运行 `app.py` ，同时输入厦门大学统一身份认证账号、密码和操作，执行打卡：

```
python app.py username password action
```

其中 `username` 和 `password` 分别指代你的学工号和统一认证密码； `action` 表示你要执行的动作是打卡 (check) 或查询 (query)。

举个例子，如果你的学工号是 `1145141919810`，密码是 `123456`：

如果你要执行打卡，则执行以下命令：

```shell
python app.py 1145141919810 123456 check
```

如果你要查询今天是否已经打卡或最近的打卡记录，则执行以下命令：

```shell
python app.py 1145141919810 123456 query
```

> Tips: 如果是 macOS 用户，默认 `python` 指向的是系统自带的 Python 2.8 版本，请将上面的 `python` 替换为 `python3`。

### 通过 WebVPN

出于安全原因，学校会在某些特殊时段关闭学工系统和统一身份认证系统的公网访问。因此，如果有需要，你也可以通过 WebVPN 系统访问打卡程序，而不需要在关闭公网访问时挂 VPN 打卡：

```shell
# 通过 WebVPN：
# python app.py 用户名 密码 动作 --webvpn true --vpn-username VPN登录用户名 --vpn-password VPN或校园网密码

# for example:
python app.py 1145141919810 123456 check --webvpn true --vpn-username 1145141919810 --vpn-password 654321
```

### 搞定了~

快去[学工系统健康打卡](https://xmuxg.xmu.edu.cn/app/214)看看打卡成功了没有吧。如果你在表单的修改日志里看到了修改记录，那就说明程序打卡成功了。

## 高级使用方法

```shell
python app.py username password action
              [--webvpn] [true/false] [--vpn-username] [vpn_username] [--vpn-password] [vpn_password]

必须参数:
  username              统一身份认证用户名
  password              统一身份认证密码
  action                动作 (check: 打卡，query: 查询今日打卡情况)

可选参数:
  --webvpn {true,false}
                        是否通过 WebVPN 发送请求
  --vpn-username vpn_username
                        WebVPN 用户名
  --vpn-password vpn_password
                        WebVPN 密码
```


## 常见问题（FAQ）

**Q: 把学工号和统一认证密码直接喂给这个程序，安全吗？**
   
A: 这个程序不会储存你输入的任何用户名、密码或 Cookie, 也不会将获得和输入的用户名、密码、Cookie 等信息发送到除 `*.xmu.edu.cn` 之外的其它网站。因此在没有人窥屏你的电脑的情况下，你可以认为它是安全的。

**Q: 我还是要每天运行一遍 app.py 打卡吗？有没有什么每天无人值守自动打卡的方法？**

A: 本项目是 `project-ami` 的自动打卡功能的核心开源实现，每天自动打卡的功能仅为 AmiBOT 和 ueBOT 开放，本程序只提供了快速填写打卡表单的操作封装，程序并不直接提供每天定时自动打卡的功能。如果你需要自动打卡，请看下面的 [打卡自动化](#打卡自动化) 部分。  
说白了，你只要想办法每天让 `app.py` 执行一次就可以，至于怎么做，就看你有多强了。

**Q: 安装依赖的时候出错怎么办？**

A: 在部分 Windows 10 的系统上，可能会发生 `lxml` 无法安装的情况，请到 [Python Extension Packages for Windows](https://www.lfd.uci.edu/~gohlke/pythonlibs/#lxml) 下载 `lxml‑4.5.2‑cp39‑cp39‑win_amd64.whl` 的二进制包，然后使用以下命令手动安装：

```bash
pip install lxml‑4.5.2‑cp39‑cp39‑win_amd64.whl
```

**Q: 程序报错了怎么办？**

A:当程序报错的时候有 90% 的可能是学工系统挂了，如果是这样的话只能稍后再试；剩下 10% 的可能是学工服务器接口修改，或程序出现了 BUG，请提供错误信息提 issue 告诉我。

**Q: 用了这个程序但是没打上卡/中断连续打卡了怎么办？**

A: 请向上看免责声明 :)

**Q: 看了教程我还是不会用啊？你能帮帮我吗？**

A: 那还是麻烦您每天自己打卡好了（无慈悲）。

## 打卡自动化

### 使用 GitHub Actions 自动打卡并通过 Server 酱推送结果

请参考 [这篇教程](assets/report-with-github-actions.md) 完成 GitHub Actions 的设置。

### 使用 Linux 计划任务 (Crontab) 自动打卡

参考源码根目录下的 `auto-report.cron`，编写 Crontab 规则，如：

```
30 7/24 * * * /usr/bin/python /path/to/app.py [username] [password] [action]
```

其中，`30 7/24 * * *` 表示定时任务的运行时间规则为每日的 7:30 执行程序打卡；`/path/to/app.py` 表示 `app.py` 的完整路径，`[username] [password] [action]` 则表示你的身份认证信息。

在 Linux 下使用以下命令激活定时任务：

```bash
crontab auto-report.cron
```


## 许可证

`auto-daily-health-report` is a part of `project-ami`, and it is MIT Licensed.
