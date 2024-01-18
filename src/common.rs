#![allow(nonstandard_style)]

use crate::dss_capi;
use std::{fmt, error::Error, ffi::{c_char, c_void, CStr, CString}, slice::{from_raw_parts, from_raw_parts_mut}};
use num_complex::Complex;

/// Wrapper for OpenDSS errors
pub struct DSSError {
    pub number: i32,
    pub message: String,
}

impl Error for DSSError {
    fn description(&self,) -> &str {
        &self.message
    }
}

impl fmt::Display for DSSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(#{}) {}", self.number, self.message)
    }
}

impl fmt::Debug for DSSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}


/// Wrapper for DSS Context pointers, including buffers
pub struct DSSContext {
    // Pointer to the context
    pub ctx_ptr: *const c_void,

    // Pointer to the error number
    errorNumberPtr: *mut i32,

    // Pointers for the GR buffers
    CountPtr_PDouble: *mut i32,
    CountPtr_PInteger: *mut i32,
    CountPtr_PByte: *mut i32,

    // TODO: update C-API headers to use more const
    DataPtr_PDouble: *mut *mut f64,
    DataPtr_PInteger: *mut *mut i32,
    DataPtr_PByte: *mut *mut i8,
}

impl Drop for DSSContext {
    fn drop(&mut self) {
        unsafe {
            if self.ctx_ptr != dss_capi::ctx_Get_Prime() {
                dss_capi::ctx_Dispose(self.ctx_ptr);
            }
        }
    }
}
impl DSSContext {
    /// Returns a wrapper to the Prime (default) DSS context.
    /// Currently, the Prime context is the default OpenDSS 
    /// instance created (automatically) when the DSS C-API 
    /// library is loaded.
    pub fn prime() -> Self {
        DSSContext::new(unsafe { dss_capi::ctx_Get_Prime() })
    }

    pub fn new(ctx_ptr: *const c_void) -> Self {
        unsafe {
            dss_capi::ctx_DSS_Start(ctx_ptr, 0);
            
            let mut CountPtr_PDouble: *mut i32 = 0 as *mut i32;
            let mut CountPtr_PPChar: *mut i32 = 0 as *mut i32;
            let mut CountPtr_PInteger: *mut i32 = 0 as *mut i32;
            let mut CountPtr_PByte: *mut i32 = 0 as *mut i32;
            let mut DataPtr_PDouble: *mut *mut f64 = 0 as *mut *mut f64;
            let mut DataPtr_PInteger: *mut *mut i32 = 0 as *mut *mut i32;
            let mut DataPtr_PByte: *mut *mut i8 = 0 as *mut *mut i8;
            let mut DataPtr_PPChar: *mut *mut *mut c_char = 0 as *mut *mut *mut c_char;
        
            dss_capi::ctx_DSS_GetGRPointers(
                ctx_ptr,
                &mut DataPtr_PPChar,
                &mut DataPtr_PDouble,
                &mut DataPtr_PInteger,
                &mut DataPtr_PByte,
                &mut CountPtr_PPChar,
                &mut CountPtr_PDouble,
                &mut CountPtr_PInteger,
                &mut CountPtr_PByte,
            );
            Self {
                ctx_ptr: ctx_ptr,
                errorNumberPtr: dss_capi::ctx_Error_Get_NumberPtr(ctx_ptr),
                CountPtr_PDouble: CountPtr_PDouble,
                CountPtr_PInteger: CountPtr_PInteger,
                CountPtr_PByte: CountPtr_PByte,
                DataPtr_PDouble: DataPtr_PDouble,
                DataPtr_PInteger: DataPtr_PInteger,
                DataPtr_PByte: DataPtr_PByte,
            }
        }
    }

    pub fn DSSError(&self) -> Result<(), DSSError> {
        if unsafe{ *self.errorNumberPtr != 0 } {
            let num = unsafe { *self.errorNumberPtr };
            let msg_ptr = unsafe { dss_capi::ctx_Error_Get_Description(self.ctx_ptr) };
            unsafe { *self.errorNumberPtr = 0 };
            Err(DSSError {
                number: num,
                message: unsafe { CStr::from_ptr(msg_ptr) }.to_string_lossy().into_owned(),
            })
        } else {
            Ok(())
        }
    }

    pub fn PrepareStringArray(&self, value: &[String]) -> (Box::<[CString]>, Box::<[*mut c_char]>) {
        let mut c_strs: Box::<[CString]> = value.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();
        let c_ptrs = c_strs.iter_mut().map(|cs| cs.as_ptr() as *mut c_char).collect();
        (c_strs, c_ptrs)
    }

    pub fn GetStringArray(&self, data: *mut *mut c_char, cnt: [i32; 4]) -> Result<Box::<[String]>, DSSError> {
        self.DSSError()?;
        let res_cnt = cnt[0] as usize;
        let cdata = unsafe { from_raw_parts_mut(data, res_cnt) };
        Ok(unsafe { (*cdata).iter_mut().map(|s| CStr::from_ptr(*s).to_string_lossy().into_owned()).collect() })
    }

    pub fn GetFloat64ArrayGR(&self) -> Result<Box::<[f64]>, DSSError> {
        self.DSSError()?;
        let res_cnt = unsafe { *self.CountPtr_PDouble } as usize;
        let cdata = unsafe { from_raw_parts(*self.DataPtr_PDouble, res_cnt) };
        Ok(cdata.iter().cloned().collect())
    }

    pub fn GetComplexArrayGR(&self) -> Result<Box::<[Complex<f64>]>, DSSError> {
        self.DSSError()?;
        let mut res_cnt = unsafe { *self.CountPtr_PDouble } as usize;
        if res_cnt == 1 {
            res_cnt = 0
        }
        res_cnt /= 2;
        let cdata = unsafe { from_raw_parts((*self.DataPtr_PDouble) as *const Complex<f64>, res_cnt) };
        Ok(cdata.iter().cloned().collect())
    }

    pub fn GetComplexSimpleGR(&self) -> Result<Complex<f64>, DSSError> {
        self.DSSError()?;
        let res_cnt = unsafe { *self.CountPtr_PDouble } as usize;
        // if (err == nil) && (res_cnt != 2) { -- TODO!
        //     err := errors.New("(DSSError) Got invalid data for a complex number.")
        //     return 0.0, err
        // }
        let cdata = unsafe { from_raw_parts(*self.DataPtr_PDouble, res_cnt) };
        Ok(Complex::new(cdata[0], cdata[1]))
    }

    pub fn GetInt32ArrayGR(&self) -> Result<Box::<[i32]>, DSSError> {
        self.DSSError()?;
        let res_cnt = unsafe { *self.CountPtr_PInteger } as usize;
        let cdata = unsafe { from_raw_parts(*self.DataPtr_PInteger, res_cnt) };
        Ok(cdata.iter().cloned().collect())
    }

    pub fn GetInt8ArrayGR(&self) -> Result<Box::<[i8]>, DSSError> {
        self.DSSError()?;
        let res_cnt = unsafe { *self.CountPtr_PByte } as usize;
        let cdata = unsafe { from_raw_parts(*self.DataPtr_PByte, res_cnt) };
        Ok(cdata.iter().cloned().collect())
    }
}

unsafe impl Send for DSSContext {
}

unsafe impl Sync for DSSContext {
}
