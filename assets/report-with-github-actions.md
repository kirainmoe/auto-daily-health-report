# 使用 GitHub Actions 实现每日自动打卡

## 准备

为了使用 GitHub Actions 实现每日自动打卡，你需要：

- 基本的文字阅读理解能力（至少能看懂这篇教程）

- 一个 GitHub 账号（必须）
  - 如果你还没有 GitHub 账号，请 [点击此链接](https://github.com/join?source=login) 注册。

- 使用 GitHub 账号登录 Server  酱并绑定微信通知（可选）
  - 绑定微信后会将每天的打卡结果通过公众号“方糖”推送到你的微信上，这一步是可选的。


## 部署步骤

### Step 1. Fork 本项目到你的账号上

在 [auto-daily-health-report](https://github.com/kirainmoe/auto-daily-health-report) 的项目主页，点击右上角的 "Fork" 键，将本项目复制一份到你的账号上。

![image.png](https://i.loli.net/2020/09/11/ZVCk6IDLOv3eSXG.png)

点击 Fork 之后，GitHub 会自动跳转到你复制的项目的页面。

### Step 2. 设置环境变量，填写账号等信息

在你 Fork 的项目页面中，依次点击 "Settings => Secret"，跳转到设置环境变量的页面，如下图所示：

![image.png](https://i.loli.net/2020/09/11/aLQZ75iVsu3EIF9.png)

点击 "New Secret"，创建两个环境变量 `xmu_username` ，`xmu_password`，值分别设置为你的厦门大学学工号、统一身份认证密码，如下图所示。

![image.png](https://i.loli.net/2020/09/11/smhU6nZXy2IWbGO.png)

![image.png](https://i.loli.net/2020/09/11/TEQ8hD9GHxzbdN1.png)

请确保你填写的账号是正确的，否则将无法完成打卡。打卡过程中这个账号和密码对公众是不可见的，并且不会发送到除了 `*.xmu.edu.cn` 之外的其它网站。

### Step 3. 启用 GitHub Actions

作者在项目的 `.github/workflows` 目录下已经设置好了程序的执行规则 [python.yml](https://github.com/kirainmoe/auto-daily-health-report/blob/master/.github/workflows/python.yml)，默认设置是每天 8:05 分进行打卡。你唯一需要做的就是触发这个 GitHub Action。

在你 Fork 的项目页面中，点击 "Actions"，跳转到设置 GitHub Actions 的页面，找到左边的 "XMU daily health report"，此时右边会提示 `This workflow has a workflow_dispatch event trigger.`，我们点击 "Run workflow"，然后确定之后手动触发一次 GitHub Actions，如下图所示：

![image.png](https://i.loli.net/2020/09/11/eEBshC5WoLtdP8w.png)

点击之后，Github Actions 会自动测试打卡操作。此时可以点击任务详情，查看是否成功，来检查你的环境变量是否设置正确，如下图所示：

![image.png](https://i.loli.net/2020/09/11/mjE2zUiPK8CT4Lu.png)

![image.png](https://i.loli.net/2020/09/11/hXLs3NuwPnqxCQ1.png)

如果看到了绿色的勾就证明设置成功，接下来只要交给 GitHub 让它每天自动帮我们解决打卡就行了。

### Step 4. 联动 Server 酱获取打卡提醒（可选）

> 「Server酱」，英文名「ServerChan」，是一款「程序员」和「服务器」之间的通信软件。说人话？就是从服务器推报警和日志到手机的工具。

好吧，Server 酱是什么并不重要，你只需要知道绑定 Server 酱之后它可以告诉你每天打卡的结果就可以了。虽然这一步并不是必须的，但是还是推荐所有人绑定。

打开 [Server 酱官网](http://sc.ftqq.com/)，点击右上角的登录，按照提示使用 GitHub 账号登录完成 OAuth。

![image.png](https://i.loli.net/2020/09/11/pagDeLnJoN1hsTi.png)

登录成功，首先点击上面的“微信推送”，关注“方糖”公众号并绑定你的微信账号；然后点击上方的“发送信息”，可以得到一串字母和数字组合的 **SCKEY**，如图所示：

![image.png](https://i.loli.net/2020/09/11/87OUImxGoMHQeAf.png)

复制 SCKEY 的内容，参照 Step 2 在 GitHub 中添加名为 `server_chan_secret` 的环境变量，如图所示：

![image.png](https://i.loli.net/2020/09/11/ykWcrNe8RqSpzb2.png)

这样就大功告成了。以后每当这个打卡的 Action 运行时，都会通过微信告诉你结果：

![image.png](https://i.loli.net/2020/09/11/yVRK2tzfG943j1b.png)

## EOF
