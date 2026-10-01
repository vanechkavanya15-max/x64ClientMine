use m2_client_bridge::ffi_guard::safe_ffi_call;
use m2_client_bridge::state::game_state::{GLOBAL_GAME_STATE, POINT_GOLD};
use std::ffi::{c_char, c_void};
use std::ffi::CString;

#[repr(C)]
pub struct PyObject {
    _private: [u8; 0],
}

mod ffi {
    use super::*;

    pub unsafe fn py_arg_parse_tuple_i(args: *mut PyObject, out: *mut i32) -> i32 {
        if args.is_null() || pyo3::ffi::Py_IsInitialized() == 0 {
            if !out.is_null() { *out = 0; }
            return 1;
        }
        pyo3::ffi::PyArg_ParseTuple(args as *mut pyo3::ffi::PyObject, b"i\0".as_ptr() as *const c_char, out)
    }

    pub unsafe fn py_arg_parse_tuple_ii(args: *mut PyObject, out1: *mut i32, out2: *mut i32) -> i32 {
        if args.is_null() || pyo3::ffi::Py_IsInitialized() == 0 {
            if !out1.is_null() { *out1 = 0; }
            if !out2.is_null() { *out2 = 0; }
            return 1;
        }
        pyo3::ffi::PyArg_ParseTuple(args as *mut pyo3::ffi::PyObject, b"ii\0".as_ptr() as *const c_char, out1, out2)
    }

    pub unsafe fn py_arg_parse_tuple_iii(args: *mut PyObject, out1: *mut i32, out2: *mut i32, out3: *mut i32) -> i32 {
        if args.is_null() || pyo3::ffi::Py_IsInitialized() == 0 {
            if !out1.is_null() { *out1 = 0; }
            if !out2.is_null() { *out2 = 0; }
            if !out3.is_null() { *out3 = 0; }
            return 1;
        }
        pyo3::ffi::PyArg_ParseTuple(args as *mut pyo3::ffi::PyObject, b"iii\0".as_ptr() as *const c_char, out1, out2, out3)
    }

    pub unsafe fn py_build_value_i(val: i64) -> *mut PyObject {
        if pyo3::ffi::Py_IsInitialized() == 0 {
            return val as *mut PyObject;
        }
        pyo3::ffi::PyLong_FromLongLong(val) as *mut PyObject
    }

    pub unsafe fn py_build_value_s(val: *const c_char) -> *mut PyObject {
        if pyo3::ffi::Py_IsInitialized() == 0 {
            return 1 as *mut PyObject;
        }
        pyo3::ffi::PyUnicode_FromString(val) as *mut PyObject
    }

    pub unsafe fn py_build_value_fff(x: f32, y: f32, z: f32) -> *mut PyObject {
        if pyo3::ffi::Py_IsInitialized() == 0 {
            return 1 as *mut PyObject;
        }
        pyo3::ffi::Py_BuildValue(b"(fff)\0".as_ptr() as *const c_char, x as f64, y as f64, z as f64) as *mut PyObject
    }

    pub unsafe fn py_build_value_ii(t: i32, v: i32) -> *mut PyObject {
        if pyo3::ffi::Py_IsInitialized() == 0 {
            return 1 as *mut PyObject;
        }
        pyo3::ffi::Py_BuildValue(b"(ii)\0".as_ptr() as *const c_char, t, v) as *mut PyObject
    }
}


