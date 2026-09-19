import sys

def fix_markdown(filepath):
    with open(filepath, 'r', encoding='ascii') as f:
        lines = f.readlines()
    
    out_lines = []
    for i, line in enumerate(lines):
        line = line.rstrip() # remove trailing spaces
        out_lines.append(line + "\n")
        
        # Add blank line after headers if not present
        if line.startswith("### "):
            if i + 1 < len(lines) and lines[i+1].strip() != "":
                out_lines.append("\n")
                
    # Basic fix for lists, but the generate script is easier to modify
    with open(filepath, 'w', encoding='ascii') as f:
        f.writelines(out_lines)

fix_markdown('docs/CLIENT_SYSTEM_MAP.md')
