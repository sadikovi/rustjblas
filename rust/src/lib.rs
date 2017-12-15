extern crate libc;
extern crate nalgebra;
extern crate rand;

pub mod matrix;

use std::ffi::CString;
use std::mem;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use libc::{int32_t, c_double, c_char, size_t};
use matrix::DoubleMatrix;

#[no_mangle]
pub extern "C" fn alloc_from_array(
    rows: int32_t,
    cols: int32_t,
    len: size_t,
    ptr: *mut c_double
) -> *const DoubleMatrix
{
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
pub extern "C" fn matrix_data_len(ptr: *const DoubleMatrix) -> int32_t {
    unsafe { (*ptr).data.len() as int32_t }
}

#[no_mangle]
pub extern "C" fn matrix_data_array(ptr: *const DoubleMatrix) -> *const c_double {
    let arr = unsafe { (*ptr).data.as_slice() };
    arr.as_ptr()
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
) -> *const DoubleMatrix
{
    let matrix = Box::new(unsafe { (*ptr).add_scalar(scalar) });
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_add_matrix(
    ptr: *const DoubleMatrix,
    aptr: *const DoubleMatrix
) -> *const DoubleMatrix
{
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
pub extern "C" fn matrix_sub_scalar(
    ptr: *const DoubleMatrix,
    scalar: c_double
) -> *const DoubleMatrix
{
    // TODO: check that negation is correct for scalar
    let matrix = Box::new(unsafe { (*ptr).add_scalar(-scalar) });
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_sub_matrix(
    ptr: *const DoubleMatrix,
    aptr: *const DoubleMatrix
) -> *const DoubleMatrix
{
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
pub extern "C" fn matrix_mul_scalar(
    ptr: *const DoubleMatrix,
    scalar: c_double
) -> *const DoubleMatrix
{
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this * scalar);
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_mul_matrix(
    ptr: *const DoubleMatrix,
    aptr: *const DoubleMatrix
) -> *const DoubleMatrix
{
    let this = unsafe { &(*ptr) };
    let that = unsafe { &(*aptr) };
    let matrix = Box::new(this.component_mul(that));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_mul_in_place_scalar(ptr: *mut DoubleMatrix, scalar: c_double) {
    unsafe { (*ptr).mul_assign(scalar) };
}

#[no_mangle]
pub extern "C" fn matrix_mul_in_place_matrix(ptr: *mut DoubleMatrix, aptr: *const DoubleMatrix) {
    let this = unsafe { &mut (*ptr) };
    let that = unsafe { &(*aptr) };
    this.component_mul_assign(that);
}

#[no_mangle]
pub extern "C" fn matrix_div_scalar(
    ptr: *const DoubleMatrix,
    scalar: c_double
) -> *const DoubleMatrix
{
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this / scalar);
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_div_matrix(
    ptr: *const DoubleMatrix,
    aptr: *const DoubleMatrix
) -> *const DoubleMatrix
{
    let this = unsafe { &(*ptr) };
    let that = unsafe { &(*aptr) };
    let matrix = Box::new(this.component_div(that));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_div_in_place_scalar(ptr: *mut DoubleMatrix, scalar: c_double) {
    unsafe { (*ptr).div_assign(scalar) };
}

#[no_mangle]
pub extern "C" fn matrix_div_in_place_matrix(ptr: *mut DoubleMatrix, aptr: *const DoubleMatrix) {
    let this = unsafe { &mut (*ptr) };
    let that = unsafe { &(*aptr) };
    this.component_div_assign(that);
}

#[no_mangle]
pub extern "C" fn matrix_column_mins(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(matrix::column_mins(this));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_column_maxs(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(matrix::column_maxs(this));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_column_means(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(matrix::column_means(this));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_column_sums(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(matrix::column_sums(this));
    Box::into_raw(matrix)
}


#[no_mangle]
pub extern "C" fn matrix_row_mins(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(matrix::row_mins(this));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_row_maxs(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(matrix::row_maxs(this));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_row_means(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(matrix::row_means(this));
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_row_sums(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(matrix::row_sums(this));
    Box::into_raw(matrix)
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

#[no_mangle]
pub extern "C" fn matrix_transpose(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(this.transpose());
    Box::into_raw(matrix)
}

#[no_mangle]
pub extern "C" fn matrix_diag(ptr: *const DoubleMatrix) -> *const DoubleMatrix {
    let this = unsafe { &(*ptr) };
    let matrix = Box::new(DoubleMatrix::from_diagonal(&this.diagonal()));
    Box::into_raw(matrix)
}
