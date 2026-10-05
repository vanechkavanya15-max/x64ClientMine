import re

with open("crates/m2-client/src/py_master_runtime_bridge.rs", "r") as f:
    text = f.read()

new_methods = """
    ("m2_app_set_frame_skip", "m2_app_set_frame_skip", [ctypes.c_int32], None),
    ("m2_app_get_random", "m2_app_get_random", [ctypes.c_int32, ctypes.c_int32], ctypes.c_int32),
    ("m2_app_get_global_time_stamp", "m2_app_get_global_time_stamp", [], ctypes.c_int32),
    ("m2_app_set_mouse_handler", "m2_app_set_mouse_handler", [ctypes.c_int64], None),
    ("m2_app_set_hair_color_enable", "m2_app_set_hair_color_enable", [ctypes.c_int32], None),
    ("m2_app_set_armor_specular_enable", "m2_app_set_armor_specular_enable", [ctypes.c_int32], None),
    ("m2_app_set_weapon_specular_enable", "m2_app_set_weapon_specular_enable", [ctypes.c_int32], None),
    ("m2_app_set_camera_max_distance", "m2_app_set_camera_max_distance", [ctypes.c_float], None),
    ("m2_app_set_default_font_name", "m2_app_set_default_font_name", [ctypes.c_char_p], None),
    ("m2_app_set_guild_mark_path", "m2_app_set_guild_mark_path", [ctypes.c_char_p], None),
    ("m2_app_set_guild_symbol_path", "m2_app_set_guild_symbol_path", [ctypes.c_char_p], None),
    ("m2_app_get_locale_name", "m2_app_get_locale_name", [], ctypes.c_char_p),
    ("m2_app_get_locale_path", "m2_app_get_locale_path", [], ctypes.c_char_p),
    ("m2_app_get_locale_path_common", "m2_app_get_locale_path_common", [], ctypes.c_char_p),
    ("m2_app_is_rtl", "m2_app_is_rtl", [], ctypes.c_int32),
    ("m2_app_is_exist_file", "m2_app_is_exist_file", [ctypes.c_char_p], ctypes.c_int32),
    ("m2_app_is_file_exist", "m2_app_is_file_exist", [ctypes.c_char_p], ctypes.c_int32),
    ("m2_app_set_center_position", "m2_app_set_center_position", [ctypes.c_float, ctypes.c_float, ctypes.c_float], None),
    ("m2_app_get_image_info_width", "m2_app_get_image_info_width", [ctypes.c_char_p], ctypes.c_int32),
    ("m2_app_get_image_info_height", "m2_app_get_image_info_height", [ctypes.c_char_p], ctypes.c_int32),
    ("m2_app_get_image_info_valid", "m2_app_get_image_info_valid", [ctypes.c_char_p], ctypes.c_int32),
"""

# Insert into app_methods = [
if "app_methods = [" in text:
    text = text.replace("app_methods = [", "app_methods = [" + new_methods)
else:
    print("Could not find app_methods = [")

with open("crates/m2-client/src/py_master_runtime_bridge.rs", "w") as f:
    f.write(text)
