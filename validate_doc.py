import sys

def check_ascii(text):
    try:
        text.encode('ascii')
        return True
    except UnicodeEncodeError as e:
        print(f"Non-ASCII character found: {e.object[e.start:e.end]} at position {e.start}")
        return False

def check_mermaid(text):
    if '```mermaid' not in text:
        print("Mermaid diagram missing.")
        return False
    
    parts = text.split('```mermaid')
    if '```' not in parts[1]:
        print("Mermaid block not closed properly.")
        return False
        
    return True

def main():
    with open('docs/04_python_bindings/05_chat_messenger_guild_api.md', 'r', encoding='utf-8') as f:
        content = f.read()

    is_ascii = check_ascii(content)
    is_mermaid_ok = check_mermaid(content)

    if is_ascii and is_mermaid_ok:
        print("Validation passed.")
        sys.exit(0)
    else:
        print("Validation failed.")
        sys.exit(1)

if __name__ == "__main__":
    main()
