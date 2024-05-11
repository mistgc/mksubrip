#!/bin/bash
source .venv/bin/activate

exec python -m flask --app src/main.py run
