import glob
import os
import json

SNIPPETS_DIR = "./snippets"

FILES = glob.glob(os.path.join(SNIPPETS_DIR, "*"))
print(f"files: {FILES}")
snippets_dict = dict()

SCOPE = 'rust'

for filepath in FILES:
    print(filepath)
    filename = filepath.split('/')[-1]
    print(filename)

    prefix = filename.split('.')[0]
    with open(filepath, 'r') as snippet:
        body = snippet.read()

    snippets_dict[filename] = dict()
    snippets_dict[filename]['prefix'] = prefix
    snippets_dict[filename]['scope'] = SCOPE
    snippets_dict[filename]['body'] = body
    # print(snippets_dict)


"""
snippet example

"Print to console": {
	"scope": "javascript,typescript",
	"prefix": "log",
	"body": [
		"console.log('$1');",
		"$2"
	],
	"description": "Log output to console"
}
"""

with open('./.vscode/snippet.code-snippets', 'w') as snippet_file:
    json_str = json.dumps(snippets_dict)
    snippet_file.write(json_str)
