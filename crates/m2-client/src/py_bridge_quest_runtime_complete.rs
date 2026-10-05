//! C-FFI bridge and runtime state for Python `quest` module exports.

use crate::app_timing_clock;
use crate::ui_quest_dialog::get_ui_quest_dialog;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::ffi::{c_char, CStr};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Default)]
pub struct QuestInstance {
    pub index: u32,
    pub icon_file_name: String,
    pub title: String,
    pub clock_name: String,
    pub counter_name: String,
    pub clock_value: i32,
    pub counter_value: i32,
    pub start_time: i32,
}

#[derive(Debug, Default)]
pub struct QuestRuntimeState {
    pub instances: Vec<QuestInstance>,
}

impl QuestRuntimeState {
    pub fn clear(&mut self) {
        self.instances.clear();
    }

    pub fn register_quest_instance(&mut self, instance: QuestInstance) {
        self.delete_quest_instance(instance.index);
        let mut new_instance = instance;
        new_instance.start_time = app_timing_clock::m2_get_time() as i32 / 1000;
        self.instances.push(new_instance);
    }

    pub fn delete_quest_instance(&mut self, index: u32) {
        if let Some(pos) = self.instances.iter().position(|inst| inst.index == index) {
            self.instances.remove(pos);
        }
    }

    pub fn make_quest(&mut self, index: u32) {
        self.delete_quest_instance(index);
        let mut new_instance = QuestInstance::default();
        new_instance.index = index;
        new_instance.start_time = app_timing_clock::m2_get_time() as i32 / 1000;
        self.instances.push(new_instance);
    }

    pub fn get_quest_instance_ptr(&self, index: u32) -> Option<&QuestInstance> {
        self.instances.iter().find(|inst| inst.index == index)
    }

    pub fn get_quest_instance_ptr_mut(&mut self, index: u32) -> Option<&mut QuestInstance> {
        self.instances.iter_mut().find(|inst| inst.index == index)
    }

    pub fn get_quest_instance_by_array_index(&self, array_index: usize) -> Option<&QuestInstance> {
        self.instances.get(array_index)
    }
}

static QUEST_RUNTIME_STATE: OnceLock<Mutex<QuestRuntimeState>> = OnceLock::new();

pub fn get_quest_runtime_state() -> &'static Mutex<QuestRuntimeState> {
    QUEST_RUNTIME_STATE.get_or_init(|| Mutex::new(QuestRuntimeState::default()))
}

