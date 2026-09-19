import re
import sys

def extract_structs(file_path):
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except UnicodeDecodeError:
        try:
            with open(file_path, 'r', encoding='iso-8859-1') as f:
                content = f.read()
        except Exception as e:
            print(f"Error reading {file_path}: {e}")
            return
    except Exception as e:
         print(f"Error reading {file_path}: {e}")
         return

    print(f"=== Structs in {file_path} ===")
    struct_pattern = re.compile(r'struct\s+\w+\s*{.*?};', re.DOTALL)
    for match in struct_pattern.finditer(content):
        print(match.group(0))
        print("-" * 40)
    
    print(f"=== Classes in {file_path} ===")
    class_pattern = re.compile(r'class\s+\w+[\s\w,:]*{.*?};', re.DOTALL)
    # Be careful, these could be huge. Let's just get the class declaration up to {
    class_decl_pattern = re.compile(r'class\s+\w+[\s\w,:]*{')
    for match in class_decl_pattern.finditer(content):
        print(match.group(0) + " ... };")
        
if __name__ == "__main__":
    if len(sys.argv) > 1:
        extract_structs(sys.argv[1])
    else:
        extract_structs('source/UserInterface/InstanceBase.h')
        extract_structs('source/UserInterface/PythonCharacterManager.h')
