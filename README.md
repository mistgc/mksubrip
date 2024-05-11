<div align="center">
    <h1>mksubrip server</h1>
</div>

## Install
```bash
git clone -b server https://github.com/mistgc/mksubrip.git mksubrip-server
cd mksubrip-server
python -m venv .venv
source .venv/bin/activate
python -m pip -r requirements.txt
```

## Usage

```bash
BAIDU_APPID="<your baidu's appid>" BAIDU_SECRET_KEY="<your baidu's secret key>" ./run.sh
```
