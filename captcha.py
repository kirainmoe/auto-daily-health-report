# -*- coding: utf-8 -*-
import base64
import json
import numpy
import re
import requests
import random

from io import BytesIO
from PIL import Image
from urllib import parse

session = requests.Session()

def bypass_captcha(session, webvpn_username, webvpn_password):
    # 1. Request `https://xmuxg.xmu.edu.cn`
    #    Redirect to `https://ids-vpn.xmu.edu.cn/wengine-auth/login`
    session.get("https://xmuxg.xmu.edu.cn/", allow_redirects=True)

    login_data = {
        "auth_type": "local",
        "sms_code": "",
        "username": webvpn_username,
        "password": webvpn_password
    }

    http_header = {
        'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) '
                    'Chrome/81.0.4044.138 Safari/537.36',
        'Referer': 'https://ids-vpn.xmu.edu.cn/wengine-auth/login?id=581&path=/&from=https://xmuxg.xmu.edu.cn/'
    }

    # 2. POST  https://ids-vpn.xmu.edu.cn/wengine-auth/do-login
    #    If returns "CAPTCHA_FAILED", then captcha is required
    resp = session.post("https://ids-vpn.xmu.edu.cn/wengine-auth/do-login", login_data,
                        headers=http_header)
    resp_json = json.loads(resp.text)
    if resp_json["success"] == True:
        resp = session.get(resp_json["url"], allow_redirects=True)
        return session

    # 3. Fetch captcha image from "https://ids-vpn.xmu.edu.cn/wengine-auth/login/image"
    resp = session.get("https://ids-vpn.xmu.edu.cn/wengine-auth/login/image")
    resp_json = json.loads(resp.text)

    resp = session.get("https://ids-vpn.xmu.edu.cn/wengine-auth/login/image")
    resp_json = json.loads(resp.text)

    offset_height = resp_json["h"]
    background_base64 = re.sub('^data:image/.+;base64,', '', resp_json["p"])

    # 4. Use PIL to detect fragment position
    #    This captcha is a PNG image! lol
    background = Image.open(BytesIO(base64.b64decode(background_base64)))

    width, height = background.size
    pixels = numpy.asarray(background)
    marks = numpy.zeros((height, width))
    leftmost = width

    for i in range(0, height):
        for j in range(0, width):
            if pixels[i][j][3] < 255:
                leftmost = min(leftmost, j)

    leftmost -= 4

    # 5. Calculate pointer move distance, and try to verify
    x_base = 722
    y_base = 498
    x_end = x_base + leftmost
    y_end = y_base - 5

    post_data = "w=%d&t=0&locations%%5B0%%5D%%5Bx%%5D=%d&locations%%5B0%%5D%%5By%%5D=%d&locations%%5B1%%5D%%5Bx%%5D=%d&locations%%5B1%%5D%%5By%%5D=%d" % (leftmost, x_base, y_base, x_end, y_end)

    verify_header = {
        'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) '
                    'Chrome/81.0.4044.138 Safari/537.36',
        'Referer': 'https://ids-vpn.xmu.edu.cn/wengine-auth/login?id=581&path=/&from=https://xmuxg.xmu.edu.cn/',
        "Content-Type":"application/x-www-form-urlencoded"
    }

    resp = session.post("https://ids-vpn.xmu.edu.cn/wengine-auth/login/verify", post_data, headers=verify_header)
    resp_json = json.loads(resp.text)["success"]

    # 6. If verify OK, retry to login
    if resp_json == True:
        resp = session.post("https://ids-vpn.xmu.edu.cn/wengine-auth/do-login", login_data,
                        headers=http_header)
        resp_json = json.loads(resp.text)
        resp = session.get(resp_json["url"], allow_redirects=True)
        return session
        