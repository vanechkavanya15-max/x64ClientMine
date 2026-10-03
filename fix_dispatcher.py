with open("crates/m2-client/src/win32_message_dispatcher.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    new_lines.append(line)
    if "state.has_focus = false;" in line:
        new_lines.append("            // Clean up keyboard state on focus lost is handled externally via input_focus_lost_cleanup::process_focus_lost_cleanup\n")

with open("crates/m2-client/src/win32_message_dispatcher.rs", "w") as f:
    f.writelines(new_lines)
