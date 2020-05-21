# Auto Daily Health Report

这是一个模拟自动登录厦门大学学工系统，并自动完成厦门大学每日健康打卡 (Daily Health Report) (https://xmuxg.xmu.edu.cn/app/214) 的 Python 程序。

仅供学习/交流/健忘症使用；若产生任何后果请自负。

## 下载 & 安装所需库

此程序基于 Python 3，需要 `requests` 和 `BeautiuflSoup, lxml` 库支持以发送网络请求、解析网页。程序在 Python 3.6, macOS 10.15 环境下测试。

首先使用 `git clone` 或从右上角的 `Download Zip` 获取程序源代码，然后执行 `pip install`：

```bash
git clone https://github.com/kirainmoe/auto-daily-health-report

pip install -r requirements.txt -i https://pypi.douban.com
```

## 使用

### 执行打卡

由于健康打卡需要首先通过“厦门大学统一身份认证系统”进行 OAuth 认证，因此需要向程序提供统一认证的学工号/密码，或提供已登录过 https://xmuxg.xmu.edu.cn 的名为 `SAAS_U` 的 Cookie.

**程序不会储存你输入的任何用户名、密码或 Cookie, 也不会将获得和输入的用户名、密码、Cookie 等信息发送到除 `*.xmu.edu.cn` 之外的其它网站。**

#### 方法 1. 使用统一身份认证学工号/密码打卡

```bash
python checkin.py [username] [password]
```

其中 `[username]` 为你的学工号，`[password]` 为你的统一认证密码。

#### 方法 2. 使用 Cookie 打卡

```bash
python checkin.py [cookie:SAAS_U]
```

获取SAAS_U 的方式如下图所示：

![get cookie](get_cookie.png)

#### 区别

使用用户名和密码打卡较方便，但有较小的可能性导致泄露。

使用 Cookie 打卡更安全，但是当主动登出账号，或学工系统服务器重启时可能会过期。

### 检查今日是否已打卡/获取最近打卡信息

```bash
python recent.py [username] [password]

# or
python recent.py [cookie:SAAS_U]
```

返回 `JSON` 类型数据。若返回数据中 `today` 为 `true` 表示今日已打卡；同时返回当前账号的姓名、最近几天打卡记录。

## 自动化打卡

> 提示：在凌晨期间 (0:00-7:00) 打卡可能会遇到服务器崩溃的问题，建议在多个时段执行打卡，或使用 `recent.py` 自行检查是否打卡成功。

### 使用 Linux 计划任务 (Crontab) 自动打卡

参考源码根目录下的 `auto-report.cron`，编写 Crontab 规则，如：

```
30 */24 * * * /usr/bin/python /path/to/checkin.py [username] [password]
```

其中，`30 */24 * * *` 表示定时任务的运行时间规则为每日的 0:30 执行程序打卡（不建议在每日 0:00 打卡，可能产生问题）；`/path/to/checkin.py` 表示 `checkin.py` 的完整路径，`[username] [password]` 则表示你的身份认证信息。

在 Linux 下使用以下命令激活定时任务：

```bash
crontab auto-report.cron
```

### 通过 API 打卡（仅支持 Cookie）

通过上文的方法获取名为 `SAAS_U` 的 Cookie，向以下地址发送 GET 请求即可自动打卡：

```
https://ami.kirainmoe.com:2333/XMUHealth/checkInByCookie?cookie=[cookie]
``` 

将 `[cookie]` 替换成你获得 Cookie 即可，该地址不会保存你的 Cookie 信息。

**你可以使用网站监控服务（监控宝、360 网站监控等）在每日 00:00-16:30 自动向地址发送 GET 请求来实现打卡。**


### 通过 Ami / ゆい 打卡

你可以在 `BanGDream@XMU` QQ 群组中，私聊 QQ 机器人 Ami（或在 `PCR@XMU` QQ 群组中私聊机器人ゆい），发送 `ami绑定打卡`（或 `ue绑定打卡`），Ami / UE 会告诉你具体的操作方式；每天凌晨 Ami / UE 会自动帮助所有绑定的人打卡。

![bot.png](https://i.loli.net/2020/05/21/ArDbsOucV8o9lCq.png)

![QQ20200521-110539@2x.png](https://i.loli.net/2020/05/21/LDwNJSBn75OaC1T.png)

## License

Do What The Fuck You Want To Public License (WTFPL)
