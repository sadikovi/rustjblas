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

#include "com_github_sadikovi_rustjblas_DoubleMatrix.h"
#include "com_github_sadikovi_rustjblas_DoubleMatrix_SvdResult.h"
#include "rust_format.h"

extern "C" {

  // generic exception class that we expect to be thrown
  const char *EXCEPTION_CLASS = "com/github/sadikovi/rustjblas/OperationException";

  // Get matrix pointer for the current instance
  void* get_matrix_pointer(JNIEnv *env, jobject obj) {
    jclass clazz = env->GetObjectClass(obj);
    jfieldID fid = env->GetFieldID(clazz, "pointer", "J");
    jlong ptr = env->GetLongField(obj, fid);
    return (void*) ptr;
  }

  // Throw exception with provided message
  void throw_exception(JNIEnv *env, const char *message) {
    jclass clazz = env->FindClass(EXCEPTION_CLASS);
    env->ThrowNew(clazz, message);
  }

  /* == Bindings == */

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    alloc_from_array
   * Signature: (II[D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1from_1array(
      JNIEnv *env, jclass clazz, jint rows, jint cols, jdoubleArray data) {
    jsize len = env->GetArrayLength(data);
    // always copy elements from java heap
    jdouble *body = env->GetDoubleArrayElements(data, 0);
    PtrResult res = alloc_from_array(rows, cols, len, body);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    alloc_rand
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1rand(
      JNIEnv *env, jclass clazz, jint rows, jint cols) {
    PtrResult res = alloc_rand(rows, cols);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    alloc_zeros
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1zeros(
      JNIEnv *env, jclass clazz, jint rows, jint cols) {
    PtrResult res = alloc_zeros(rows, cols);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    alloc_ones
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1ones(
      JNIEnv *env, jclass clazz, jint rows, jint cols) {
    PtrResult res = alloc_ones(rows, cols);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    alloc_identity
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1identity(
      JNIEnv *env, jclass clazz, jint rows, jint cols) {
    PtrResult res = alloc_identity(rows, cols);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_rows
   * Signature: ()I
   */
  JNIEXPORT jint JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1rows(
      JNIEnv *env, jobject obj) {
    return matrix_rows(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_cols
   * Signature: ()I
   */
  JNIEXPORT jint JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1cols(
      JNIEnv *env, jobject obj) {
    return matrix_cols(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_data_array
   * Signature: ()[D
   */
  JNIEXPORT jdoubleArray JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1data_1array(
      JNIEnv *env, jobject obj) {
    void *ptr = get_matrix_pointer(env, obj);
    DoubleArray arr = matrix_data_array(ptr);
    jdoubleArray result = env->NewDoubleArray(arr.len);
    env->SetDoubleArrayRegion(result, 0, arr.len, arr.data);
    return result;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_pretty_string
   * Signature: (Z)Ljava/lang/String;
   */
  JNIEXPORT jstring JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1pretty_1string(
      JNIEnv *env, jobject obj) {
    const char *cstr = matrix_pretty_string(get_matrix_pointer(env, obj));
    jstring result = env->NewStringUTF(cstr);
    return result;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_dealloc
   * Signature: ()V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1dealloc(
      JNIEnv *env, jobject obj) {
    matrix_dealloc(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_add_scalar
   * Signature: (D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    PtrResult res = matrix_add_scalar(get_matrix_pointer(env, obj), scalar);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_add_matrix
   * Signature: (J)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    PtrResult res = matrix_add_matrix(get_matrix_pointer(env, obj), (void*) aptr);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_add_in_place_scalar
   * Signature: (D)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1in_1place_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    VoidResult res = matrix_add_in_place_scalar(get_matrix_pointer(env, obj), scalar);
    if (res.err) {
      throw_exception(env, res.err);
    }
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_add_in_place_matrix
   * Signature: (J)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1in_1place_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    VoidResult res = matrix_add_in_place_matrix(get_matrix_pointer(env, obj), (void*) aptr);
    if (res.err) {
      throw_exception(env, res.err);
    }
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_sub_scalar
   * Signature: (D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    PtrResult res = matrix_sub_scalar(get_matrix_pointer(env, obj), scalar);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_sub_matrix
   * Signature: (J)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    PtrResult res = matrix_sub_matrix(get_matrix_pointer(env, obj), (void*) aptr);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_sub_in_place_scalar
   * Signature: (D)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1in_1place_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    VoidResult res = matrix_sub_in_place_scalar(get_matrix_pointer(env, obj), scalar);
    if (res.err) {
      throw_exception(env, res.err);
    }
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_sub_in_place_matrix
   * Signature: (J)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1in_1place_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    VoidResult res = matrix_sub_in_place_matrix(get_matrix_pointer(env, obj), (void*) aptr);
    if (res.err) {
      throw_exception(env, res.err);
    }
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_mul_scalar
   * Signature: (D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    PtrResult res = matrix_mul_scalar(get_matrix_pointer(env, obj), scalar);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_mul_matrix
   * Signature: (J)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    PtrResult res = matrix_mul_matrix(get_matrix_pointer(env, obj), (void*) aptr);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_mul_in_place_scalar
   * Signature: (D)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1in_1place_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    VoidResult res = matrix_mul_in_place_scalar(get_matrix_pointer(env, obj), scalar);
    if (res.err) {
      throw_exception(env, res.err);
    }
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_mul_in_place_matrix
   * Signature: (J)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1in_1place_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    VoidResult res = matrix_mul_in_place_matrix(get_matrix_pointer(env, obj), (void*) aptr);
    if (res.err) {
      throw_exception(env, res.err);
    }
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_div_scalar
   * Signature: (D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    PtrResult res = matrix_div_scalar(get_matrix_pointer(env, obj), scalar);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_div_matrix
   * Signature: (J)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    PtrResult res = matrix_div_matrix(get_matrix_pointer(env, obj), (void*) aptr);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_div_in_place_scalar
   * Signature: (D)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1in_1place_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    VoidResult res = matrix_div_in_place_scalar(get_matrix_pointer(env, obj), scalar);
    if (res.err) {
      throw_exception(env, res.err);
    }
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_div_in_place_matrix
   * Signature: (J)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1in_1place_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    VoidResult res = matrix_div_in_place_matrix(get_matrix_pointer(env, obj), (void*) aptr);
    if (res.err) {
      throw_exception(env, res.err);
    }
  }


  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_mmul_matrix
   * Signature: (J)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mmul_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    PtrResult res = matrix_mmul_matrix(get_matrix_pointer(env, obj), (void*) aptr);
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_mmul_in_place_matrix
   * Signature: (J)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mmul_1in_1place_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    VoidResult res = matrix_mmul_in_place_matrix(get_matrix_pointer(env, obj), (void*) aptr);
    if (res.err) {
      throw_exception(env, res.err);
    }
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_column_mins
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1column_1mins(
      JNIEnv *env, jobject obj) {
    return (long) matrix_column_mins(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_column_maxs
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1column_1maxs(
      JNIEnv *env, jobject obj) {
    return (long) matrix_column_maxs(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_column_means
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1column_1means(
      JNIEnv *env, jobject obj) {
    return (long) matrix_column_means(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_column_sums
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1column_1sums(
      JNIEnv *env, jobject obj) {
    return (long) matrix_column_sums(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_row_mins
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1row_1mins(
      JNIEnv *env, jobject obj) {
    return (long) matrix_row_mins(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_row_maxs
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1row_1maxs(
      JNIEnv *env, jobject obj) {
    return (long) matrix_row_maxs(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_row_means
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1row_1means(
      JNIEnv *env, jobject obj) {
    return (long) matrix_row_means(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_row_sums
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1row_1sums(
      JNIEnv *env, jobject obj) {
    return (long) matrix_row_sums(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_min
   * Signature: ()D
   */
  JNIEXPORT jdouble JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1min(
      JNIEnv *env, jobject obj) {
    return matrix_min(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_max
   * Signature: ()D
   */
  JNIEXPORT jdouble JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1max(
      JNIEnv *env, jobject obj) {
    return matrix_max(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_sum
   * Signature: ()D
   */
  JNIEXPORT jdouble JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sum(
      JNIEnv *env, jobject obj) {
    return matrix_sum(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_norm1
   * Signature: ()D
   */
  JNIEXPORT jdouble JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1norm1(
      JNIEnv *env, jobject obj) {
    return matrix_norm1(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_norm2
   * Signature: ()D
   */
  JNIEXPORT jdouble JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1norm2(
      JNIEnv *env, jobject obj) {
    return matrix_norm2(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_transpose
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1transpose(
      JNIEnv *env, jobject obj) {
    return (long) matrix_transpose(get_matrix_pointer(env, obj));
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_diag
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1diag(
      JNIEnv *env, jobject obj) {
    PtrResult res = matrix_diag(get_matrix_pointer(env, obj));
    if (res.err) {
      throw_exception(env, res.err);
    }
    return (long) res.ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_abs
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1abs(
      JNIEnv *env, jobject obj) {
    return (long) matrix_abs(get_matrix_pointer(env, obj));
  }

  /* == Bindings for singular value decomposition == */

  // Set SVD result into pointers array
  void set_svd_result(JNIEnv *env, SvdResult res, jobject ptrs) {
    if (res.err) {
      throw_exception(env, res.err);
    }
    // set fields if they are available
    jclass clazz = env->GetObjectClass(ptrs);
    if (res.u) {
      jfieldID fid = env->GetFieldID(clazz, "u", "J");
      env->SetLongField(ptrs, fid, (long) res.u);
    }
    if (res.s) {
      jfieldID fid = env->GetFieldID(clazz, "s", "J");
      env->SetLongField(ptrs, fid, (long) res.s);
    }
    if (res.v) {
      jfieldID fid = env->GetFieldID(clazz, "v", "J");
      env->SetLongField(ptrs, fid, (long) res.v);
    }
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_full_svd
   * Signature: (Lcom/github/sadikovi/rustjblas/DoubleMatrix/SvdResult;)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1full_1svd(
      JNIEnv *env, jobject obj, jobject ptrs) {
    SvdResult res = matrix_full_svd(get_matrix_pointer(env, obj));
    set_svd_result(env, res, ptrs);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_singular_values
   * Signature: ()J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1singular_1values(
      JNIEnv *env, jobject obj) {
    return (long) matrix_singular_values(get_matrix_pointer(env, obj));
  }
}
