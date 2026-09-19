import re
import sys

def extract_methods(file_path):
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except UnicodeDecodeError:
        try:
            with open(file_path, 'r', encoding='iso-8859-1') as f:
                content = f.read()
        except Exception:
            return
            
    pattern = re.compile(r'^\s*(?:virtual\s+)?[\w\*<>\s_:]+\s+(\w+)\s*\([^)]*\)\s*(?:const)?\s*(?:{|;)', re.MULTILINE)
    print(f"=== Methods in {file_path} ===")
    for match in pattern.finditer(content):
        print(match.group(0).strip())

if __name__ == "__main__":
    extract_methods(sys.argv[1])
