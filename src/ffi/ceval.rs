use crate::ffi::code::FreeFunc;
use crate::ffi::object::PyObject;
use crate::ffi::pystate::{PyThreadState, Py_tracefunc};
use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyEval_CallObjectWithKeywords")]
    pub fn PyEval_CallObjectWithKeywords(
        func: *mut PyObject,
        obj: *mut PyObject,
        kwargs: *mut PyObject,
    ) -> *mut PyObject;
}

#[inline]
pub unsafe fn PyEval_CallObject(func: *mut PyObject, arg: *mut PyObject) -> *mut PyObject {
    PyEval_CallObjectWithKeywords(func, arg, ::std::ptr::null_mut())
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyEval_CallFunction")]
    pub fn PyEval_CallFunction(obj: *mut PyObject, format: *const c_char, ...) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyEval_CallMethod")]
    pub fn PyEval_CallMethod(
        obj: *mut PyObject,
        methodname: *const c_char,
        format: *const c_char,
        ...
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyEval_GetBuiltins")]
    pub fn PyEval_GetBuiltins() -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyEval_GetGlobals")]
    pub fn PyEval_GetGlobals() -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyEval_GetLocals")]
    pub fn PyEval_GetLocals() -> *mut PyObject;
    pub fn PyEval_GetFrame() -> *mut crate::ffi::PyFrameObject;
    #[cfg_attr(PyPy, link_name = "PyPy_AddPendingCall")]
    pub fn Py_AddPendingCall(
        func: Option<extern "C" fn(arg1: *mut c_void) -> c_int>,
        arg: *mut c_void,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPy_MakePendingCalls")]
    pub fn Py_MakePendingCalls() -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPy_SetRecursionLimit")]
    pub fn Py_SetRecursionLimit(arg1: c_int);
    #[cfg_attr(PyPy, link_name = "PyPy_GetRecursionLimit")]
    pub fn Py_GetRecursionLimit() -> c_int;
    fn _Py_CheckRecursiveCall(_where: *mut c_char) -> c_int;
}

// TODO: Py_EnterRecursiveCall etc
pub type _PyFrameEvalFunction =
    extern "C" fn(*mut crate::ffi::PyFrameObject, c_int) -> *mut PyObject;

extern "C" {
    pub fn PyEval_GetFuncName(arg1: *mut PyObject) -> *const c_char;
    pub fn PyEval_GetFuncDesc(arg1: *mut PyObject) -> *const c_char;
    pub fn PyEval_GetCallStats(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyEval_EvalFrame(arg1: *mut crate::ffi::PyFrameObject) -> *mut PyObject;
    pub fn _PyEval_EvalFrameDefault(
        arg1: *mut crate::ffi::PyFrameObject,
        exc: c_int,
    ) -> *mut PyObject;
    pub fn _PyEval_RequestCodeExtraIndex(func: FreeFunc) -> c_int;
    pub fn PyEval_EvalFrameEx(f: *mut crate::ffi::PyFrameObject, exc: c_int) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyEval_SaveThread")]
    pub fn PyEval_SaveThread() -> *mut PyThreadState;
    #[cfg_attr(PyPy, link_name = "PyPyEval_RestoreThread")]
    pub fn PyEval_RestoreThread(arg1: *mut PyThreadState);
    pub fn PyEval_SetProfile(trace_func: Py_tracefunc, arg1: *mut PyObject);
    pub fn PyEval_SetTrace(trace_func: Py_tracefunc, arg1: *mut PyObject);
}

#[cfg(py_sys_config = "WITH_THREAD")]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyEval_ThreadsInitialized")]
    pub fn PyEval_ThreadsInitialized() -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyEval_InitThreads")]
    pub fn PyEval_InitThreads();
    pub fn PyEval_AcquireLock();
    pub fn PyEval_ReleaseLock();
    #[cfg_attr(PyPy, link_name = "PyPyEval_AcquireThread")]
    pub fn PyEval_AcquireThread(tstate: *mut PyThreadState);
    #[cfg_attr(PyPy, link_name = "PyPyEval_ReleaseThread")]
    pub fn PyEval_ReleaseThread(tstate: *mut PyThreadState);
    #[cfg(not(Py_3_8))]
    pub fn PyEval_ReInitThreads();
}
