import os
import random
import hashlib
import requests
import json

appid = os.environ["BAIDU_APPID"]
secret_key = os.environ["BAIDU_SECRET_KEY"]

if len(appid) == 0 or len(secret_key) == 0:
    print("appid or secret_key is empty.")
    exit(1)

def _send_request_to_baidu(text: str, from_lang="jp", to_lang="zh") -> str:
    url = _format_url(text, from_lang, to_lang)
    result = requests.get(url, timeout=4)
    result_json = json.loads(result.text)

    return result_json["trans_result"][0]["dst"]

def _format_url(text: str, from_lang: str, to_lang: str) -> str:
    salt = random.randint(32768, 65535)
    sign_text = "{}".format(str(appid)+text+str(salt)+str(secret_key))
    sign = hashlib.md5(sign_text.encode("utf-8")).hexdigest()
    url = f"https://fanyi-api.baidu.com/api/trans/vip/translate?q={text}&from={from_lang}&to={to_lang}&appid={appid}&salt={salt}&sign={sign}"

    return url

def translate_by_baidu_api(text: str, from_lang="jp", to_lang="zh") -> str:
    return _send_request_to_baidu(text, from_lang, to_lang)
