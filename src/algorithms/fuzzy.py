from subprocess import PIPE, run
from urllib.parse import unquote

from tqdm import tqdm

from .entries import get_search_entries

urls = []

def get(query, amount):
	global urls

	search_input = "\n".join(list(get_search_entries().keys()))
	fzy = run(["fzy", "-e", query, "-l", str(amount)], stdout=PIPE, input=search_input.encode())
	return fzy.stdout.decode().split("\n")

def load():
	return

def save():
	return

def compute(algorithm_data):
	global urls

	for _, data in tqdm(algorithm_data, desc="entries"):
		decodedUrl = unquote(data.url)
		urls.append(decodedUrl)
