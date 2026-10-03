import re

with open("crates/m2-client/src/recv_refine_packets_complete.rs", "r") as f:
    content = f.read()

content = content.replace("m2_python_call_open_refine_dialog", "m2_recv_refine_open_dialog")
content = content.replace("m2_python_call_open_refine_dialog_new", "m2_recv_refine_open_dialog_new")
content = content.replace("m2_python_call_append_material_to_refine_dialog", "m2_recv_refine_append_material")

with open("crates/m2-client/src/recv_refine_packets_complete.rs", "w") as f:
    f.write(content)
