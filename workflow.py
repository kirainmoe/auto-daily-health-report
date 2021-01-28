import requests
import sys
import os

from checkin import health_report
from recent import check_recent

username = ""
password = ""


def report_with_server_chan(flag, reason="", success=""):
    try:
        server_chan_secret = os.environ["server_chan_secret"]
        push_url = "https://sc.ftqq.com/" + server_chan_secret + ".send"
        if flag:
            result_title = "打卡成功提醒"
            result_text = "今日打卡操作已成功！" + success
        else:
            result_title = "打卡失败提醒"
            result_text = "今日打卡操作没有成功，请手动完成打卡。错误细节：" + reason

        print(result_text)

        session = requests.Session()
        session.post(push_url, {
            "text": result_title,
            "desp": result_text
        })
    except KeyError:
        print("Cannot report with Server-Chan: secret_key not set")
        return
    except Exception:
        print("Cannot report with Server-Chan: unknown error")
        return


try:
    username = os.environ["xmu_username"]
    password = os.environ["xmu_password"]
except KeyError:
    reason = "You must provide a valid username & password to log in xmuxg.xmu.edu.cn！"
    print(reason)
    report_with_server_chan(False, reason)
    sys.exit(1)

try:
    today_log, status = check_recent(username, password)
    if status == 0 and today_log["today"]:
        print("Already reported today :)")
        sys.exit(0)

    response, status = health_report(username, password)
    if status != 0:
        print("Report error, reason: " + response["reason"])
        report_with_server_chan(False, response["reason"])
        sys.exit(1)

    today_log, status = check_recent(username, password)
    if status == 0:
        if today_log["today"]:
            print("Automatically reported successfully!")
            success_info = "当前连续打卡" + str(today_log["days"]) + "天，健康码为" + str(today_log["color"]) + "码！"
            report_with_server_chan(True, success=success_info)
            sys.exit(0)
        else:
            print("Automatically reported failed.")
            reason = "System rejected the health-report request."
            report_with_server_chan(False, reason)
            sys.exit(1)
    else:
        report_with_server_chan(False, "Internal server error")
        sys.exit(1)

except Exception as e:
    reason = "Error occurred while sending the report request."
    print(reason, e)
    report_with_server_chan(False, reason)
    sys.exit(1)