#[no_mangle]
pub extern "C" fn m2_py_quest_clear() {
    safe_ffi_call((), || {
        if let Ok(mut state) = get_quest_runtime_state().lock() {
            state.clear();
        }
        if let Ok(mut dialog) = get_ui_quest_dialog().lock() {
            dialog.clear();
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_get_quest_count() -> u32 {
    safe_ffi_call(0, || {
        if let Ok(state) = get_quest_runtime_state().lock() {
            state.instances.len() as u32
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn m2_py_quest_delete_quest(index: u32) {
    safe_ffi_call((), || {
        if let Ok(mut state) = get_quest_runtime_state().lock() {
            state.delete_quest_instance(index);
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_get_quest_name(
    array_index: u32,
    out_name: *mut c_char,
    out_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        if out_name.is_null() || out_len == 0 {
            return false;
        }
        if let Ok(state) = get_quest_runtime_state().lock() {
            if let Some(instance) = state.get_quest_instance_by_array_index(array_index as usize) {
                let bytes = instance.title.as_bytes();
                if bytes.len() >= out_len {
                    return false;
                }
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        bytes.as_ptr() as *const c_char,
                        out_name,
                        bytes.len(),
                    );
                    *out_name.add(bytes.len()) = 0;
                }
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_quest_get_quest_icon_file_name(
    array_index: u32,
    out_name: *mut c_char,
    out_len: usize,
) -> bool {
    safe_ffi_call(false, || {
        if out_name.is_null() || out_len == 0 {
            return false;
        }
        if let Ok(state) = get_quest_runtime_state().lock() {
            if let Some(instance) = state.get_quest_instance_by_array_index(array_index as usize) {
                let bytes = instance.icon_file_name.as_bytes();
                if bytes.len() >= out_len {
                    return false;
                }
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        bytes.as_ptr() as *const c_char,
                        out_name,
                        bytes.len(),
                    );
                    *out_name.add(bytes.len()) = 0;
                }
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_py_quest_get_quest_data(
    index: u32,
    out_title: *mut c_char,
    out_title_len: usize,
    out_counter_name: *mut c_char,
    out_counter_name_len: usize,
    out_counter_value: *mut i32,
) -> bool {
    safe_ffi_call(false, || {
        if out_title.is_null()
            || out_title_len == 0
            || out_counter_name.is_null()
            || out_counter_name_len == 0
            || out_counter_value.is_null()
        {
            return false;
        }

        let state_guard = match get_quest_runtime_state().lock() {
            Ok(guard) => guard,
            Err(_) => return false,
        };

        // Match CPythonQuest::GetQuestInstancePtr which expects the array index, not dwIndex!
        // Wait, python module uses GetQuestInstancePtr which does array index lookup, not ID lookup?
        // Let's check PythonQuest.cpp
        // int CPythonQuest::GetQuestCount()
        // bool CPythonQuest::GetQuestInstancePtr(DWORD dwArrayIndex, SQuestInstance ** ppQuestInstance) { if (dwArrayIndex >= size) return false; ... }
        // questGetQuestData receives index. It calls GetQuestInstancePtr(iIndex). So it's an ARRAY INDEX.
        // GetQuestIndex uses GetQuestInstancePtr(iIndex) and returns pQuestInstance->dwIndex.
        // GetQuestLastTime uses GetQuestInstancePtr(iIndex).
        // Let's implement getting by array index.
        if index as usize >= state_guard.instances.len() {
            return false;
        }

        let instance = &state_guard.instances[index as usize];

        let title_bytes = instance.title.as_bytes();
        if title_bytes.len() >= out_title_len {
            return false;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                title_bytes.as_ptr() as *const c_char,
                out_title,
                title_bytes.len(),
            );
            *out_title.add(title_bytes.len()) = 0;
        }

        let counter_name_bytes = instance.counter_name.as_bytes();
        if counter_name_bytes.len() >= out_counter_name_len {
            return false;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                counter_name_bytes.as_ptr() as *const c_char,
                out_counter_name,
                counter_name_bytes.len(),
            );
            *out_counter_name.add(counter_name_bytes.len()) = 0;

            *out_counter_value = instance.counter_value;
        }

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_quest_get_quest_index(array_index: u32) -> u32 {
    safe_ffi_call(0, || {
        if let Ok(state) = get_quest_runtime_state().lock() {
            if (array_index as usize) < state.instances.len() {
                return state.instances[array_index as usize].index;
            }
        }
        0
    })
}

#[no_mangle]
pub extern "C" fn m2_py_quest_get_quest_last_time(
    array_index: u32,
    out_clock_name: *mut c_char,
    out_clock_name_len: usize,
    out_last_time: *mut i32,
) -> bool {
    safe_ffi_call(false, || {
        if out_clock_name.is_null() || out_clock_name_len == 0 || out_last_time.is_null() {
            return false;
        }

        let state = match get_quest_runtime_state().lock() {
            Ok(guard) => guard,
            Err(_) => return false,
        };

        if array_index as usize >= state.instances.len() {
            return false;
        }

        let instance = &state.instances[array_index as usize];

        let clock_name_bytes = instance.clock_name.as_bytes();
        if clock_name_bytes.len() >= out_clock_name_len {
            return false;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                clock_name_bytes.as_ptr() as *const c_char,
                out_clock_name,
                clock_name_bytes.len(),
            );
            *out_clock_name.add(clock_name_bytes.len()) = 0;
        }

        let mut last_time = 0;
        if instance.clock_value >= 0 {
            let current_sec = app_timing_clock::m2_get_time() as i32 / 1000;
            last_time = (instance.start_time + instance.clock_value) - current_sec;
        }

        unsafe {
            *out_last_time = last_time;
        }

        true
    })
}

#[no_mangle]
pub extern "C" fn m2_py_quest_make_quest(index: u32) {
    safe_ffi_call((), || {
        if let Ok(mut state) = get_quest_runtime_state().lock() {
            state.make_quest(index);
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_quest_title(index: u32, title: *const c_char) {
    safe_ffi_call((), || {
        if title.is_null() {
            return;
        }
        let title_str = unsafe { CStr::from_ptr(title) }
            .to_string_lossy()
            .into_owned();
        if let Ok(mut state) = get_quest_runtime_state().lock() {
            if let Some(inst) = state.get_quest_instance_ptr_mut(index) {
                inst.title = title_str;
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_quest_clock_name(index: u32, name: *const c_char) {
    safe_ffi_call((), || {
        if name.is_null() {
            return;
        }
        let name_str = unsafe { CStr::from_ptr(name) }
            .to_string_lossy()
            .into_owned();
        if let Ok(mut state) = get_quest_runtime_state().lock() {
            if let Some(inst) = state.get_quest_instance_ptr_mut(index) {
                inst.clock_name = name_str;
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_quest_counter_name(index: u32, name: *const c_char) {
    safe_ffi_call((), || {
        if name.is_null() {
            return;
        }
        let name_str = unsafe { CStr::from_ptr(name) }
            .to_string_lossy()
            .into_owned();
        if let Ok(mut state) = get_quest_runtime_state().lock() {
            if let Some(inst) = state.get_quest_instance_ptr_mut(index) {
                inst.counter_name = name_str;
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_quest_clock_value(index: u32, value: i32) {
    safe_ffi_call((), || {
        if let Ok(mut state) = get_quest_runtime_state().lock() {
            if let Some(inst) = state.get_quest_instance_ptr_mut(index) {
                inst.clock_value = value;
                inst.start_time = app_timing_clock::m2_get_time() as i32 / 1000;
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_quest_counter_value(index: u32, value: i32) {
    safe_ffi_call((), || {
        if let Ok(mut state) = get_quest_runtime_state().lock() {
            if let Some(inst) = state.get_quest_instance_ptr_mut(index) {
                inst.counter_value = value;
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_py_quest_set_quest_icon_file_name(index: u32, file_name: *const c_char) {
    safe_ffi_call((), || {
        if file_name.is_null() {
            return;
        }
        let file_name_str = unsafe { CStr::from_ptr(file_name) }
            .to_string_lossy()
            .into_owned();
        if let Ok(mut state) = get_quest_runtime_state().lock() {
            if let Some(inst) = state.get_quest_instance_ptr_mut(index) {
                inst.icon_file_name = file_name_str;
            }
        }
    });
}
