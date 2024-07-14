from base64 import b64decode
from threading import Thread

from flask import Flask, Response, abort, send_from_directory

from .common import parse_pb
from .options import load_options
from .preflight import get_search_entries, preflight


def create_app():
	options = load_options()
	preflight_thread = Thread(target=preflight)
	preflight_thread.start()

	app = Flask(__name__)

	@app.route('/', defaults={'path': 'index.html'})
	@app.route("/<path:path>")
	def frontend(path):
		return send_from_directory(options["web_path"], path)

	@app.route('/original/<path:url>')
	def original(url):
		entries = get_search_entries()
		if url not in entries:
			abort(404)

		entry = entries[url]
		data = parse_pb(entry["path"])
		return Response(data.original, mimetype=data.mime)

	@app.route('/api/search-results')
	def searchResults():
		return list(get_search_entries().values())

	return app
