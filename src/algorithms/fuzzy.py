import os
import pickle
from subprocess import PIPE, run

from tqdm import tqdm

from options import get_options

urls = []

def get(query, amount):
	global urls

	search_input = "\n".join(urls)
	fzy = run(["fzy", "-e", query, "-l", str(amount)], stdout=PIPE, input=search_input.encode())
	return fzy.stdout.decode().split("\n")

def load():
	global urls

	options = get_options()

	with open(os.path.join(options["data_path"], "fuzzy.pkl"), "rb") as f:
		urls = pickle.load(f)

	return

def save():
	global urls

	options = get_options()

	with open(os.path.join(options["data_path"], "fuzzy.pkl"), "wb") as f:
		pickle.dump(urls, f)

def compute(algorithm_data):
	global urls

	for _, data in tqdm(algorithm_data, desc="load fuzzy"):
		urls.append(data.url)
	return
