import utils
from translator.openai_whisper import OpenaiWhisper
from translator import ModelScale
from translator.web import translate_by_baidu_api

from flask import Flask
from flask import request

app = Flask(__name__)

@app.route("/")
def homepage():
    return \
"""
<div align=\"center\">
    <h1>Hello, MKS(mksubrip) Server!</h1>
    <p>GitHub Repo: <a href="https://github.com/mistgc/mksubrip/tree/server">https://github.com/mistgc/mksubrip/tree/server</a></p>
</div>
"""

@app.route("/model/<string:model_name>/<string:scale>", methods=['POST', 'GET'])
def call_model(model_name, scale):
    file_storage = request.files["data"]
    scale = ModelScale.from_str(scale)
    model = OpenaiWhisper(scale)
    file_path = utils.handle_upload_file(file_storage)

    model.init()
    result = model.translate(file_path)
    for segement in result:
        segement["text"] = translate_by_baidu_api(segement["text"], from_lang="jp", to_lang="zh")
    result = OpenaiWhisper.convert_to_json(result)

    return result
