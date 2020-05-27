import sys
import flask
from flask import Flask, render_template


app = Flask(__name__)


@app.route('/')
def index():
    return render_template("index.html")

@app.route('/client')
def client():
    return render_template("client.html", operatingClientSystem=sys.platform, operatingServerSystem="Ubuntu 18.04 LTS")

app.run(debug=True)