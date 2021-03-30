import requests
import json
import sys

from bs4 import BeautifulSoup


# request with webvpn.xmu.edu.cn
def with_webvpn(session, header, vpn_username, vpn_password):
    try: 
        login_page = session.get("https://webvpn.xmu.edu.cn/login",
                                headers=header).text
        soup = BeautifulSoup(login_page, 'lxml')

        need_captcha = soup.select('input[name="needCaptcha"]')[0]['value']
        if need_captcha == 'true':
            print(json.dumps({
                "status": "failed",
                "reason": "WebVPN Login failed (captcha required)"
            }, indent=4))
            sys.exit(1)

        captcha_id = soup.select('input[name="captcha_id"]')[0]['value']
        
        vpn_login_url = "https://webvpn.xmu.edu.cn/do-login"
        login_data = {
            "auth_type": "local",
            "username": vpn_username,
            "password": vpn_password,
            "sms_code":"",
            "captcha": "",
            "needCaptcha": False,
            "captcha_id": captcha_id
        }

        session.post(vpn_login_url,
                    login_data,
                    headers=header,
                    allow_redirects=True)

        return session
    except KeyError:
        print(json.dumps({
            "status": "failed",
            "reason": "WebVPN Login failed (server error)"
        }, indent=4))
        sys.exit(1)