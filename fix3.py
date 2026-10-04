with open("crates/m2-client/src/py_bridge_player_items.rs", "r") as f:
    text = f.read()

import re

# Remove m2_py_player_can_attach_metin and m2_py_player_can_detach from py_bridge_player_items.rs

text = re.sub(r'#\[no_mangle\]\s*pub extern "C" fn m2_py_player_can_attach_metin.*?\n}', '', text, flags=re.DOTALL)
text = re.sub(r'#\[no_mangle\]\s*pub extern "C" fn m2_py_player_can_detach.*?\n}', '', text, flags=re.DOTALL)

with open("crates/m2-client/src/py_bridge_player_items.rs", "w") as f:
    f.write(text)

