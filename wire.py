import re
import os

with open("crates/m2-client/src/py_master_runtime_bridge.rs", "r") as f:
    text = f.read()

replacements = {
    "setattr(AppModule, 'SetFrameSkip', staticmethod(lambda *args: None))": 
        "setattr(AppModule, 'SetFrameSkip', staticmethod(getattr(m2_lib, 'm2_app_set_frame_skip')))",
        
    "setattr(AppModule, 'GetRandom', staticmethod(lambda a, b: random.randint(int(a), int(b))))": 
        "setattr(AppModule, 'GetRandom', staticmethod(getattr(m2_lib, 'm2_app_get_random')))",
        
    "setattr(AppModule, 'GetGlobalTimeStamp', staticmethod(lambda: int(time.time())))": 
        "setattr(AppModule, 'GetGlobalTimeStamp', staticmethod(getattr(m2_lib, 'm2_app_get_global_time_stamp')))",
        
    "setattr(AppModule, 'SetMouseHandler', staticmethod(lambda handler=None: None))": 
        "setattr(AppModule, 'SetMouseHandler', staticmethod(getattr(m2_lib, 'm2_app_set_mouse_handler')))",
        
    "setattr(AppModule, 'SetHairColorEnable', staticmethod(lambda *args: None))": 
        "setattr(AppModule, 'SetHairColorEnable', staticmethod(getattr(m2_lib, 'm2_app_set_hair_color_enable')))",
        
    "setattr(AppModule, 'SetArmorSpecularEnable', staticmethod(lambda *args: None))": 
        "setattr(AppModule, 'SetArmorSpecularEnable', staticmethod(getattr(m2_lib, 'm2_app_set_armor_specular_enable')))",
        
    "setattr(AppModule, 'SetWeaponSpecularEnable', staticmethod(lambda *args: None))": 
        "setattr(AppModule, 'SetWeaponSpecularEnable', staticmethod(getattr(m2_lib, 'm2_app_set_weapon_specular_enable')))",
        
    "setattr(AppModule, 'SetCameraMaxDistance', staticmethod(lambda d: None))": 
        "setattr(AppModule, 'SetCameraMaxDistance', staticmethod(getattr(m2_lib, 'm2_app_set_camera_max_distance')))",
        
    "setattr(AppModule, 'SetDefaultFontName', staticmethod(lambda name: None))": 
        "setattr(AppModule, 'SetDefaultFontName', staticmethod(getattr(m2_lib, 'm2_app_set_default_font_name')))",
        
    "setattr(AppModule, 'SetGuildMarkPath', staticmethod(lambda path: None))": 
        "setattr(AppModule, 'SetGuildMarkPath', staticmethod(getattr(m2_lib, 'm2_app_set_guild_mark_path')))",
        
    "setattr(AppModule, 'SetGuildSymbolPath', staticmethod(lambda path: None))": 
        "setattr(AppModule, 'SetGuildSymbolPath', staticmethod(getattr(m2_lib, 'm2_app_set_guild_symbol_path')))",
        
    "setattr(AppModule, 'GetLocaleName', staticmethod(lambda: \"en\"))": 
        "setattr(AppModule, 'GetLocaleName', staticmethod(getattr(m2_lib, 'm2_app_get_locale_name')))",
        
    "setattr(AppModule, 'GetLocalePath', staticmethod(lambda: \"locale/en\"))": 
        "setattr(AppModule, 'GetLocalePath', staticmethod(getattr(m2_lib, 'm2_app_get_locale_path')))",
        
    "setattr(AppModule, 'GetLocalePathCommon', staticmethod(lambda: \"locale/common\"))": 
        "setattr(AppModule, 'GetLocalePathCommon', staticmethod(getattr(m2_lib, 'm2_app_get_locale_path_common')))",
        
    "setattr(AppModule, 'IsRTL', staticmethod(lambda *a: False))": 
        "setattr(AppModule, 'IsRTL', staticmethod(getattr(m2_lib, 'm2_app_is_rtl')))",
        
    "setattr(AppModule, 'IsExistFile', staticmethod(lambda f: PackModule.Exist(f)))": 
        "setattr(AppModule, 'IsExistFile', staticmethod(getattr(m2_lib, 'm2_app_is_exist_file')))",
        
    "setattr(AppModule, 'IsFileExist', staticmethod(lambda f: PackModule.Exist(f)))": 
        "setattr(AppModule, 'IsFileExist', staticmethod(getattr(m2_lib, 'm2_app_is_file_exist')))",
        
    "setattr(AppModule, 'SetSoftwareCursor', staticmethod(lambda: None))": 
        "setattr(AppModule, 'SetSoftwareCursor', staticmethod(getattr(m2_lib, 'm2_py_app_set_software_cursor')))",
        
    "setattr(AppModule, 'SetHardwareCursor', staticmethod(lambda: None))": 
        "setattr(AppModule, 'SetHardwareCursor', staticmethod(getattr(m2_lib, 'm2_py_app_set_hardware_cursor')))",
        
    "setattr(AppModule, 'SetCenterPosition', staticmethod(lambda *args: None))": 
        "setattr(AppModule, 'SetCenterPosition', staticmethod(getattr(m2_lib, 'm2_app_set_center_position')))",
        
    "setattr(AppModule, 'GetCameraPitch', staticmethod(lambda: _app_get_camera()[1]))": 
        "setattr(AppModule, 'GetCameraPitch', staticmethod(getattr(m2_lib, 'm2_py_app_get_camera_pitch')))",
        
    "setattr(AppModule, 'GetCameraRotation', staticmethod(lambda: _app_get_camera()[2]))": 
        "setattr(AppModule, 'GetCameraRotation', staticmethod(getattr(m2_lib, 'm2_py_app_get_camera_rotation')))",
}

# Image info is a tuple so we need a python wrapper to call the three FFI functions:
img_info_wrapper = """def _app_get_image_info(path):
    if not m2_lib: return (False, 0, 0)
    w = m2_lib.m2_app_get_image_info_width(path.encode('utf-8'))
    h = m2_lib.m2_app_get_image_info_height(path.encode('utf-8'))
    valid = m2_lib.m2_app_get_image_info_valid(path.encode('utf-8'))
    return (bool(valid), w, h)
setattr(AppModule, 'GetImageInfo', staticmethod(_app_get_image_info))"""

text = text.replace("setattr(AppModule, 'GetImageInfo', staticmethod(lambda path: (False, 0, 0)))", img_info_wrapper)

for k, v in replacements.items():
    if k not in text:
        print(f"NOT FOUND: {k}")
    text = text.replace(k, v)

with open("crates/m2-client/src/py_master_runtime_bridge.rs", "w") as f:
    f.write(text)
