def get_wrapped_url(url, webvpn=False):
    if not webvpn:
        return url

    if "ids.xmu.edu.cn" in url:
        return url.replace("ids.xmu.edu.cn", "webvpn.xmu.edu.cn/https/77726476706e69737468656265737421f9f352d23f3d7d1e7b0c9ce29b5b")
    
    if "xmuxg.xmu.edu.cn" in url:
        return url.replace("xmuxg.xmu.edu.cn", "webvpn.xmu.edu.cn/https/77726476706e69737468656265737421e8fa5484207e705d6b468ca88d1b203b")