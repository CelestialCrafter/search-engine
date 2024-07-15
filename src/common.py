from .protos import crawled_pb2 as crawled_pb2


def parse_pb(path):
	data = crawled_pb2.Crawled()
	with open(path, 'rb') as f:
		data.ParseFromString(f.read())
	return data