#[no_mangle]
pub unsafe extern "C" fn m2_py_player_GetStatus(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    safe_ffi_call(std::ptr::null_mut(), || {
        let mut point_type: i32 = 0;
        if ffi::py_arg_parse_tuple_i(args, &mut point_type) == 0 {
            return std::ptr::null_mut();
        }
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let val = state.get_point(point_type as u8);
        ffi::py_build_value_i(val)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_GetItemIndex(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    safe_ffi_call(std::ptr::null_mut(), || {
        let mut window: i32 = 0;
        let mut slot: i32 = 0;
        if ffi::py_arg_parse_tuple_ii(args, &mut window, &mut slot) == 0 {
            return std::ptr::null_mut();
        }
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let val = state.get_item_vnum(window as u8, slot as u16);
        ffi::py_build_value_i(val as i64)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_GetItemCount(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    safe_ffi_call(std::ptr::null_mut(), || {
        let mut window: i32 = 0;
        let mut slot: i32 = 0;
        if ffi::py_arg_parse_tuple_ii(args, &mut window, &mut slot) == 0 {
            return std::ptr::null_mut();
        }
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let val = state.get_item_count(window as u8, slot as u16);
        ffi::py_build_value_i(val as i64)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_GetItemFlags(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    safe_ffi_call(std::ptr::null_mut(), || {
        let mut window: i32 = 0;
        let mut slot: i32 = 0;
        if ffi::py_arg_parse_tuple_ii(args, &mut window, &mut slot) == 0 {
            return std::ptr::null_mut();
        }
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let val = state.get_item(window as u8, slot as u16).map(|it| it.flags).unwrap_or(0);
        ffi::py_build_value_i(val as i64)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_GetItemMetinSocket(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    safe_ffi_call(std::ptr::null_mut(), || {
        let mut window: i32 = 0;
        let mut slot: i32 = 0;
        let mut socket_idx: i32 = 0;
        if ffi::py_arg_parse_tuple_iii(args, &mut window, &mut slot, &mut socket_idx) == 0 {
            return std::ptr::null_mut();
        }
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let val = state.get_item(window as u8, slot as u16)
            .map(|it| {
                if socket_idx >= 0 && socket_idx < 3 {
                    it.sockets[socket_idx as usize]
                } else {
                    0
                }
            })
            .unwrap_or(0);
        ffi::py_build_value_i(val as i64)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_GetItemAttribute(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    safe_ffi_call(std::ptr::null_mut(), || {
        let mut window: i32 = 0;
        let mut slot: i32 = 0;
        let mut attr_idx: i32 = 0;
        if ffi::py_arg_parse_tuple_iii(args, &mut window, &mut slot, &mut attr_idx) == 0 {
            return std::ptr::null_mut();
        }
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let (a_type, a_value) = state.get_item(window as u8, slot as u16)
            .map(|it| {
                if attr_idx >= 0 && attr_idx < 7 {
                    let attr = it.attrs[attr_idx as usize];
                    (attr.0 as i32, attr.1 as i32)
                } else {
                    (0, 0)
                }
            })
            .unwrap_or((0, 0));
        ffi::py_build_value_ii(a_type, a_value)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_GetMoney(_self: *mut PyObject, _args: *mut PyObject) -> *mut PyObject {
    safe_ffi_call(std::ptr::null_mut(), || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let val = state.get_point(POINT_GOLD);
        ffi::py_build_value_i(val)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_GetName(_self: *mut PyObject, _args: *mut PyObject) -> *mut PyObject {
    safe_ffi_call(std::ptr::null_mut(), || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let vid = state.get_player_vid();
        let mut name = String::new();
        if let Some(actor) = state.get_actor(vid) {
            name = actor.name.clone();
        }
        let c_name = CString::new(name).unwrap_or_else(|_| CString::new("").unwrap());
        ffi::py_build_value_s(c_name.as_ptr())
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_GetMainCharacterPosition(_self: *mut PyObject, _args: *mut PyObject) -> *mut PyObject {
    safe_ffi_call(std::ptr::null_mut(), || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let (x, y, z, _rot) = state.get_player_pos();
        ffi::py_build_value_fff(x, y, z)
    })
}

#[no_mangle]
pub unsafe extern "C" fn m2_py_player_GetMainCharacterIndex(_self: *mut PyObject, _args: *mut PyObject) -> *mut PyObject {
    safe_ffi_call(std::ptr::null_mut(), || {
        let state = GLOBAL_GAME_STATE.read().unwrap();
        let vid = state.get_player_vid();
        ffi::py_build_value_i(vid as i64)
    })
}