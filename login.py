import requests

from bs4 import BeautifulSoup


# Login session by cookie / username&password
def login(session, username, password, cookie, use_cookie, http_header):
    """
    use cookie: just requires "SAAS_U"
    emulate OAuth login:
        POST https://ids.xmu.edu.cn/authserver/login?service=https://xmuxg.xmu.edu.cn/login/cas
        form data: username, password, lt, dllt, execution, _eventId="submit", rmShown=1
    """

    if use_cookie:
        requests.utils.add_dict_to_cookiejar(session.cookies, {
            "SAAS_U": cookie
        })
    else:
        oauth_login_url = "https://ids.xmu.edu.cn/authserver/login?service=https://xmuxg.xmu.edu.cn/login/cas/xmu"
        resp = session.get(oauth_login_url, headers=http_header)
        soup = BeautifulSoup(resp.text, 'lxml')
        lt = soup.select('input[name="lt"]')[0]["value"]
        dllt = soup.select('input[name="dllt"]')[0]['value']
        execution = soup.select('input[name="execution"]')[0]['value']

        login_data = {
            "username": username,
            "password": password,
            "lt": lt,
            "dllt": dllt,
            "execution": execution,
            "_eventId": "submit",
            "rmShown": 1
        }
        resp = session.post(oauth_login_url, login_data,
                            headers=http_header,
                            allow_redirects=True)       # will redirect to https://xmuxg.xmu.edu.cn
