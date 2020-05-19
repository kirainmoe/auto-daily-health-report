# Auto Daily Health Report

这是一个模拟自动登录厦门大学学工系统，并自动完成厦门大学每日健康打卡 (Daily Health Report) (https://xmuxg.xmu.edu.cn/app/214) 的 Python 程序。

仅供学习/交流/健忘症使用；若产生任何后果请自负。

## 下载 & 安装所需库

此程序基于 Python 3，需要 `requests` 和 `BeautiuflSoup, lxml` 库支持以发送网络请求、解析网页。程序在 Python 3.6, macOS 10.15 环境下测试。

首先使用 `git clone` 或从右上角的 `Download Zip` 获取程序源代码，然后执行 `pip install`：

```bash
git clone https://github.com/kirainmoe/auto-daily-health-report

pip install
```

## 使用

由于健康打卡需要首先通过“厦门大学统一身份认证系统”进行 OAuth 认证，因此需要向程序提供统一认证的学工号/密码，或提供已登录过 https://xmuxg.xmu.edu.cn 的名为 `SAAS_U` 的 Cookie.

**程序不会储存你输入的任何用户名、密码或 Cookie, 也不会将获得和输入的用户名、密码、Cookie 等信息发送到除 `*.xmu.edu.cn` 之外的其它网站。**

### 使用厦大统一身份认证学工号/密码打卡

```bash
python checkin.py [username] [password]
```

其中 `[username]` 为你的学工号，`[password]` 为你的统一认证密码。

### 使用 Cookie 打卡

```bash
python checkin.py [cookie:SAAS_U]
```

获取SAAS_U 的方式如下图所示：

![get cookie](get_cookie.png)

### 区别

使用用户名和密码打卡较方便，但有较小的可能性导致泄露。

使用 Cookie 打卡更安全，但是可能会过期。

## 自动化打卡

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

## Licese

Do What The Fuck You Want To Public License (WTFPL)
