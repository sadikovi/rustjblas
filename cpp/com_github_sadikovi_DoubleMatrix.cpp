#include "com_github_sadikovi_DoubleMatrix.h"

extern "C" {
  /*
   * Define methods to link from rust static library
   */
  long double_matrix_alloc_new(int, int, size_t, const double*);
  long double_matrix_alloc_rand(int, int);
  long double_matrix_alloc_zeros(int, int);
  long double_matrix_alloc_ones(int, int);
  /* instance methods */
  int double_matrix_instance_rows(void*);
  int double_matrix_instance_cols(void*);
  void double_matrix_instance_show(void*, int);
  const char* double_matrix_instance_tostring(void*);
  void double_matrix_instance_dealloc(void*);

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    alloc_new
   * Signature: (II[D)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_DoubleMatrix_alloc_1new(
      JNIEnv *env, jclass type, jint rows, jint cols, jdoubleArray data) {
    jsize len = env->GetArrayLength(data);
    // always copy elements from java heap
    jdouble *body = env->GetDoubleArrayElements(data, 0);
    return double_matrix_alloc_new(rows, cols, len, body);
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    alloc_rand
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_DoubleMatrix_alloc_1rand(
      JNIEnv *env, jclass type, jint rows, jint cols) {
    return double_matrix_alloc_rand(rows, cols);
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    alloc_zeros
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_DoubleMatrix_alloc_1zeros(
      JNIEnv *env, jclass type, jint rows, jint cols) {
    return double_matrix_alloc_zeros(rows, cols);
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    alloc_ones
   * Signature: (II)J
   */
  JNIEXPORT jlong JNICALL Java_com_github_sadikovi_DoubleMatrix_alloc_1ones(
      JNIEnv *env, jclass type, jint rows, jint cols) {
    return double_matrix_alloc_ones(rows, cols);
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_rows
   * Signature: ()I
   */
  JNIEXPORT jint JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1rows(
      JNIEnv *env, jobject obj) {
    jclass clazz = env->GetObjectClass(obj);
    jfieldID fid = env->GetFieldID(clazz, "pointer", "J");
    jlong ptr = env->GetLongField(obj, fid);
    return double_matrix_instance_rows((void*) ptr);
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_cols
   * Signature: ()I
   */
  JNIEXPORT jint JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1cols(
      JNIEnv *env, jobject obj) {
    jclass clazz = env->GetObjectClass(obj);
    jfieldID fid = env->GetFieldID(clazz, "pointer", "J");
    jlong ptr = env->GetLongField(obj, fid);
    return double_matrix_instance_cols((void*) ptr);
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_show
   * Signature: (Z)V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1show(
      JNIEnv *env, jobject obj, jboolean truncate) {
    jclass clazz = env->GetObjectClass(obj);
    jfieldID fid = env->GetFieldID(clazz, "pointer", "J");
    jlong ptr = env->GetLongField(obj, fid);
    double_matrix_instance_show((void*) ptr, (int) truncate);
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_tostring
   * Signature: ()Ljava/lang/String;
   */
  JNIEXPORT jstring JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1tostring(
      JNIEnv *env, jobject obj) {
    jclass clazz = env->GetObjectClass(obj);
    jfieldID fid = env->GetFieldID(clazz, "pointer", "J");
    jlong ptr = env->GetLongField(obj, fid);
    const char *cstr = double_matrix_instance_tostring((void*) ptr);
    jstring result = env->NewStringUTF(cstr);
    return result;
  }

  /*
   * Class:     com_github_sadikovi_DoubleMatrix
   * Method:    matrix_dealloc
   * Signature: ()V
   */
  JNIEXPORT void JNICALL Java_com_github_sadikovi_DoubleMatrix_matrix_1dealloc(
      JNIEnv *env, jobject obj) {
    jclass clazz = env->GetObjectClass(obj);
    jfieldID fid = env->GetFieldID(clazz, "pointer", "J");
    jlong ptr = env->GetLongField(obj, fid);
    double_matrix_instance_dealloc((void*) ptr);
  }
}
