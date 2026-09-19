import sys

def remove_diacritics(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    mapping = {
        'ą': 'a', 'ć': 'c', 'ę': 'e', 'ł': 'l', 'ń': 'n', 'ó': 'o', 'ś': 's', 'ź': 'z', 'ż': 'z',
        'Ą': 'A', 'Ć': 'C', 'Ę': 'E', 'Ł': 'L', 'Ń': 'N', 'Ó': 'O', 'Ś': 'S', 'Ź': 'Z', 'Ż': 'Z'
    }

    for k, v in mapping.items():
        content = content.replace(k, v)

    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)

remove_diacritics('docs/04_python_bindings/03_chrmgr_and_instance_base.md')
