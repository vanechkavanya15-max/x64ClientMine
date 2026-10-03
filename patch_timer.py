import re

with open("crates/m2-client/src/py_bridge_game_systems.rs", "r") as f:
    content = f.read()

content = content.replace("crate::timer::get_msec()", "m2_client_bridge::timer::get_msec()")

with open("crates/m2-client/src/py_bridge_game_systems.rs", "w") as f:
    f.write(content)
