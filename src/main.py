import utils
from translator.openai_whisper import OpenaiWhisper
from translator import ModelScale
from translator.web import translate_by_baidu_api

from flask import Flask
from flask import request

app = Flask(__name__)

@app.route("/")
def homepage():
    return "<p>Hello, MKS(mksubrip) Server!</p>"

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
