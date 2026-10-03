import os
import re

exports = []
for file in os.listdir('crates/m2-client/src/'):
    if file.startswith('recv_'):
        path = os.path.join('crates/m2-client/src/', file)
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
            matches = re.findall(r'pub\s+(?:extern\s+"C"\s+)?fn\s+([a-zA-Z0-9_]+)\(', content)
            exports.extend(matches)

for exp in exports:
    if 'm2_' in exp:
        print(exp)
