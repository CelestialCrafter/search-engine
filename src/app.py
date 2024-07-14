from threading import Thread

from flask import Flask, Response, abort, request, send_from_directory

from .common import parse_pb
from .options import load_options
from .preflight import get_search_entries, preflight


def create_app():
	options = load_options()
	preflight_thread = Thread(target=preflight)
	preflight_thread.start()

	app = Flask(__name__)

	@app.route("/", defaults={"path": "index.html"})
	@app.route("/<path:path>")
	def frontend(path):
		return send_from_directory(options["web_path"], path)

	@app.route("/original/<path:url>")
	def original(url):
		entries = get_search_entries()
		if url not in entries:
			abort(404)

		entry = entries[url]
		data = parse_pb(entry["path"])
		return Response(data.original, mimetype=data.mime)

	@app.route("/api/search-results")
	def searchResults():
		offset = request.args.get("offset", default=0, type=int)
		limit = min(500, request.args.get("limit", default=100, type=int))
		entries = list(get_search_entries().values())

		return {"total": len(entries), "results": entries[offset:offset + limit]}

	return app
