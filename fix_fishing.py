content = open("crates/m2-client/src/fishing_logic_v2.rs", "r").read()
if "m2_fishing_get_hit_window" in content:
    content = content.replace('pub extern "C" fn m2_fishing_get_hit_window', 'pub extern "C" fn m2_fishing_get_hit_window_dup')
    open("crates/m2-client/src/fishing_logic_v2.rs", "w").write(content)
