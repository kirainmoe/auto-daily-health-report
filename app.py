import argparse
import json

from checkin import health_report
from recent import check_recent


parser = argparse.ArgumentParser()

parser.add_argument('username',
                    metavar='username',
                    help="统一身份认证用户名")
parser.add_argument('password',
                    metavar='password',
                    help="统一身份认证密码")
parser.add_argument('action',
                    metavar='action',
                    choices=["check", "query"],
                    help="动作 (check: 打卡，query: 查询今日打卡情况)")
parser.add_argument('--webvpn',
                    choices=["true", "false"],
                    help="是否通过 WebVPN 发送请求")
parser.add_argument('--vpn-username',
                    metavar='vpn_username',
                    help="WebVPN 用户名")
parser.add_argument('--vpn-password',
                    metavar='vpn_password',
                    help="WebVPN 密码")

args = parser.parse_args()

username = args.username
password = args.password
use_webvpn = args.webvpn == "true"


if args.action == "check":
    res, sta = health_report(username,
                            password,
                            use_webvpn=use_webvpn,
                            vpn_username=args.vpn_username,
                            vpn_password=args.vpn_password)
    print(json.dumps(res, indent=4, ensure_ascii=False))

if args.action == "query":
    res, sta = check_recent(username,
                           password,
                           use_webvpn=use_webvpn,
                           vpn_username=args.vpn_username,
                           vpn_password=args.vpn_password)
    print(json.dumps(res, indent=4, ensure_ascii=False))

