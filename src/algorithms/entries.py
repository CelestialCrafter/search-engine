import os
import pickle
from urllib.parse import unquote

from ..options import get_options

search_entries = {}

def get_search_entries():
	global search_entries
	return search_entries

def transform(urls):
	filtered = filter(lambda url: url in search_entries, urls)
	mapped = map(lambda url: search_entries[url], filtered)
	return list(mapped)

def load():
	global search_entries

	options = get_options()

	with open(os.path.join(options["data_path"], "entries-algorithm.pkl"), "rb") as f:
		search_entries = pickle.load(f)

def save():
	global search_entries

	options = get_options()

	with open(os.path.join(options["data_path"], "entries-algorithm.pkl"), "wb") as f:
		pickle.dump(search_entries, f)

def compute(path, data):
	global search_entries

	decodedUrl = unquote(data.url)
	search_entries[decodedUrl] = {
	  'path': path,
	  'url': decodedUrl,
	  'crawledAt': data.crawledAt.ToMilliseconds(),
	  'mime': data.mime,
	}
