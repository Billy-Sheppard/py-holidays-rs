import os
import json
import gzip

for root, _, files in os.walk('.'):
    for file in files:
        if file.endswith('.json'):
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                data = json.load(f)
            gz_path = path + '.gz'
            with gzip.open(gz_path, 'wt', encoding='utf-8') as gz:
                json.dump(data, gz, separators=(',', ':'))
            os.remove(path)
