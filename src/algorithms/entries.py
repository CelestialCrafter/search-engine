import os
import pickle

from tqdm import tqdm

from options import get_options
from protos import crawled_pb2 as crawled_pb2

search_entries = {}

def get_search_entries():
	global search_entries
	return search_entries

def transform(urls):
	filtered = filter(lambda url: url in search_entries, urls)
	mapped = map(lambda url: str(search_entries[url]), filtered)
	return list(mapped)

def load():
	global search_entries

	options = get_options()

	with open(os.path.join(options["data_path"], "entries.pkl"), "rb") as f:
		search_entries = pickle.load(f)

def save():
	global search_entries

	options = get_options()

	with open(os.path.join(options["data_path"], "entries.pkl"), "wb") as f:
		pickle.dump(search_entries, f)

def compute(algorithm_data):
	global search_entries

	for _, data in tqdm(algorithm_data, desc="load entries"):
		data.ClearField("children")
		data.ClearField("text")
		data.ClearField("original")

		search_entries[data.url] = data.SerializeToString()
