use std::ffi::{CString, c_char, c_void};

#[cfg(windows)]
extern "system" {
    fn LoadLibraryA(lpLibFileName: *const c_char) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void;
}

#[test]
fn test_python_embed_314() {
    #[cfg(windows)]
    unsafe {
        let dll_path = CString::new("E:/m2dev-client-src-main/m2dev-client-main/python314.dll").unwrap();
        let h = LoadLibraryA(dll_path.as_ptr());
        if h.is_null() {
            println!("python314.dll could not be loaded directly from path");
            return;
        }

        let py_init_sym = GetProcAddress(h, CString::new("Py_InitializeEx").unwrap().as_ptr());
        let py_run_sym = GetProcAddress(h, CString::new("PyRun_SimpleString").unwrap().as_ptr());

        assert!(!py_init_sym.is_null(), "Py_InitializeEx must exist");
        assert!(!py_run_sym.is_null(), "PyRun_SimpleString must exist");

        let py_init: extern "C" fn(i32) = std::mem::transmute(py_init_sym);
        let py_run: extern "C" fn(*const c_char) -> i32 = std::mem::transmute(py_run_sym);

        py_init(0);

        // Mount root.pck first in Rust
        let _ = m2_client::vfs_pck_memory_mapped::mount_archive("E:/m2dev-client-src-main/m2dev-client-main/pack/root.pck");

        let code = CString::new(r#"
import sys, types, ctypes

dll = ctypes.CDLL("E:/m2dev-client-src-main/m2dev-client-main/m2_client.dll")

pack_exist_fn = dll.m2_py_pack_exist
pack_exist_fn.argtypes = [ctypes.c_char_p]
pack_exist_fn.restype = ctypes.c_bool

pack_size_fn = dll.m2_py_pack_get_size
pack_size_fn.argtypes = [ctypes.c_char_p]
pack_size_fn.restype = ctypes.c_size_t

pack_read_fn = dll.m2_py_pack_read_data
pack_read_fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_size_t]
pack_read_fn.restype = ctypes.c_size_t

# Mount all packs
for p in ["root.pck", "locale.pck", "uiscript.pck", "ETC.pck", "uiloading.pck"]:
    dll.m2_py_pack_mount(f"E:/m2dev-client-src-main/m2dev-client-main/pack/{p}".encode('utf-8'))

class PackModule:
    @staticmethod
    def Exist(filename):
        if not pack_exist_fn:
            return False
        return pack_exist_fn(filename.encode('utf-8'))
    @staticmethod
    def Get(filename):
        if not pack_size_fn or not pack_read_fn:
            return b''
        f_bytes = filename.encode('utf-8')
        size = pack_size_fn(f_bytes)
        if size == 0:
            return b''
        buf = ctypes.create_string_buffer(size)
        read_len = pack_read_fn(f_bytes, buf, size)
        return buf.raw[:read_len]

sys.modules['pack'] = PackModule

# dbg module
log_file = open("python_boot.log", "w", encoding="utf-8")
class DbgModule:
    @staticmethod
    def Trace(msg):
        log_file.write(f"[DBG] {msg}\n")
        log_file.flush()
    @staticmethod
    def TraceError(msg):
        log_file.write(f"[DBG ERROR] {msg}\n")
        log_file.flush()
    @staticmethod
    def RegisterExceptionString(msg):
        pass
    @staticmethod
    def LogBox(msg, title="Error"):
        log_file.write(f"[LOGBOX {title}] {msg}\n")
        log_file.flush()

sys.modules['dbg'] = DbgModule

# app module
class AppModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 0
        return lambda *args, **kwargs: 0

class AppModule(metaclass=AppModuleMeta):
    INFO_ITEM = 1
    INFO_ACTOR = 2
    INFO_EFFECT = 3
    INFO_TEXTTAIL = 4
    NORMAL = 0
    ATTACK = 1
    TARGET = 2
    TALK = 3
    CANT_GO = 4
    PICK = 5
    DOOR = 6
    CHAIR = 7
    MAGIC = 8
    BUY = 9
    SELL = 10
    CAMERA_ROTATE = 11
    HSIZE = 12
    VSIZE = 13
    HVSIZE = 14
    CAMERA_TO_POSITIVE = 1
    CAMERA_TO_NEGATIVE = 2
    CAMERA_STOP = 0
    ENABLE_COSTUME_SYSTEM = 1
    ENABLE_ENERGY_SYSTEM = 1
    ENABLE_DRAGON_SOUL_SYSTEM = 1
    DIK_ESCAPE = 1
    DIK_1 = 2
    DIK_2 = 3
    DIK_3 = 4
    DIK_4 = 5
    DIK_5 = 6
    DIK_6 = 7
    DIK_7 = 8
    DIK_8 = 9
    DIK_9 = 10
    DIK_0 = 11
    DIK_RETURN = 28
    DIK_SPACE = 57
    DIK_TAB = 15
    DIK_UP = 200
    DIK_LEFT = 203
    DIK_RIGHT = 205
    DIK_DOWN = 208

    @staticmethod
    def Abort():
        print("[APP] Abort called")
    @staticmethod
    def Exit():
        print("[APP] Exit called")
    @staticmethod
    def GetWidth():
        return 1024
    @staticmethod
    def GetHeight():
        return 768
    @staticmethod
    def GetTime():
        import time
        return time.time()
    @staticmethod
    def GetGlobalTime():
        import time
        return int(time.time())
    @staticmethod
    def GetGlobalTimeStamp():
        import time
        return int(time.time())
    @staticmethod
    def SetHairColorEnable(enable):
        pass
    @staticmethod
    def SetArmorSpecularEnable(enable):
        pass
    @staticmethod
    def SetWeaponSpecularEnable(enable):
        pass
    @staticmethod
    def SetMouseHandler(handler):
        pass
    @staticmethod
    def Create(title, w, h, fullscreen):
        print(f"[APP] Create('{title}', {w}, {h}, fullscreen={fullscreen})")
    @staticmethod
    def SetCamera(dist, pitch, rot, target_h):
        pass
    @staticmethod
    def Loop():
        print("[APP] Loop() called - starting main game loop")
    @staticmethod
    def IsPressed(key):
        return False
    @staticmethod
    def GetCursorPosition():
        return (0, 0)
    @staticmethod
    def EnableSpecialCameraMode():
        pass
    @staticmethod
    def GetLocaleName():
        return "en"
    @staticmethod
    def GetLocalePath():
        return "locale/en"
    @staticmethod
    def GetLocalePathCommon():
        return "locale/common"
    @staticmethod
    def LoadLocaleData(path):
        return True
    @staticmethod
    def IsRTL(*args):
        return False
    @staticmethod
    def SetDefaultFontName(font_name):
        pass
    @staticmethod
    def SetTextTailLivingTime(time):
        pass
    @staticmethod
    def EnablePerformanceTime():
        pass
    @staticmethod
    def SetSkillEffectUpgradeEnable(enable):
        pass
    @staticmethod
    def SetTwoHandedWeaponAttSpeedDecreaseValue(val):
        pass
    @staticmethod
    def SetRideHorseEnable(enable):
        pass
    @staticmethod
    def SetCameraMaxDistance(dist):
        pass
    @staticmethod
    def SetMinFog(fog):
        pass
    @staticmethod
    def SetFrameSkip(skip):
        pass
    @staticmethod
    def GetImageInfo(v):
        return 0
    @staticmethod
    def GetInfo(v):
        return 0
    @staticmethod
    def UpdateGame():
        pass
    @staticmethod
    def RenderGame():
        pass
    @staticmethod
    def Process():
        pass
    @staticmethod
    def IsExistFile(f):
        return True
    @staticmethod
    def GetFileList(pattern):
        return []
    @staticmethod
    def GetCamera():
        return (0.0, 0.0, 0.0)
    @staticmethod
    def GetCameraPitch():
        return 30.0
    @staticmethod
    def GetCameraRotation():
        return 0.0
    @staticmethod
    def GetUpdateFPS():
        return 60
    @staticmethod
    def GetRenderFPS():
        return 60
    @staticmethod
    def RotateCamera(deg):
        pass
    @staticmethod
    def PitchCamera(deg):
        pass
    @staticmethod
    def ZoomCamera(dist):
        pass
    @staticmethod
    def SetFPS(fps):
        pass
    @staticmethod
    def SetGlobalCenterPosition(x, y, z):
        pass
    @staticmethod
    def SetCenterPosition(x, y, z):
        pass
    @staticmethod
    def GetRandom(a, b):
        return a
    @staticmethod
    def SetCursor(c):
        pass
    @staticmethod
    def GetCursor():
        return 0
    @staticmethod
    def ShowCursor():
        pass
    @staticmethod
    def HideCursor():
        pass
    @staticmethod
    def IsShowCursor():
        return True
    @staticmethod
    def SetConnectData(ip, port):
        pass
    @staticmethod
    def GetConnectData():
        return ("", 0)
    @staticmethod
    def Sleep(ms):
        pass
    @staticmethod
    def SetGuildSymbolPath(path):
        pass
    @staticmethod
    def SetGuildMarkPath(path):
        pass
    @staticmethod
    def SetCameraSpeed(s):
        pass
    @staticmethod
    def SaveCameraSetting(name):
        pass
    @staticmethod
    def LoadCameraSetting(name):
        pass
    @staticmethod
    def SetDefaultCamera():
        pass
    @staticmethod
    def SetCameraSetting(val):
        pass
    @staticmethod
    def SetSightRange(r):
        pass
    @staticmethod
    def IsFileExist(f):
        return True
    @staticmethod
    def OpenTextFile(f):
        return 0
    @staticmethod
    def CloseTextFile(h):
        pass
    @staticmethod
    def GetTextFileLineCount(h):
        return 0
    @staticmethod
    def GetTextFileLine(h, l):
        return ""
    @staticmethod
    def SetControlFP():
        pass
    @staticmethod
    def SetSpecularSpeed(s):
        pass
    @staticmethod
    def SetVisibleNotice(v):
        pass
    @staticmethod
    def IsVisibleNotice():
        return True
    @staticmethod
    def EnableTestServerFlag():
        pass
    @staticmethod
    def IsEnableTestServerFlag():
        return False
    @staticmethod
    def ReloadLocaleConfig():
        pass
    @staticmethod
    def ReloadLocale():
        pass

sys.modules['app'] = AppModule

# systemSetting module
class SystemSettingModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 0
        return lambda *args, **kwargs: 0

class SystemSettingModule(metaclass=SystemSettingModuleMeta):
    WINDOW_STATUS = 0
    WINDOW_INVENTORY = 1
    WINDOW_ABILITY = 2
    WINDOW_SOCIETY = 3
    WINDOW_JOURNAL = 4
    WINDOW_COMMAND = 5
    WINDOW_QUICK = 6
    WINDOW_GAUGE = 7
    WINDOW_MINIMAP = 8
    WINDOW_CHAT = 9

    @staticmethod
    def GetWidth():
        return 1024
    @staticmethod
    def GetHeight():
        return 768
    @staticmethod
    def GetBPP():
        return 32
    @staticmethod
    def GetFogLevel():
        return 0
    @staticmethod
    def SetFogLevel(l):
        pass
    @staticmethod
    def SetInterfaceHandler(h):
        pass
    @staticmethod
    def DestroyInterfaceHandler():
        pass
    @staticmethod
    def ReserveResource(r):
        pass
    @staticmethod
    def isInterfaceConfig():
        return 1
    @staticmethod
    def SaveWindowStatus(*args):
        pass
    @staticmethod
    def GetWindowStatus(w):
        return (0, 0, 1)
    @staticmethod
    def GetResolutionCount():
        return 1
    @staticmethod
    def GetFrequencyCount():
        return 1
    @staticmethod
    def GetCurrentResolution():
        return (1024, 768, 32)
    @staticmethod
    def GetResolution(idx):
        return (1024, 768, 32)
    @staticmethod
    def GetFrequency(idx):
        return 60
    @staticmethod
    def ApplyConfig():
        pass
    @staticmethod
    def SetConfig(cfg):
        pass
    @staticmethod
    def SaveConfig():
        pass
    @staticmethod
    def GetConfig():
        return {}
    @staticmethod
    def SetSaveID(v):
        pass
    @staticmethod
    def isSaveID():
        return 0
    @staticmethod
    def GetSaveID():
        return ""
    @staticmethod
    def GetMusicVolume():
        return 1.0
    @staticmethod
    def GetSoundVolume():
        return 1.0
    @staticmethod
    def SetMusicVolume(vol):
        pass
    @staticmethod
    def SetSoundVolume(vol):
        pass
    @staticmethod
    def IsSoftwareCursor():
        return 0
    @staticmethod
    def SetViewChatFlag(f):
        pass
    @staticmethod
    def IsViewChat():
        return 1
    @staticmethod
    def SetAlwaysShowNameFlag(f):
        pass
    @staticmethod
    def IsAlwaysShowName():
        return 1
    @staticmethod
    def SetShowDamageFlag(f):
        pass
    @staticmethod
    def IsShowDamage():
        return 1
    @staticmethod
    def SetShowSalesTextFlag(f):
        pass
    @staticmethod
    def IsShowSalesText():
        return 1
    @staticmethod
    def GetShadowLevel():
        return 1
    @staticmethod
    def SetShadowLevel(l):
        pass
    @staticmethod
    def SetWidth(w):
        pass
    @staticmethod
    def SetHeight(h):
        pass
    @staticmethod
    def Save():
        pass

sys.modules['systemSetting'] = SystemSettingModule
sys.modules['system'] = SystemSettingModule

# wndMgr module
class WndMgrModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 0
        return lambda *args, **kwargs: 0

class WndMgrModule(metaclass=WndMgrModuleMeta):
    _handle_seq = 100
    SLOT_STYLE_NONE = 0
    SLOT_STYLE_PICK_UP = 1
    SLOT_STYLE_SELECT = 2

    HORIZONTAL_ALIGN_LEFT = 0
    HORIZONTAL_ALIGN_CENTER = 1
    HORIZONTAL_ALIGN_RIGHT = 2
    VERTICAL_ALIGN_TOP = 0
    VERTICAL_ALIGN_CENTER = 1
    VERTICAL_ALIGN_BOTTOM = 2

    @staticmethod
    def SetMouseHandler(handler):
        pass
    @staticmethod
    def SetScreenSize(w, h):
        log_file.write(f"[wndMgr] SetScreenSize({w}, {h})\n")
    @staticmethod
    def GetScreenWidth():
        return 1024
    @staticmethod
    def GetScreenHeight():
        return 768
    @staticmethod
    def AttachIcon(*args):
        pass
    @staticmethod
    def DeattachIcon(*args):
        pass
    @staticmethod
    def SetAttachingFlag(flag):
        pass
    @staticmethod
    def GetAspect():
        return 1024.0 / 768.0
    @staticmethod
    def GetHyperlink():
        return ""
    @staticmethod
    def OnceIgnoreMouseLeftButtonUpEvent():
        pass
    @classmethod
    def RegisterGridSlotWindow(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterMarkBox(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterBar3D(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterNumberLine(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @staticmethod
    def AddFlag(handle, flag):
        pass
    @classmethod
    def Register(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterSlotWindow(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterTextLine(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterImageBox(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterExpandedImageBox(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterAniImageBox(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterButton(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterRadioButton(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterToggleButton(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterDragButton(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterBar(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterLine(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterBox(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @classmethod
    def RegisterWindow(cls, win, layer="UI"):
        cls._handle_seq += 1
        return cls._handle_seq
    @staticmethod
    def SetPosition(*args):
        pass
    @staticmethod
    def SetSize(*args):
        pass
    @staticmethod
    def SetWindowPosition(*args):
        pass
    @staticmethod
    def SetWindowSize(*args):
        pass
    @staticmethod
    def GetWindowWidth(*args):
        return 100
    @staticmethod
    def GetWindowHeight(*args):
        return 30
    @staticmethod
    def GetWindowLocalPosition(*args):
        return (0, 0)
    @staticmethod
    def GetWindowGlobalPosition(*args):
        return (0, 0)
    @staticmethod
    def GetWindowRect(*args):
        return (0, 0, 100, 30)
    @staticmethod
    def SetWindowHorizontalAlign(*args):
        pass
    @staticmethod
    def SetWindowVerticalAlign(*args):
        pass
    @staticmethod
    def SetParent(*args):
        pass
    @staticmethod
    def SetPickAlways(*args):
        pass
    @staticmethod
    def Show(*args):
        pass
    @staticmethod
    def Hide(*args):
        pass
    @staticmethod
    def SetTop(*args):
        pass
    @staticmethod
    def IsShow(*args):
        return True
    @staticmethod
    def SetTextColor(*args):
        pass
    @staticmethod
    def SetFontName(*args):
        pass
    @staticmethod
    def SetFontColor(*args):
        pass
    @staticmethod
    def SetText(*args):
        pass
    @staticmethod
    def GetText(*args):
        return ""
    @staticmethod
    def GetTextSize(*args):
        return (100, 20)
    @staticmethod
    def GetTextWidth(*args):
        return 100
    @staticmethod
    def GetTextHeight(*args):
        return 20
    @staticmethod
    def SetRenderingMode(*args):
        pass
    @staticmethod
    def SetRenderingRect(*args):
        pass
    @staticmethod
    def SetDiffuseColor(*args):
        pass
    @staticmethod
    def SetScale(*args):
        pass
    @staticmethod
    def SetOrigin(*args):
        pass
    @staticmethod
    def SetRotation(*args):
        pass
    @staticmethod
    def Update(*args):
        pass
    @staticmethod
    def Render(*args):
        pass
    @staticmethod
    def Destroy(*args):
        pass
    @staticmethod
    def SetName(*args):
        pass
    @staticmethod
    def GetName(*args):
        return ""
    @staticmethod
    def IsFocus(*args):
        return False
    @staticmethod
    def SetFocus(*args):
        pass
    @staticmethod
    def KillFocus(*args):
        pass
    @staticmethod
    def GetMousePosition(*args):
        return (0, 0)
    @staticmethod
    def GetMouseLocalPosition(*args):
        return (0, 0)
    @staticmethod
    def IsRTL(*args):
        return False
    @staticmethod
    def SetHorizontalAlign(*args):
        pass
    @staticmethod
    def SetVerticalAlign(*args):
        pass
    @staticmethod
    def SetMax(*args):
        pass
    @staticmethod
    def SetSecret(*args):
        pass
    @staticmethod
    def SetOutline(*args):
        pass
    @staticmethod
    def SetFeather(*args):
        pass
    @staticmethod
    def SetMultiLine(*args):
        pass
    @staticmethod
    def SetLimitWidth(*args):
        pass
    @staticmethod
    def SetBaseDirection(*args):
        pass
    @staticmethod
    def ShowCursor(*args):
        pass
    @staticmethod
    def HideCursor(*args):
        pass
    @staticmethod
    def SetSelection(*args):
        pass
    @staticmethod
    def ClearSelection(*args):
        pass
    @staticmethod
    def GetCursorPosition(*args):
        return (0, 0)
    @staticmethod
    def SetNumber(*args):
        pass
    @staticmethod
    def SetPath(*args):
        pass
    @staticmethod
    def LoadImage(*args):
        if len(args) >= 2:
            log_file.write(f"[wndMgr] LoadImage({args[0]}, '{args[1]}')\n")
    @staticmethod
    def LoadImageFromFile(*args):
        if len(args) >= 2:
            log_file.write(f"[wndMgr] LoadImageFromFile({args[0]}, '{args[1]}')\n")
    @staticmethod
    def GetWidth(*args):
        return 100
    @staticmethod
    def GetHeight(*args):
        return 30
    @staticmethod
    def SetDelay(*args):
        pass
    @staticmethod
    def AppendImage(*args):
        pass
    @staticmethod
    def SetUpVisual(*args):
        pass
    @staticmethod
    def SetOverVisual(*args):
        pass
    @staticmethod
    def SetDownVisual(*args):
        pass
    @staticmethod
    def SetDisableVisual(*args):
        pass
    @staticmethod
    def GetUpVisualFileName(*args):
        return ""
    @staticmethod
    def GetOverVisualFileName(*args):
        return ""
    @staticmethod
    def GetDownVisualFileName(*args):
        return ""
    @staticmethod
    def Flash(*args):
        pass
    @staticmethod
    def Enable(*args):
        pass
    @staticmethod
    def Disable(*args):
        pass
    @staticmethod
    def Down(*args):
        pass
    @staticmethod
    def SetUp(*args):
        pass
    @staticmethod
    def IsDown(*args):
        return False
    @staticmethod
    def AppendSlot(*args):
        pass
    @staticmethod
    def ArrangeSlot(*args):
        pass
    @staticmethod
    def ClearSlot(*args):
        pass
    @staticmethod
    def ClearAllSlot(*args):
        pass
    @staticmethod
    def HasSlot(*args):
        return False
    @staticmethod
    def SetSlot(*args):
        pass
    @staticmethod
    def SetSlotCount(*args):
        pass
    @staticmethod
    def SetSlotCountNew(*args):
        pass
    @staticmethod
    def SetSlotCoolTime(*args):
        pass
    @staticmethod
    def RefreshSlot(*args):
        pass
    @staticmethod
    def SetSlotType(*args):
        pass
    @staticmethod
    def SetSlotStyle(*args):
        pass
    @staticmethod
    def SetSlotBaseImage(*args):
        pass
    @staticmethod
    def SetColor(*args):
        pass
    @staticmethod
    def UpdateRect(*args):
        pass
    @staticmethod
    def SetLimitBias(*args):
        pass
    @staticmethod
    def EnableScissorRect(*args):
        pass
    @staticmethod
    def DisableScissorRect(*args):
        pass
    @staticmethod
    def IsScissorRectEnabled(*args):
        return False
    @staticmethod
    def SetOutlineFlag(*args):
        pass
    @staticmethod
    def ShowOverInWindowName(*args):
        pass
    @staticmethod
    def IsDragging(*args):
        return False
    @staticmethod
    def IsPickedWindow(*args):
        return False
    @staticmethod
    def IsIn(*args):
        return False
    @staticmethod
    def GetChildCount(*args):
        return 0

sys.modules['wndMgr'] = WndMgrModule

# grp module
class GrpModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 0
        return lambda *args, **kwargs: 0

class GrpModule(metaclass=GrpModuleMeta):
    @staticmethod
    def GenerateColor(r, g, b, a=1.0):
        # Convert float components (0.0..1.0) to 32-bit ARGB
        ia = int(a * 255.0) & 0xff
        ir = int(r * 255.0) & 0xff
        ig = int(g * 255.0) & 0xff
        ib = int(b * 255.0) & 0xff
        return (ia << 24) | (ir << 16) | (ig << 8) | ib
    @staticmethod
    def Init():
        pass
    @staticmethod
    def ClearDepthBuffer():
        pass
    @staticmethod
    def SetClearColor(r, g, b, a):
        pass
    @staticmethod
    def SetViewport(x, y, w, h):
        pass
    @staticmethod
    def SetPerspective(fov, aspect, near, far):
        pass
    @staticmethod
    def SetOrtho2d(w, h):
        pass
    @staticmethod
    def Identity():
        pass
    @staticmethod
    def PushMatrix():
        pass
    @staticmethod
    def PopMatrix():
        pass
    @staticmethod
    def Translate(x, y, z):
        pass
    @staticmethod
    def SetDiffuseColor(r, g, b, a):
        pass
    @staticmethod
    def RenderLine(sx, sy, ex, ey):
        pass
    @staticmethod
    def RenderBox(sx, sy, ex, ey):
        pass
    @staticmethod
    def RenderBar(sx, sy, ex, ey):
        pass
    @staticmethod
    def RenderDownButton(sx, sy, ex, ey):
        pass
    @staticmethod
    def RenderUpButton(sx, sy, ex, ey):
        pass

sys.modules['grp'] = GrpModule
sys.modules['grpText'] = GrpModule
sys.modules['grpImage'] = GrpModule

# net module
class NetModuleMeta(type):
    def __getattr__(cls, name):
        if name.isupper():
            return 0
        return lambda *args, **kwargs: 0

class NetModule(metaclass=NetModuleMeta):
    @staticmethod
    def SetServerInfo(info):
        pass
    @staticmethod
    def SetLoginInfo(user, pwd):
        pass
    @staticmethod
    def Connect(ip, port):
        return True
    @staticmethod
    def Disconnect():
        pass
    @staticmethod
    def GetMainStream():
        return True
    @staticmethod
    def SetPhaseWindow(phase, win):
        print(f"[NET] SetPhaseWindow({phase}, {win})")
    @staticmethod
    def ClearPhaseWindow(phase, win):
        pass
    @staticmethod
    def SetAccountConnectorHandler(handler):
        pass
    @staticmethod
    def SendChatPacket(msg, type_val=0):
        pass
    @staticmethod
    def EnablePacketSeq():
        pass

sys.modules['net'] = NetModule

# player, item, chr, chrmgr, snd, guild modules
class DynamicModuleMeta(type):
    def __getattr__(cls, name):
        if name.startswith('__'):
            raise AttributeError(name)
        if name.isupper():
            return 1
        return lambda *args, **kwargs: 0

class DynamicModule(metaclass=DynamicModuleMeta):
    pass

for m in [
    'player', 'Player', 'item', 'Item', 'chr', 'chrmgr', 'snd', 'Snd',
    'guild', 'Guild', 'nonplayer', 'NonPlayer', 'quest', 'Quest',
    'safebox', 'Safebox', 'shop', 'Shop', 'skill', 'Skill',
    'target', 'Target', 'event', 'Event', 'ime', 'IME',
    'miniMap', 'MiniMap', 'profiler', 'Profiler',
    'background', 'Background', 'trade', 'Trade', 'chat', 'Chat',
    'textTail', 'TextTail', 'effect', 'Effect', 'fly', 'Fly',
    'eventmgr', 'EventMgr', 'messenger', 'Messenger',
    'serverStateChecker', 'ServerStateChecker'
]:
    sys.modules[m] = DynamicModule

import builtins
builtins.TRUE = 1
builtins.FALSE = 0

# __main__.__DEBUG__
import __main__
__main__.__DEBUG__ = 1

# Now load and run system.py from root.pck
system_py_data = PackModule.Get('system.py')
print(f"[+] Loaded system.py from root.pck: {len(system_py_data)} bytes")
exec(compile(system_py_data, 'system.py', 'exec'), __main__.__dict__)
print("[+] system.py executed successfully!")
"#).unwrap();
        let ret = py_run(code.as_ptr());
        assert_eq!(ret, 0, "PyRun_SimpleString should succeed with 0");
    }
}
