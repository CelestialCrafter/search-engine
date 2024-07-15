from subprocess import PIPE, run

from .entries import get_search_entries


def get(query, amount):
	search_input = "\n".join(list(get_search_entries().keys()))
	fzy = run(["fzy", "-e", query, "-l", str(amount)], stdout=PIPE, input=search_input.encode())
	return fzy.stdout.decode().split("\n")

def load():
	return

def save():
	return

def compute(path, data):
	return
