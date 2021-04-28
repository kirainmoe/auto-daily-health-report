import requests
import json
import sys
import os
import execjs

from bs4 import BeautifulSoup
from utils import get_wrapped_url


# Login session by username & password
# cookie login is deprecated because it is likely to be expired in 1-2 days
def login(session, username, password, http_header, use_webvpn=False):
    """
    use cookie: just requires "SAAS_U"
    emulate OAuth login:
        POST https://ids.xmu.edu.cn/authserver/login?service=https://xmuxg.xmu.edu.cn/login/cas/xmu
        form data: username, password, lt, dllt, execution, _eventId="submit", rmShown=1
    """

    # workaround for the AES encryption added in 2020/12/27
    with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "encrypt.js"), "r") as file:
        cryptjs = file.read()
    ctx = execjs.compile(cryptjs)

    try:
        oauth_login_url = get_wrapped_url("https://ids.xmu.edu.cn/authserver/login?service=https://xmuxg.xmu.edu.cn/login/cas/xmu", use_webvpn)
        resp = session.get(oauth_login_url, headers=http_header)

        soup = BeautifulSoup(resp.text, 'lxml')
        lt = soup.select('input[name="lt"]')[0]["value"]
        dllt = soup.select('input[name="dllt"]')[0]['value']
        execution = soup.select('input[name="execution"]')[0]['value']
        salt = soup.select('input#pwdDefaultEncryptSalt')[0]['value']

        login_data = {
            "username": username,
            "password": ctx.call("encryptAES", password, salt),
            "lt": lt,
            "dllt": dllt,
            "execution": execution,
            "_eventId": "submit",
            "rmShown": 1
        }
        session.post(oauth_login_url, login_data,
                     headers=http_header,
                     allow_redirects=True)  # will redirect to https://xmuxg.xmu.edu.cn

    except KeyError:
        print(json.dumps({
            "status": "failed",
            "reason": "Login failed (server error)"
        }, indent=4))
        sys.exit(1)
