// Copyright (c) 2017 sadikovi
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate libc;
extern crate blas;
extern crate lapack;
extern crate openblas_src;
extern crate rand;

pub mod internal;

use std::ffi::CString;
use std::mem;
use std::panic;
use std::ptr;
use libc::{int32_t, c_double, c_char, size_t};
use internal::{DoubleMatrix, SVD};

// PtrResult to capture and return either valid pointer to a matrix or error message.
// Only one pointer should be set.
#[repr(C)]
pub struct PtrResult {
    ptr: *const DoubleMatrix,
    err: *const c_char
}

// VoidResult to capture exception and return error message. If no exception is thrown, then
// err pointer should be set to NULL.
#[repr(C)]
pub struct VoidResult {
    err: *const c_char
}

// DoubleArray struct represents C array with length included.
#[repr(C)]
pub struct DoubleArray {
    len: int32_t,
    data: *const c_double
}

// SvdResult to capture and return either set of valid matrix pointers or error message.
// Pointers are allowed to set to NULL.
#[repr(C)]
pub struct SvdResult {
    u: *const DoubleMatrix,
    s: *const DoubleMatrix,
    v: *const DoubleMatrix,
    err: *const c_char
}

// Convert error/panic cause into C string
fn err_to_cstr(cause: Box<std::any::Any>) -> *const c_char {
    let err_msg = if cause.is::<String>() {
        format!("{}", *(*cause).downcast_ref::<String>().unwrap())
    } else if cause.is::<&str>() {
        format!("{}", *(*cause).downcast_ref::<&str>().unwrap())
    } else {
        format!("Unknown cause")
    };
    let cstr = CString::new(err_msg).unwrap();
    let cstr_ptr = cstr.as_ptr();
    mem::forget(cstr);
    cstr_ptr
}

// Function to catch panic and return ptr result for matrix
fn try_catch_ptr<F: FnOnce() -> DoubleMatrix + panic::UnwindSafe>(func: F) -> PtrResult {
    match panic::catch_unwind(func) {
        Ok(matrix) => {
            let matrix = Box::new(matrix);
            PtrResult { ptr: Box::into_raw(matrix), err: ptr::null() }
        },
        Err(cause) => {
            PtrResult { ptr: ptr::null(), err: err_to_cstr(cause) }
        }
    }
}

// Function to catch panic and return void result
fn try_catch_void<F: FnOnce() -> () + panic::UnwindSafe>(func: F) -> VoidResult {
    match panic::catch_unwind(func) {
        Ok(_) => VoidResult { err: ptr::null() },
        Err(cause) => VoidResult { err: err_to_cstr(cause) }
    }
}

