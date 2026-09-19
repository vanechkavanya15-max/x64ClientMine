import re
import sys

def extract_methods(filename):
    with open(filename, 'r', encoding='ISO-8859-1') as f:
        content = f.read()

    # Look for PyMethodDef array
    pattern = r"PyMethodDef\s+\w+\[\]\s*=\s*\{([\s\S]*?)\};"
    match = re.search(pattern, content)
    
    if match:
        methods_block = match.group(1)
        # Extract individual method entries: { "MethodName", c_function_name, METH_VARARGS }
        method_pattern = r'\{\s*"([^"]+)"\s*,\s*([^,]+)\s*,\s*([^}]+)\s*\}'
        methods = re.findall(method_pattern, methods_block)
        
        for name, func, flags in methods:
            func = func.strip()
            if func == "NULL":
                continue
            
            # Now find the function definition
            func_pattern = r"PyObject\s*\*\s*" + func + r"\s*\([^)]*\)\s*\{([\s\S]*?)\n\}"
            func_match = re.search(func_pattern, content)
            
            print(f"Method: {name} -> {func}")
            if func_match:
                print(func_match.group(0))
            else:
                print("Function definition not found.")
            print("-" * 40)

extract_methods("source/UserInterface/PythonPlayerModule.cpp")
extract_methods("source/UserInterface/PythonSkill.cpp")
