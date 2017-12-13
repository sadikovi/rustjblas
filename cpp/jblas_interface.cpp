#include "com_github_sadikovi_rustjblas_DoubleMatrix.h"
#include "rust_interface.h"

extern "C" {

  /* == Bindings == */

  // Get matrix pointer for the current instance
  void* get_matrix_pointer(JNIEnv *env, jobject obj) {
    jclass clazz = env->GetObjectClass(obj);
    jfieldID fid = env->GetFieldID(clazz, "pointer", "J");
    jlong ptr = env->GetLongField(obj, fid);
    return (void*) ptr;
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    alloc_from_array
   * Signature: (II[D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1from_1array(
      JNIEnv *env, jclass type, jint rows, jint cols, jdoubleArray data) {
    jsize len = env->GetArrayLength(data);
    // always copy elements from java heap
    jdouble *body = env->GetDoubleArrayElements(data, 0);
    return (long) alloc_from_array(rows, cols, len, body);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    alloc_rand
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1rand(
      JNIEnv *env, jclass type, jint rows, jint cols) {
    return (long) alloc_rand(rows, cols);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    alloc_zeros
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1zeros(
      JNIEnv *env, jclass type, jint rows, jint cols) {
    return (long) alloc_zeros(rows, cols);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    alloc_ones
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1ones(
      JNIEnv *env, jclass type, jint rows, jint cols) {
    return (long) alloc_ones(rows, cols);
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
   * Method:    matrix_pretty_string
   * Signature: (Z)Ljava/lang/String;
   */
  JNIEXPORT jstring JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1pretty_1string(
      JNIEnv *env, jobject obj, jboolean truncate) {
    const char *cstr = matrix_pretty_string(get_matrix_pointer(env, obj), truncate);
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
    return (long) matrix_add_scalar(get_matrix_pointer(env, obj), scalar);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_add_matrix
   * Signature: (J)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    return (long) matrix_add_matrix(get_matrix_pointer(env, obj), (void*) aptr);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_add_in_place_scalar
   * Signature: (D)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1in_1place_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    matrix_add_in_place_scalar(get_matrix_pointer(env, obj), scalar);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_add_in_place_matrix
   * Signature: (J)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1in_1place_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    matrix_add_in_place_matrix(get_matrix_pointer(env, obj), (void*) aptr);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_sub_scalar
   * Signature: (D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    return (long) matrix_sub_scalar(get_matrix_pointer(env, obj), scalar);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_sub_matrix
   * Signature: (J)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    return (long) matrix_sub_matrix(get_matrix_pointer(env, obj), (void*) aptr);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_sub_in_place_scalar
   * Signature: (D)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1in_1place_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    matrix_sub_in_place_scalar(get_matrix_pointer(env, obj), scalar);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_sub_in_place_matrix
   * Signature: (J)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1in_1place_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    matrix_sub_in_place_matrix(get_matrix_pointer(env, obj), (void*) aptr);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_mul_scalar
   * Signature: (D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    return (long) matrix_mul_scalar(get_matrix_pointer(env, obj), scalar);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_mul_matrix
   * Signature: (J)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    return (long) matrix_mul_matrix(get_matrix_pointer(env, obj), (void*) aptr);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_mul_in_place_scalar
   * Signature: (D)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1in_1place_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    matrix_mul_in_place_scalar(get_matrix_pointer(env, obj), scalar);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_mul_in_place_matrix
   * Signature: (J)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1in_1place_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    matrix_mul_in_place_matrix(get_matrix_pointer(env, obj), (void*) aptr);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_div_scalar
   * Signature: (D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    return (long) matrix_div_scalar(get_matrix_pointer(env, obj), scalar);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_div_matrix
   * Signature: (J)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    return (long) matrix_div_matrix(get_matrix_pointer(env, obj), (void*) aptr);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_div_in_place_scalar
   * Signature: (D)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1in_1place_1scalar(
      JNIEnv *env, jobject obj, jdouble scalar) {
    matrix_div_in_place_scalar(get_matrix_pointer(env, obj), scalar);
  }

  /*
   * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
   * Method:    matrix_div_in_place_matrix
   * Signature: (J)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1in_1place_1matrix(
      JNIEnv *env, jobject obj, jlong aptr) {
    matrix_div_in_place_matrix(get_matrix_pointer(env, obj), (void*) aptr);
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
    return (long) matrix_diag(get_matrix_pointer(env, obj));
  }
}
