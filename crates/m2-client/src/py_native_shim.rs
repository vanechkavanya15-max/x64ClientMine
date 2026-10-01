use pyo3::ffi::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

/// 1:1 Implementation of Py_InitModule from ScriptLib/PythonUtils.h
///
/// Registers a Python module globally via C-API and returns a borrowed reference.
/// # Safety
/// The caller must ensure that `name` is a valid null-terminated C string
/// and `methods` is a valid pointer to a null-terminated array of `PyMethodDef`.
pub unsafe fn py_init_module(name: *const c_char, methods: *mut PyMethodDef) -> *mut PyObject {
    // Note: C++ dynamically allocates PyModuleDef because PyModule_Create expects
    // the pointer to remain valid for the lifetime of the module.
    let module_def = Box::new(PyModuleDef {
        m_base: PyModuleDef_HEAD_INIT,
        m_name: name,
        m_doc: ptr::null(),
        m_size: -1,
        m_methods: methods,
        m_slots: ptr::null_mut(),
        m_traverse: None,
        m_clear: None,
        m_free: None,
    });

    let module_def_ptr = Box::into_raw(module_def);

    if module_def_ptr.is_null() {
        return PyErr_NoMemory();
    }

    let module = PyModule_Create(module_def_ptr);
    if module.is_null() {
        let _ = Box::from_raw(module_def_ptr); // Free
        return ptr::null_mut();
    }

    let modules = PyImport_GetModuleDict();
    if PyDict_SetItemString(modules, name, module) < 0 {
        Py_DecRef(module);
        return ptr::null_mut();
    }

    Py_DecRef(module);
    
    // Return borrowed reference matching python < 3 behavior
    PyDict_GetItemString(modules, name)
}

// --- Python 2 / Python 3 Compatibility Utilities ---

/// Equivalent to Python 2's PyString_Check.
/// Checks if the object is either a bytes or unicode object.
pub unsafe fn py_string_check(obj: *mut PyObject) -> bool {
    if obj.is_null() {
        return false;
    }
    PyUnicode_Check(obj) != 0 || PyBytes_Check(obj) != 0
}

/// Equivalent to Python 2's PyString_AsString.
/// Returns a UTF-8 string pointer if it's a unicode object, or the underlying bytes.
/// Sets TypeError if the object is neither or is null.
pub unsafe fn py_string_as_string(obj: *mut PyObject) -> *mut c_char {
    if obj.is_null() {
        let msg = CString::new("NULL Python object").unwrap();
        PyErr_SetString(PyExc_TypeError, msg.as_ptr());
        return ptr::null_mut();
    }

    if PyUnicode_Check(obj) != 0 {
        return PyUnicode_AsUTF8AndSize(obj, ptr::null_mut()) as *mut c_char;
    }

    if PyBytes_Check(obj) != 0 {
        return PyBytes_AsString(obj);
    }

    let msg = CString::new("Expected str or bytes object").unwrap();
    PyErr_SetString(PyExc_TypeError, msg.as_ptr());
    ptr::null_mut()
}

/// Equivalent to Python 2's PyString_AS_STRING macro.
pub unsafe fn py_string_as_string_macro(obj: *mut PyObject) -> *mut c_char {
    py_string_as_string(obj)
}

/// Equivalent to Python 2's PyString_FromStringAndSize.
/// Creates a new bytes object in Python 3.
pub unsafe fn py_string_from_string_and_size(str: *const c_char, size: isize) -> *mut PyObject {
    PyBytes_FromStringAndSize(str, size)
}

// Map python 2 names to python 3 names matching PythonUtils.h behavior
#[inline(always)]
pub unsafe fn py_int_check(obj: *mut PyObject) -> bool {
    PyLong_Check(obj) != 0
}

#[inline(always)]
pub unsafe fn py_int_as_long(obj: *mut PyObject) -> std::os::raw::c_long {
    PyLong_AsLong(obj)
}

#[inline(always)]
pub unsafe fn py_int_from_long(v: std::os::raw::c_long) -> *mut PyObject {
    PyLong_FromLong(v)
}

#[inline(always)]
pub unsafe fn py_string_from_string(str: *const c_char) -> *mut PyObject {
    PyUnicode_FromString(str)
}

#[inline(always)]
pub unsafe fn py_string_intern_from_string(str: *const c_char) -> *mut PyObject {
    PyUnicode_InternFromString(str)
}