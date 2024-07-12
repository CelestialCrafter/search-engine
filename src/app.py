import os
from urllib.parse import unquote

from flask import Flask, Response, abort, render_template
from werkzeug.security import safe_join

import protos.crawled_pb2 as crawled_pb2

dataPath = '../data/'
app = Flask(__name__)

def parsePb(path):
	data = crawled_pb2.Crawled()
	with open(path, 'rb') as f:
		data.ParseFromString(f.read())
	return data

@app.route('/')
def index():
	results = []

	for root, _, files in os.walk(dataPath):
		for file in files:
			data = parsePb(os.path.join(root, file))
			results.append({
			  'url': unquote(data.url),
			  'crawledAt': data.crawledAt.ToMilliseconds(),
			  'mime': data.mime,
			})

	return render_template('index.html', search_results=results)

@app.route('/original/<host>/<path>')
def original(host, path):
	path = safe_join(dataPath, os.path.join(host, path + '.pb'))

	if not os.path.isfile(path):
		abort(404)

	data = parsePb(path)
	return Response(data.original, mimetype=data.mime)
