with open("crates/m2-client/src/wgpu_engine_context.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    new_lines.append(line)
    if "let status = crate::win32_message_dispatcher::poll_window_messages();" in line:
        pass # we'll find a better place to integrate it. The message polling doesn't return individual messages.
