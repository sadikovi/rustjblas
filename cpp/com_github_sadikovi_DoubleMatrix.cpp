#include "com_github_sadikovi_DoubleMatrix.h"

extern "C" {
  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    alloc_new
   * Signature: (II[D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_DoubleMatrix_alloc_1new(
      JNIEnv *env, jclass type, jint rows, jint cols, jdoubleArray data) {
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
    jlong ptr = 1L;
    return ptr;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_rows
   * Signature: ()I
   */
  JNIEXPORT jint JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1rows(
      JNIEnv *env, jobject obj) {
    jint rows = 123;
    return rows;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_cols
   * Signature: ()I
   */
  JNIEXPORT jint JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1cols(
      JNIEnv *env, jobject obj);
    jint cols = 123;
    return cols;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_show
   * Signature: (Z)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1show(
      JNIEnv *env, jobject obj, jboolean truncate) {
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_tostring
   * Signature: ()Ljava/lang/String;
   */
  JNIEXPORT jstring JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1tostring(
      JNIEnv *env, jobject obj) {
    char str[32] = "Hello, World!";
    jstring result = env->NewStringUTF(str);
    return result;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_dealloc
   * Signature: ()V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1dealloc(
      JNIEnv *env, jobject obj) {
  }
}
