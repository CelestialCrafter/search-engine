import tomllib

options_path = "options.toml"

options = {}
default = {
  "search_files_path": "search-files/",
  "data_path": "data/",
  "web_path": "web/dist/",
  "page_size": 100,
  "pages": 10,
}

def get_options():
	global options
	return options

def load_options():
	global options

	with open(options_path, "rb") as f:
		options = tomllib.load(f)

	for key in default.keys():
		if key in options:
			continue
		options[key] = default[key]

	return options
