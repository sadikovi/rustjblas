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

//! Module that defines bindings and wrapper methods for Lanczos SVD.

use libc::{c_char, c_double, c_int};

#[allow(non_camel_case_types)]
type PROPACK_EXTERN = Option<
    extern "C" fn(*const c_char, *const c_int, *const c_int, *const c_double, *mut c_double, *const c_double, *mut c_int)
>;

#[no_mangle]
pub extern "C" fn dense_matmul(
    transa: *const c_char,
    m: *const c_int,
    n: *const c_int,
    x: *const c_double,
    y: *mut c_double,
    dparm: *const c_double,
    _iparm: *mut c_int
) {
    unsafe {
        // `dgemv` function to call as aprod
        dgemv_(transa, m, n, &1f64, dparm, m, x, &1i32, &0f64, y, &1i32);
    }
}

extern "C" {
    // from blas-sys, redefine so we don't need to import it
    fn dgemv_(
        trans: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_double,
        a: *const c_double,
        lda: *const c_int,
        x: *const c_double,
        incx: *const c_int,
        beta: *const c_double,
        y: *mut c_double,
        incy: *const c_int,
    );

    // DLANSVD: Compute the leading singular triplets of a large and sparse matrix by Lanczos
    // bidiagonalization with partial reorthogonalization
    fn dlansvd_(
        jobu: *const c_char,
        jobv: *const c_char,
        m: *const c_int,
        n: *const c_int,
        k: *const c_int,
        kmax: *const c_int,
        aprod: PROPACK_EXTERN,
        u: *mut c_double,
        ldu: *const c_int,
        sigma: *mut c_double,
        bnd: *mut c_double,
        v: *mut c_double,
        ldv: *const c_int,
        tolin: *const c_double,
        work: *mut c_double,
        lwork: *const c_int,
        iwork: *mut c_int,
        liwork: *const c_int,
        doption: *const c_double,
        ioption: *const c_int,
        info: *mut c_int,
        dparm: *const c_double,
        iparm: *mut c_int
    );

    // DLANSVD_IRL: Compute the leading singular triplets of a large and sparse matrix A by
    // implicitly restarted Lanczos bidiagonalization with partial reorthogonalization
    fn dlansvd_irl_(
        which: *const c_char,
        jobu: *const c_char,
        jobv: *const c_char,
        m: *const c_int,
        n: *const c_int,
        dim: *const c_int,
        p: *const c_int,
        neig: *const c_int,
        maxiter: *const c_int,
        aprod: PROPACK_EXTERN,
        u: *mut c_double,
        ldu: *const c_int,
        sigma: *mut c_double,
        bnd: *mut c_double,
        v: *mut c_double,
        ldv: *const c_int,
        tolin: *const c_double,
        work: *mut c_double,
        lwork: *const c_int,
        iwork: *mut c_int,
        liwork: *const c_int,
        doption: *const c_double,
        ioption: *const c_int,
        info: *mut c_int,
        dparm: *const c_double,
        iparm: *mut c_int
    );
}

#[inline]
pub unsafe fn dlansvd(
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    k: i32,
    kmax: i32,
    u: &mut [f64],
    ldu: i32,
    sigma: &mut [f64],
    bnd: &mut [f64],
    v: &mut [f64],
    ldv: i32,
    tolin: f64,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut [i32],
    liwork: i32,
    doption: &[f64],
    ioption: &[i32],
    info: &mut i32,
    dparm: &[f64],
    iparm: &mut [i32]
) {
    dlansvd_(
        &(jobu as c_char),
        &(jobv as c_char),
        &m,
        &n,
        &k,
        &kmax,
        Some(dense_matmul),
        u.as_mut_ptr(),
        &ldu,
        sigma.as_mut_ptr(),
        bnd.as_mut_ptr(),
        v.as_mut_ptr(),
        &ldv,
        &tolin,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        doption.as_ptr(),
        ioption.as_ptr(),
        info,
        dparm.as_ptr(),
        iparm.as_mut_ptr()
    )
}

#[inline]
pub unsafe fn dlansvd_irl(
    which: u8,
    jobu: u8,
    jobv: u8,
    m: i32,
    n: i32,
    dim: i32,
    p: i32,
    neig: i32,
    maxiter: i32,
    u: &mut [f64],
    ldu: i32,
    sigma: &mut [f64],
    bnd: &mut [f64],
    v: &mut [f64],
    ldv: i32,
    tolin: f64,
    work: &mut [f64],
    lwork: i32,
    iwork: &mut[i32],
    liwork: i32,
    doption: &[f64],
    ioption: &[i32],
    info: &mut i32,
    dparm: &[f64],
    iparm: &mut [i32]
) {
    dlansvd_irl_(
        &(which as c_char),
        &(jobu as c_char),
        &(jobv as c_char),
        &m,
        &n,
        &dim,
        &p,
        &neig,
        &maxiter,
        Some(dense_matmul),
        u.as_mut_ptr(),
        &ldu,
        sigma.as_mut_ptr(),
        bnd.as_mut_ptr(),
        v.as_mut_ptr(),
        &ldv,
        &tolin,
        work.as_mut_ptr(),
        &lwork,
        iwork.as_mut_ptr(),
        &liwork,
        doption.as_ptr(),
        ioption.as_ptr(),
        info,
        dparm.as_ptr(),
        iparm.as_mut_ptr()
    );
}
