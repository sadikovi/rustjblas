extern crate libc;

pub mod matrix;

use std::mem;
use std::ffi::CString;
use libc::{boolean_t, int32_t, c_double, c_char, size_t};
use matrix::DoubleMatrix;

#[no_mangle]
pub extern "C" fn double_matrix_alloc_new(rows: int32_t, cols: int32_t, len: size_t, ptr: *mut c_double) -> *const DoubleMatrix {
    assert!(rows >= 0 && cols >= 0);
    let vec = unsafe { Vec::from_raw_parts(ptr, len, len) };
    println!("rows = {}, cols = {}, vec = {:?}", rows, cols, vec);
    let matrix = Box::new(DoubleMatrix::new(rows as usize, cols as usize, vec));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn double_matrix_alloc_rand(rows: int32_t, cols: int32_t) -> *const DoubleMatrix {
    println!("rand, rows = {}, cols = {}", rows, cols);
    let matrix = Box::new(DoubleMatrix::rand(rows as usize, cols as usize));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn double_matrix_alloc_zeros(rows: int32_t, cols: int32_t) -> *const DoubleMatrix {
    println!("zeros, rows = {}, cols = {}", rows, cols);
    let matrix = Box::new(DoubleMatrix::zeros(rows as usize, cols as usize));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn double_matrix_alloc_ones(rows: int32_t, cols: int32_t) -> *const DoubleMatrix {
    println!("ones, rows = {}, cols = {}", rows, cols);
    let matrix = Box::new(DoubleMatrix::ones(rows as usize, cols as usize));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn double_matrix_instance_rows(ptr: *const DoubleMatrix) -> int32_t {
    unsafe {
        (*ptr).rows() as int32_t
    }
}

#[no_mangle]
pub extern "C" fn double_matrix_instance_cols(ptr: *const DoubleMatrix) -> int32_t {
    unsafe {
        (*ptr).cols() as int32_t
    }
}

#[no_mangle]
pub extern "C" fn double_matrix_instance_show(ptr: *const DoubleMatrix, truncate: boolean_t) {
    let truncate = truncate == 1;
    unsafe {
        (*ptr).show(truncate);
    }
}

#[no_mangle]
pub extern "C" fn double_matrix_instance_tostring(ptr: *const DoubleMatrix) -> *const c_char {
    let matrix_str = unsafe { (*ptr).to_string() };
    let res = CString::new(matrix_str).unwrap();
    let ptr = res.as_ptr();
    mem::forget(res);
    ptr
}

#[no_mangle]
pub extern "C" fn double_matrix_instance_dealloc(ptr: *mut DoubleMatrix) {
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
