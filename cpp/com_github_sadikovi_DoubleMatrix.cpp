#include "com_github_sadikovi_DoubleMatrix.h"

extern "C" {
  /*
   * == IMPLEMENTATION ==
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    alloc_new
   * Signature: (II[D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_DoubleMatrix_alloc_1new(
      JNIEnv *env, jclass type, jint rows, jint cols, jdoubleArray data) {
    // std::cout << "Call alloc_new" << std::endl;
    jlong ptr = 1L;
    return ptr;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    alloc_rand
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_DoubleMatrix_alloc_1rand(
      JNIEnv *env, jclass type, jint rows, jint cols) {
    // std::cout << "Call alloc_rand" << std::endl;
    jlong ptr = 1L;
    return ptr;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    alloc_zeros
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_DoubleMatrix_alloc_1zeros(
      JNIEnv *env, jclass type, jint rows, jint cols) {
    // std::cout << "Call alloc_zeros" << std::endl;
    jlong ptr = 1L;
    return ptr;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    alloc_ones
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_DoubleMatrix_alloc_1ones(
      JNIEnv *env, jclass type, jint rows, jint cols) {
    // std::cout << "Call alloc_ones" << std::endl;
    jlong ptr = 1L;
    return ptr;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_rows
   * Signature: (J)I
   */
  JNIEXPORT jint JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1rows(
      JNIEnv *env, jclass type, jlong ptr) {
    // std::cout << "Call matrix_rows" << std::endl;
    jint rows = 123;
    return rows;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_cols
   * Signature: (J)I
   */
  JNIEXPORT jint JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1cols(
      JNIEnv *env, jclass type, jlong ptr) {
    // std::cout << "Call matrix_cols" << std::endl;
    jint cols = 123;
    return cols;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_show
   * Signature: (JZ)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1show(
      JNIEnv *env, jclass type, jlong ptr, jboolean truncate) {
    // std::cout << "Call matrix_show" << std::endl;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_tostring
   * Signature: (J)Ljava/lang/String;
   */
  JNIEXPORT jstring JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1tostring(
      JNIEnv *env, jclass type, jlong ptr) {
    // std::cout << "Call matrix_tostring" << std::endl;
    char str[32] = "Hello, World!";
    jstring result = env->NewStringUTF(str);
    return result;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_dealloc
   * Signature: (J)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1dealloc(
      JNIEnv *env, jclass type, jlong ptr) {
    // std::cout << "Call matrix_dealloc" << std::endl;
  }
}
