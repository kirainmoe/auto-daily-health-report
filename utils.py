# -*- coding: utf-8 -*-
import base64
import random

from Cryptodome.Cipher import AES
from Cryptodome.Util.Padding import pad


def get_wrapped_url(url, webvpn=False):
    if not webvpn:
        return url

    if "ids.xmu.edu.cn" in url:
        return url.replace("ids.xmu.edu.cn",
                           "webvpn.xmu.edu.cn/https/77726476706e69737468656265737421f9f352d23f3d7d1e7b0c9ce29b5b")

    if "xmuxg.xmu.edu.cn" in url:
        return url.replace("xmuxg.xmu.edu.cn",
                           "webvpn.xmu.edu.cn/https/77726476706e69737468656265737421e8fa5484207e705d6b468ca88d1b203b")


def encryptAES(data: str, salt: str):
    salt = salt.encode('utf-8')
    iv = randstr(16).encode('utf-8')
    cipher = AES.new(salt, AES.MODE_CBC, iv)
    data = randstr(64) + data
    data = data.encode('utf-8')
    data = pad(data, 16, 'pkcs7')
    cipher_text = cipher.encrypt(data)
    encoded64 = str(base64.encodebytes(cipher_text), encoding='utf-8').replace("\n", "")
    return encoded64


def randstr(num):
    H = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'

    salt = ''
    for i in range(num):
        salt += random.choice(H)

    return salt
