import requests
import json
import sys

from login import login
from webvpn import with_webvpn
from utils import get_wrapped_url


http_header = {
    'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) '
                  'Chrome/81.0.4044.138 Safari/537.36',
    'Referer': 'https://xmuxg.xmu.edu.cn/xmu/login?app=214'
}


def health_report(username,
                  password,
                  use_webvpn=False,
                  vpn_username=None,
                  vpn_password=None):
    # create session
    session = requests.Session()

    if use_webvpn:
        session = with_webvpn(session, http_header, vpn_username, vpn_password)


    # login
    login(session, username, password, http_header, use_webvpn=use_webvpn)

    # get form id
    now_url = get_wrapped_url("https://xmuxg.xmu.edu.cn/api/app/214/business/now", use_webvpn)
    resp = None
    form_id = None
    try:
        resp = session.get(now_url).text
        form_id = str(json.loads(resp)['data'][0]['business']['id'])
    except Exception:
        return ({
            "status": "failed",
            "reason": "Login failed (incorrect auth info or captcha required)"
        }, 1)

    # get form instance
    form_url = get_wrapped_url("https://xmuxg.xmu.edu.cn/api/formEngine/business/%s/formRenderData?playerId=owner" % form_id, use_webvpn)
    form_components = None
    try:
        resp = session.get(form_url, headers=http_header).text
        form_components = json.loads(resp)["data"]["components"]
    except Exception:
        return ({
            "status": "failed",
            "reason": "Internal server error (logged in but cannot get form id)"
        }, 1)

    # get owner modification
    form_instance_url = get_wrapped_url("https://xmuxg.xmu.edu.cn/api/formEngine/business/%s/myFormInstance" % form_id, use_webvpn)
    resp = session.get(form_instance_url, headers=http_header).text
    form_json = json.loads(resp)["data"]
    instance_id = form_json["id"]

    # change form content
    value_list = {}
    for (k, v) in enumerate(form_json["formData"]):
        name = v['name']
        hide = v['hide']
        title = v['title']
        value = {}

        if "学生本人是否填写" in title:
            value['stringValue'] = '是'
        elif "Can you hereby declare that" in title:
            value['stringValue'] = '是 Yes'
        elif v['value']['dataType'] == 'STRING':
            value['stringValue'] = v['value']['stringValue']
        elif v['value']['dataType'] == 'ADDRESS_VALUE':
            value['addressValue'] = v['value']['addressValue']

        value_list[name] = {
            'hide': hide,
            'title': title,
            'value': value
        }

    # prepare post data
    post_array = []
    for item in form_components:
        name = item['name']
        if name in value_list:
            hide = True if value_list[name]['hide'] else False
            if 'select' in name and 'stringValue' in value_list[name]['value'] and value_list[name]['value']['stringValue'] == "":
                hide = True
            post_array.append({
                'name': name,
                'title': value_list[name]['title'],
                'value': value_list[name]['value'],
                'hide': hide
            })
        else:
            post_array.append({
                'name': name,
                'title': item['title'],
                'value': {},
                'hide': True if 'label' not in name else False,
            })


    # post change
    post_modify_url = get_wrapped_url("https://xmuxg.xmu.edu.cn/api/formEngine/formInstance/" + instance_id, use_webvpn)
    post_json = {
        "formData": post_array,
        "playerId": "owner"
    }
    post_json_str = json.dumps(post_json, ensure_ascii=False)
    http_header['Content-Type'] = 'application/json'
    http_header['Referer'] = 'https://xmuxg.xmu.edu.cn/app/214'
    resp = session.post(post_modify_url, headers=http_header, data=post_json_str.encode('utf-8'))

    return ({
        "status": "success",
        "info": "automatically checked in successfully.",
        "name": form_json["owner"]["name"]
    }, 0)


if __name__ == "__main__":
    username = ""
    password = ""
    if len(sys.argv) == 3:
        username = sys.argv[1]
        password = sys.argv[2]
    else:
        print("Usage: python checkin.py [username] [password]")
        sys.exit(1)
    
    response, status = health_report(username, password)
    print(json.dumps(response, indent=4, ensure_ascii=False))
    sys.exit(status)

