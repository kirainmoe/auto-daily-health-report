# Auto Daily Health Report

这是一个模拟自动登录厦门大学学工系统，并自动完成厦门大学每日健康打卡 (Daily Health Report) (https://xmuxg.xmu.edu.cn/app/214) 的 Python 程序。

截至 2020 年 7 月，这段程序已经稳定帮作者本人打卡了两个月。若打卡逻辑没有特别变动，本项目将不再更新。

![image.png](https://i.loli.net/2020/07/18/9kspbW6LIUjcQ1t.png)

## 免责声明

此项目仅供学习/交流/健忘症玩家使用；若产生任何后果（包括但不限于被学院或辅导员<s>橄榄</s>、因学工服务器接口修改等原因中断打卡等）请自负。

## 下载 & 安装所需库

此程序基于 Python 3，需要 `requests` 和 `BeautiuflSoup, lxml` 库支持以发送网络请求、解析网页。程序在 Python 3.6, macOS 10.15 环境下测试。

首先使用 `git clone` 或从右上角的 `Download Zip` 获取程序源代码，然后执行 `pip install`：

```bash
git clone https://github.com/kirainmoe/auto-daily-health-report

pip install -r requirements.txt -i https://pypi.douban.com/simple
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

> 提醒：由于学工系统日常抽风会导致会话失效，因此 Cookie 可能很快就会失效，建议使用用户名和密码打卡。

### 检查今日是否已打卡/获取最近打卡信息

```bash
python recent.py [username] [password]

# or
python recent.py [cookie:SAAS_U]
```

返回 `JSON` 类型数据。若返回数据中 `today` 为 `true` 表示今日已打卡；同时返回当前账号的姓名、最近几天打卡记录。

## 打卡自动化

### 使用持续集成环境 (CI) 或 GitHub Actions 自动打卡 （推荐）

以使用 Travis CI 为例，首先前往 travis-ci.org 注册一个账号，并在 GitHub 创建一个名称任意的项目仓库，如 `ci-health-report`.

然后在 `ci-health-report` 项目仓库中，创建 `.travis.yml` 文件，写入如下内容：

```yaml
sudo: required
os: linux
language: python
python:
- 3.8
install:
- git clone https://github.com/kirainmoe/auto-daily-health-report healthreport
- cd healthreport
- pip install -r requirements.txt
script:
- python checkin.py ${xmu_username} ${xmu_password}
```

转到 travis-ci.org ，使用 GitHub 账号登陆，然后启用 `ci-health-report` 项目的持续集成开关，并在项目设置中设置以下环境变量：

| NAME | VALUE |
|------|-------|
| xmu_username | 你的学工号 |
| xmu_password | 你的统一认证密码 |

以上环境变量在任何地方对公众都是不可见的。完成后同时在项目设置中添加 Cron Jobs，分支填写 `master`，运行时刻选 `daily` 和 `Do not run... 24h`。

![image.png](https://i.loli.net/2020/07/18/7uPExRmdbAULWHs.png)

然后**在打卡时间内手动触发一次集成构建**即可。

### 通过 Ami / ゆい 打卡(推荐)

你可以私聊 QQ 机器人 AmiBOT：“`ami绑定打卡`”，或私聊机器人ゆいBOT “`ue绑定打卡`”，Ami / ue 会告诉你绑定账号的具体操作方式。每天 7:05 开始， Ami（或 ゆい） 会自动帮助所有绑定的人打卡。截止 2020 年 7 月，已有超过 20 名用户使用 Ami 或 ゆい 稳定打卡。

目前 Ami / ゆい 在以下群组中开放：

- BanGDream!@XMU
- PCR@XMU

![bot.png](https://i.loli.net/2020/05/21/ArDbsOucV8o9lCq.png)

![QQ20200521-110539@2x.png](https://i.loli.net/2020/05/21/LDwNJSBn75OaC1T.png)


### 使用 Linux 计划任务 (Crontab) 自动打卡

参考源码根目录下的 `auto-report.cron`，编写 Crontab 规则，如：

```
30 */24 * * * /usr/bin/python /path/to/checkin.py [username] [password]
```

其中，`30 */24 * * *` 表示定时任务的运行时间规则为每日的 0:30 执行程序打卡；`/path/to/checkin.py` 表示 `checkin.py` 的完整路径，`[username] [password]` 则表示你的身份认证信息。

> PS: 从 5.27 起全校打卡的时间已经变更，现在只有 7:00-19:30 可以执行打卡，你需要自行修改上述 Cron 计划任务的规则。

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

**你可以使用网站监控服务（监控宝、360 网站监控等）在每日 07:00-19:00 自动向地址发送 GET 请求来实现打卡。**

## 返回值

### checkin.py

签到成功与否都在 `stdout` 输出一串 JSON，包含字段 `status` （值为 `success` 或 `failed`）表示是否成功打卡。

若为 `failed`，则在 `reason` 字段说明原因（登录失败、鉴权信息错误、学工服务器内部错误等）。程序返回值为 `1`.

若为 `success`，则在 `name` 字段描述打卡对象姓名，程序返回值为 `0`.

### recent.py

获取信息成功与否都在 `stdout` 输出一串 JSON，包含字段 `status` （值为 `success` 或 `failed`） 表示是否成功获取信息。

若为 `failed`，则在 `reason` 字段说明无法获取信息的原因；若为 `success`，则有 `owner` 字段显示打卡对象姓名，`today` 字段表示今日是否打卡，`recent` 字段表示最近的打卡数据。

## License

Do What The Fuck You Want To Public License (WTFPL)
