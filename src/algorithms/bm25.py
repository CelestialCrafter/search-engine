import os

import bm25s
import Stemmer

from ..options import get_options
from .entries import get_search_entries

retriever = bm25s.BM25()
corpus = []

def get(query, amount):
	global stemmer
	global corpus
	global retriever

	options = get_options()
	resultUrls = []
	tokens = bm25s.tokenize(query)
	results, scores = retriever.retrieve(tokens, k=amount)
	urls = list(get_search_entries().values())

	for i in range(results.shape[1]):
		doc, score = results[0, i], scores[0, i]
		if score < options["bm25_threshold"]:
			continue
		resultUrls.append(urls[doc]["url"])

	return resultUrls

def load():
	global retriever
	global urls
	global stemmer

	options = get_options()
	stemmer = Stemmer.Stemmer(options["bm25_stem_lang"])
	retriever = bm25s.BM25.load(os.path.join(options["data_path"], "bm25"))

def save():
	global corpus
	global retriever

	options = get_options()
	retriever.save(os.path.join(options["data_path"], "bm25"), corpus=corpus)

def compute(algorithm_data):
	global corpus
	global retriever
	global stemmer

	options = get_options()
	stemmer = Stemmer.Stemmer(options["bm25_stem_lang"])

	for _, data in algorithm_data:
		text = data.text.decode(errors="ignore")
		corpus.append(text)

	tokens = bm25s.tokenize(corpus)
	retriever.index(tokens)