// Function to catch panic and return svd result
fn try_catch_svd<F: FnOnce() -> SVD + panic::UnwindSafe>(func: F) -> SvdResult {
    match panic::catch_unwind(func) {
        Ok(svd) => {
            // convert matrices into raw pointers
            let u_ptr = match svd.u {
                Some(matrix) => Box::into_raw(Box::new(matrix)),
                None => ptr::null()
            };
            let s_ptr = Box::into_raw(Box::new(svd.s));
            let v_ptr = match svd.v {
                Some(matrix) => Box::into_raw(Box::new(matrix)),
                None => ptr::null()
            };

            SvdResult {
                u: u_ptr,
                s: s_ptr,
                v: v_ptr,
                err: ptr::null()
            }
        },
        Err(cause) => {
            SvdResult {
                u: ptr::null(),
                s: ptr::null(),
                v: ptr::null(),
                err: err_to_cstr(cause)
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn alloc_from_array(
    rows: int32_t,
    cols: int32_t,
    len: size_t,
    ptr: *mut c_double
) -> PtrResult
{
    let vec = unsafe { Vec::from_raw_parts(ptr, len, len) };
    try_catch_ptr(|| DoubleMatrix::new(rows as usize, cols as usize, vec))
}

#[no_mangle]
pub extern "C" fn alloc_rand(rows: int32_t, cols: int32_t) -> PtrResult {
    try_catch_ptr(|| DoubleMatrix::new_random(rows as usize, cols as usize))
}

#[no_mangle]
pub extern "C" fn alloc_zeros(rows: int32_t, cols: int32_t) -> PtrResult {
    try_catch_ptr(|| DoubleMatrix::zeros(rows as usize, cols as usize))
}

#[no_mangle]
pub extern "C" fn alloc_ones(rows: int32_t, cols: int32_t) -> PtrResult {
    try_catch_ptr(|| DoubleMatrix::ones(rows as usize, cols as usize))
}

#[no_mangle]
pub extern "C" fn alloc_identity(rows: int32_t, cols: int32_t) -> PtrResult {
    try_catch_ptr(|| DoubleMatrix::identity(rows as usize, cols as usize))
}

#[no_mangle]
pub extern "C" fn matrix_rows(ptr: *const DoubleMatrix) -> int32_t {
    unsafe { (*ptr).rows() as int32_t }
}

#[no_mangle]
pub extern "C" fn matrix_cols(ptr: *const DoubleMatrix) -> int32_t {
    unsafe { (*ptr).cols() as int32_t }
}

#[no_mangle]
pub extern "C" fn matrix_data_array(ptr: *const DoubleMatrix) -> DoubleArray {
    // TODO: Add check for length at most being i32 max value, currently check exists in Java
    let arr = unsafe { (*ptr).data() };
    DoubleArray { len: arr.len() as i32, data: arr.as_ptr() }
}

#[no_mangle]
pub extern "C" fn matrix_pretty_string(ptr: *const DoubleMatrix) -> *const c_char {
    let matrix_str = unsafe { (*ptr).to_string() };
    let cstr = CString::new(matrix_str).unwrap();
    let ptr = cstr.as_ptr();
    mem::forget(cstr);
    ptr
}

#[no_mangle]
pub extern "C" fn matrix_dealloc(ptr: *mut DoubleMatrix) {
    unsafe { drop(Box::from_raw(ptr)); }
}

#[no_mangle]
pub extern "C" fn matrix_add_scalar(
    ptr: *const DoubleMatrix,
    scalar: c_double
) -> PtrResult
{
    try_catch_ptr(|| unsafe { (*ptr).add_scalar(scalar) })
}

#[no_mangle]
pub extern "C" fn matrix_add_matrix(
    ptr: *const DoubleMatrix,
    aptr: *const DoubleMatrix
) -> PtrResult
{
    let this = unsafe { &(*ptr) };
    let that = unsafe { &(*aptr) };
    try_catch_ptr(|| this.add_matrix(that))
}

#[no_mangle]
pub extern "C" fn matrix_add_in_place_scalar(
    ptr: *mut DoubleMatrix,
    scalar: c_double
) -> VoidResult
{
    try_catch_void(|| unsafe { (*ptr).add_scalar_mut(scalar); })
}

#[no_mangle]
pub extern "C" fn matrix_add_in_place_matrix(
    ptr: *mut DoubleMatrix,
    aptr: *const DoubleMatrix
) -> VoidResult
{
    try_catch_void(|| {
        let this = unsafe { &mut (*ptr) };
        let that = unsafe { &(*aptr) };
        this.add_matrix_mut(that);
    })
}

#[no_mangle]
pub extern "C" fn matrix_sub_scalar(
    ptr: *const DoubleMatrix,
    scalar: c_double
) -> PtrResult
{
    try_catch_ptr(|| unsafe { (*ptr).sub_scalar(scalar) })
}

#[no_mangle]
pub extern "C" fn matrix_sub_matrix(
    ptr: *const DoubleMatrix,
    aptr: *const DoubleMatrix
) -> PtrResult
{
    let this = unsafe { &(*ptr) };
    let that = unsafe { &(*aptr) };
    try_catch_ptr(|| this.sub_matrix(that))
}

#[no_mangle]
pub extern "C" fn matrix_sub_in_place_scalar(
    ptr: *mut DoubleMatrix,
    scalar: c_double
) -> VoidResult
{
    // TODO: check that negation is correct for scalar
    try_catch_void(|| unsafe { (*ptr).sub_scalar_mut(scalar); })
}

#[no_mangle]
pub extern "C" fn matrix_sub_in_place_matrix(
    ptr: *mut DoubleMatrix,
    aptr: *const DoubleMatrix
) -> VoidResult
{
    try_catch_void(|| {
        let this = unsafe { &mut (*ptr) };
        let that = unsafe { &(*aptr) };
        this.sub_matrix_mut(that);
    })
}

#[no_mangle]
pub extern "C" fn matrix_mul_scalar(
    ptr: *const DoubleMatrix,
    scalar: c_double
) -> PtrResult
{
    let this = unsafe { &(*ptr) };
    try_catch_ptr(|| this.mul_scalar(scalar))
}

#[no_mangle]
pub extern "C" fn matrix_mul_matrix(
    ptr: *const DoubleMatrix,
    aptr: *const DoubleMatrix
) -> PtrResult
{
    let this = unsafe { &(*ptr) };
    let that = unsafe { &(*aptr) };
    try_catch_ptr(|| this.mul_matrix(that))
}

#[no_mangle]
pub extern "C" fn matrix_mul_in_place_scalar(
    ptr: *mut DoubleMatrix,
    scalar: c_double
) -> VoidResult
{
    try_catch_void(|| unsafe { (*ptr).mul_scalar_mut(scalar); })
}

#[no_mangle]
pub extern "C" fn matrix_mul_in_place_matrix(
    ptr: *mut DoubleMatrix,
    aptr: *const DoubleMatrix
) -> VoidResult
{
    try_catch_void(|| {
        let this = unsafe { &mut (*ptr) };
        let that = unsafe { &(*aptr) };
        this.mul_matrix_mut(that);
    })
}

#[no_mangle]
pub extern "C" fn matrix_div_scalar(
    ptr: *const DoubleMatrix,
    scalar: c_double
) -> PtrResult
{
    let this = unsafe { &(*ptr) };
    try_catch_ptr(|| this.div_scalar(scalar))
}

#[no_mangle]
pub extern "C" fn matrix_div_matrix(
    ptr: *const DoubleMatrix,
    aptr: *const DoubleMatrix
) -> PtrResult
{
    let this = unsafe { &(*ptr) };
    let that = unsafe { &(*aptr) };
    try_catch_ptr(|| this.div_matrix(that))
}

#[no_mangle]
pub extern "C" fn matrix_div_in_place_scalar(
    ptr: *mut DoubleMatrix,
    scalar: c_double
) -> VoidResult
{
    try_catch_void(|| unsafe { (*ptr).div_scalar_mut(scalar); })
}

#[no_mangle]
pub extern "C" fn matrix_div_in_place_matrix(
    ptr: *mut DoubleMatrix,
    aptr: *const DoubleMatrix
) -> VoidResult
{
    try_catch_void(|| {
        let this = unsafe { &mut (*ptr) };
        let that = unsafe { &(*aptr) };
        this.div_matrix_mut(that);
    })
}

#[no_mangle]
pub extern "C" fn matrix_mmul_matrix(
    ptr: *const DoubleMatrix,
    aptr: *const DoubleMatrix
) -> PtrResult
{
    let this = unsafe { &(*ptr) };
    let that = unsafe { &(*aptr) };
    try_catch_ptr(|| this.mmul(that))
}

#[no_mangle]
pub extern "C" fn matrix_mmul_in_place_matrix(
    ptr: *mut DoubleMatrix,
    aptr: *const DoubleMatrix
) -> VoidResult
{
    try_catch_void(|| {
        let this = unsafe { &mut (*ptr) };
        let that = unsafe { &(*aptr) };
        this.mmul_assign(that);
    })
}

#[no_mangle]
pub extern "C" fn matrix_column_mins(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.column_mins());
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_column_maxs(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.column_maxs());
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_column_means(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.column_means());
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_column_sums(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.column_sums());
    Box::into_raw(matrix)
}


#[no_mangle]
pub extern "C" fn matrix_row_mins(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.row_mins());
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_row_maxs(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.row_maxs());
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_row_means(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.row_means());
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_row_sums(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.row_sums());
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_min(ptr: *const DoubleMatrix) -> c_double {
    unsafe { (*ptr).min() }
}

#[no_mangle]
pub extern "C" fn matrix_max(ptr: *const DoubleMatrix) -> c_double {
    unsafe { (*ptr).max() }
}

#[no_mangle]
pub extern "C" fn matrix_sum(ptr: *const DoubleMatrix) -> c_double {
    unsafe { (*ptr).sum() }
}

#[no_mangle]
pub extern "C" fn matrix_norm1(ptr: *const DoubleMatrix) -> c_double {
    unsafe { (*ptr).norm1() }
}

#[no_mangle]
pub extern "C" fn matrix_norm2(ptr: *const DoubleMatrix) -> c_double {
    unsafe { (*ptr).norm2() }
}

#[no_mangle]
pub extern "C" fn matrix_transpose(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.transpose());
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_diag(ptr: *const DoubleMatrix) -> PtrResult {
    let this = unsafe { &(*ptr) };
    // return diagonal as column vector similar to jblas
    try_catch_ptr(|| this.diag())
}

#[no_mangle]
pub extern "C" fn matrix_abs(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.abs());
    Box::into_raw(matrix)
}

// == Singular value decomposition methods ==

#[no_mangle]
pub extern "C" fn matrix_full_svd(ptr: *const DoubleMatrix) -> SvdResult {
    let this = unsafe { &(*ptr) };
    try_catch_svd(|| this.full_svd())
}
