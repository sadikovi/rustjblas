extern crate libc;
extern crate nalgebra;
extern crate rand;

pub mod dmatrix;

use std::ffi::CString;
use std::mem;
use std::ops::{AddAssign, SubAssign};
use libc::{int32_t, c_double, c_char, size_t};
use dmatrix::DoubleMatrix;

#[no_mangle]
pub extern "C" fn alloc_from_array(rows: int32_t, cols: int32_t, len: size_t, ptr: *mut c_double) -> *const DoubleMatrix {
    let vec = unsafe { Vec::from_raw_parts(ptr, len, len) };
    let matrix = Box::new(DoubleMatrix::from_column_slice(rows as usize, cols as usize, &vec));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn alloc_rand(rows: int32_t, cols: int32_t) -> *const DoubleMatrix {
    let matrix = Box::new(DoubleMatrix::new_random(rows as usize, cols as usize));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn alloc_zeros(rows: int32_t, cols: int32_t) -> *const DoubleMatrix {
    let matrix = Box::new(DoubleMatrix::zeros(rows as usize, cols as usize));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn alloc_ones(rows: int32_t, cols: int32_t) -> *const DoubleMatrix {
    let matrix = Box::new(DoubleMatrix::from_element(rows as usize, cols as usize, 1f64));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_rows(ptr: *const DoubleMatrix) -> int32_t {
    unsafe { (*ptr).nrows() as int32_t }
}

#[no_mangle]
pub extern "C" fn matrix_cols(ptr: *const DoubleMatrix) -> int32_t {
    unsafe { (*ptr).ncols() as int32_t }
}

#[no_mangle]
pub extern "C" fn matrix_pretty_string(ptr: *const DoubleMatrix) -> *const c_char {
    let matrix_str = unsafe { (*ptr).to_string() };
    let res = CString::new(matrix_str).unwrap();
    let ptr = res.as_ptr();
    mem::forget(res);
    ptr
}

#[no_mangle]
pub extern "C" fn matrix_dealloc(ptr: *mut DoubleMatrix) {
    unsafe { drop(Box::from_raw(ptr)); }
}

#[no_mangle]
pub extern "C" fn matrix_add_scalar(ptr: *const DoubleMatrix, scalar: c_double) -> *const DoubleMatrix {
    let matrix = Box::new(unsafe { (*ptr).add_scalar(scalar) });
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_add_matrix(ptr: *const DoubleMatrix, aptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let that = unsafe { &(*aptr) };
    let matrix = Box::new(this + that);
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_add_in_place_scalar(ptr: *mut DoubleMatrix, scalar: c_double) {
    unsafe { (*ptr).add_scalar_mut(scalar) };
}

#[no_mangle]
pub extern "C" fn matrix_add_in_place_matrix(ptr: *mut DoubleMatrix, aptr: *const DoubleMatrix) {
    let this = unsafe { &mut (*ptr) };
    let that = unsafe { &(*aptr) };
    this.add_assign(that);
}

#[no_mangle]
pub extern "C" fn matrix_sub_scalar(ptr: *const DoubleMatrix, scalar: c_double) -> *const DoubleMatrix {
    // TODO: check that negation is correct for scalar
    let matrix = Box::new(unsafe { (*ptr).add_scalar(-scalar) });
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_sub_matrix(ptr: *const DoubleMatrix, aptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let that = unsafe { &(*aptr) };
    let matrix = Box::new(this - that);
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_sub_in_place_scalar(ptr: *mut DoubleMatrix, scalar: c_double) {
    // TODO: check that negation is correct for scalar
    unsafe { (*ptr).add_scalar_mut(-scalar) };
}

#[no_mangle]
pub extern "C" fn matrix_sub_in_place_matrix(ptr: *mut DoubleMatrix, aptr: *const DoubleMatrix) {
    let this = unsafe { &mut (*ptr) };
    let that = unsafe { &(*aptr) };
    this.sub_assign(that);
}

#[no_mangle]
pub extern "C" fn matrix_min(ptr: *const DoubleMatrix) -> c_double {
    let iter = unsafe { (*ptr).iter() };
    iter.fold(std::f64::NAN, |left, right| left.min(*right))
}

#[no_mangle]
pub extern "C" fn matrix_max(ptr: *const DoubleMatrix) -> c_double {
    let iter = unsafe { (*ptr).iter() };
    iter.fold(std::f64::NAN, |left, right| left.max(*right))
}

#[no_mangle]
pub extern "C" fn matrix_sum(ptr: *const DoubleMatrix) -> c_double {
    let iter = unsafe { (*ptr).iter() };
    iter.sum()
}

#[no_mangle]
pub extern "C" fn matrix_norm1(ptr: *const DoubleMatrix) -> c_double {
    unsafe { (*ptr).norm_squared() }
}

#[no_mangle]
pub extern "C" fn matrix_norm2(ptr: *const DoubleMatrix) -> c_double {
    unsafe { (*ptr).norm() }
}
