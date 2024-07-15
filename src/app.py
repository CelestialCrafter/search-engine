import os
from importlib import import_module

from flask import Flask, Response, abort, request, send_from_directory

from .algorithms.entries import get_search_entries
from .algorithms.entries import transform as transform_entries
from .algorithms.fuzzy import get as fuzzy_get
from .common import parse_pb
from .options import load_options
from .preflight import preflight


def create_app():
	options = load_options()
	os.makedirs(options["data_path"], exist_ok=True)

	# move this to valkey
	query_cache = {}

	preflight()

	app = Flask(__name__)

	@app.route("/", defaults={"path": "index.html"})
	@app.route("/<path:path>")
	def frontend(path):
		return send_from_directory(options["web_path"], path)

	@app.route("/original/<path:url>")
	def original(url):
		entries = get_search_entries()
		if url not in entries:
			return abort(404)

		data = parse_pb(entries[url]["path"])
		return Response(data.original, mimetype=data.mime)

	@app.route("/api/search-results")
	def searchResults():
		page = request.args.get("page", default=1, type=int)
		query = request.args.get("query", type=str)
		algorithm_name = request.args.get("algorithm", type=str)

		try:
			algorithm = import_module(f".algorithms.{algorithm_name}", "src")
		except ModuleNotFoundError:
			return abort(400)

		if query is None:
			return abort(400)

		page_size = options["page_size"]
		pages = options["pages"]

		query_key = f"{algorithm_name}-{query}"
		if query_key not in query_cache:
			entries = algorithm.get(query, pages * page_size)
			query_cache[query_key] = entries
		else:
			entries = query_cache[query_key]

		offset = page_size * (page - 1)
		return {"total": pages, "results": transform_entries(entries[offset:offset + page_size])}

	return app
