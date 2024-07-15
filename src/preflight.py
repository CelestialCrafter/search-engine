import os
from hashlib import sha1
from importlib import import_module

from tqdm import tqdm

from .common import parse_pb
from .options import get_options


def preflight():
	options = get_options()

	files = []
	sum = ""

	for root, _, walk_files in os.walk(options["search_files_path"]):
		for file in walk_files:
			path = os.path.join(root, file)
			files.append(path)
			sum += sha1(path.encode()).hexdigest()

	compute = True
	sum = sha1(sum.encode()).hexdigest()

	path = os.path.join(options["data_path"], "search-files-hash")

	f = open(path, "r+")
	f.seek(0)

	if sum == f.read():
		compute = False
		f.close()

	# @NOTE make sure entries is the 0th element
	algorithms = ["entries", "fuzzy", "bm25"]
	algorithm_data = []

	if compute:
		for path in tqdm(files, desc="parse-pb"):
			algorithm_data.append((path, parse_pb(path)))

	for algorithm_name in algorithms:
		algorithm = import_module(f".algorithms.{algorithm_name}", "src")

		if compute:
			algorithm.compute(algorithm_data)
			algorithm.save()
		else:
			algorithm.load()

	if compute:
		f.seek(0)
		f.write(sum)
		f.truncate()
		f.close()
