with open("crates/m2-client/src/lib.rs", 'r') as f:
    lines = f.readlines()

new_lines = []
found = False
for line in lines:
    if "game_item_refine_socket_kd_manager" in line:
        if not found:
            new_lines.append(line)
            found = True
    else:
        new_lines.append(line)
        
with open("crates/m2-client/src/lib.rs", 'w') as f:
    f.writelines(new_lines)

