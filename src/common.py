from protos import crawled_pb2 as pb


def parse_pb(path):
	doc = pb.Document()
	with open(path, "rb") as f:
		doc.ParseFromString(f.read())
	return doc
