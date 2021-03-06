/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class com_github_sadikovi_rustjblas_DoubleMatrix */

#ifndef _Included_com_github_sadikovi_rustjblas_DoubleMatrix
#define _Included_com_github_sadikovi_rustjblas_DoubleMatrix
#ifdef __cplusplus
extern "C" {
#endif
#undef com_github_sadikovi_rustjblas_DoubleMatrix_INVALID_PTR
#define com_github_sadikovi_rustjblas_DoubleMatrix_INVALID_PTR -1LL
/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    alloc_from_array
 * Signature: (II[D)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1from_1array
  (JNIEnv *, jclass, jint, jint, jdoubleArray);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    alloc_rand
 * Signature: (II)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1rand
  (JNIEnv *, jclass, jint, jint);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    alloc_zeros
 * Signature: (II)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1zeros
  (JNIEnv *, jclass, jint, jint);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    alloc_ones
 * Signature: (II)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1ones
  (JNIEnv *, jclass, jint, jint);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    alloc_identity
 * Signature: (II)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_alloc_1identity
  (JNIEnv *, jclass, jint, jint);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_rows
 * Signature: ()I
 */
JNIEXPORT jint JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1rows
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_cols
 * Signature: ()I
 */
JNIEXPORT jint JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1cols
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_data_array
 * Signature: ()[D
 */
JNIEXPORT jdoubleArray JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1data_1array
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_pretty_string
 * Signature: (I)Ljava/lang/String;
 */
JNIEXPORT jstring JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1pretty_1string
  (JNIEnv *, jobject, jint);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_dealloc
 * Signature: ()V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1dealloc
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_add_scalar
 * Signature: (D)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1scalar
  (JNIEnv *, jobject, jdouble);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_add_matrix
 * Signature: (J)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1matrix
  (JNIEnv *, jobject, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_add_in_place_scalar
 * Signature: (D)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1in_1place_1scalar
  (JNIEnv *, jobject, jdouble);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_add_in_place_matrix
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1add_1in_1place_1matrix
  (JNIEnv *, jobject, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_sub_scalar
 * Signature: (D)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1scalar
  (JNIEnv *, jobject, jdouble);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_sub_matrix
 * Signature: (J)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1matrix
  (JNIEnv *, jobject, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_sub_in_place_scalar
 * Signature: (D)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1in_1place_1scalar
  (JNIEnv *, jobject, jdouble);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_sub_in_place_matrix
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sub_1in_1place_1matrix
  (JNIEnv *, jobject, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_mul_scalar
 * Signature: (D)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1scalar
  (JNIEnv *, jobject, jdouble);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_mul_matrix
 * Signature: (J)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1matrix
  (JNIEnv *, jobject, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_mul_in_place_scalar
 * Signature: (D)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1in_1place_1scalar
  (JNIEnv *, jobject, jdouble);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_mul_in_place_matrix
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mul_1in_1place_1matrix
  (JNIEnv *, jobject, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_div_scalar
 * Signature: (D)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1scalar
  (JNIEnv *, jobject, jdouble);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_div_matrix
 * Signature: (J)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1matrix
  (JNIEnv *, jobject, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_div_in_place_scalar
 * Signature: (D)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1in_1place_1scalar
  (JNIEnv *, jobject, jdouble);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_div_in_place_matrix
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1div_1in_1place_1matrix
  (JNIEnv *, jobject, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_mmul_matrix
 * Signature: (J)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mmul_1matrix
  (JNIEnv *, jobject, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_mmul_in_place_matrix
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1mmul_1in_1place_1matrix
  (JNIEnv *, jobject, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_column_mins
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1column_1mins
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_column_maxs
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1column_1maxs
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_column_means
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1column_1means
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_column_sums
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1column_1sums
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_row_mins
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1row_1mins
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_row_maxs
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1row_1maxs
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_row_means
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1row_1means
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_row_sums
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1row_1sums
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_put_column
 * Signature: (IJ)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1put_1column
  (JNIEnv *, jobject, jint, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_get_column
 * Signature: (I)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1get_1column
  (JNIEnv *, jobject, jint);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_put_row
 * Signature: (IJ)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1put_1row
  (JNIEnv *, jobject, jint, jlong);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_get_row
 * Signature: (I)J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1get_1row
  (JNIEnv *, jobject, jint);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_min
 * Signature: ()D
 */
JNIEXPORT jdouble JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1min
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_max
 * Signature: ()D
 */
JNIEXPORT jdouble JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1max
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_sum
 * Signature: ()D
 */
JNIEXPORT jdouble JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1sum
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_norm1
 * Signature: ()D
 */
JNIEXPORT jdouble JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1norm1
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_norm2
 * Signature: ()D
 */
JNIEXPORT jdouble JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1norm2
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_transpose
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1transpose
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_diag
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1diag
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_abs
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1abs
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_full_svd
 * Signature: (Lcom/github/sadikovi/rustjblas/DoubleMatrix/SvdResult;)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1full_1svd
  (JNIEnv *, jobject, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_singular_values
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1singular_1values
  (JNIEnv *, jobject);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_svd_k
 * Signature: (Lcom/github/sadikovi/rustjblas/DoubleMatrix/SvdResult;I)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1svd_1k
  (JNIEnv *, jobject, jobject, jint);

/*
 * Class:     com_github_sadikovi_rustjblas_DoubleMatrix
 * Method:    matrix_lansvd_k
 * Signature: (Lcom/github/sadikovi/rustjblas/DoubleMatrix/SvdResult;I)V
 */
JNIEXPORT void JNICALL Java_com_github_sadikovi_rustjblas_DoubleMatrix_matrix_1lansvd_1k
  (JNIEnv *, jobject, jobject, jint);

#ifdef __cplusplus
}
#endif
#endif
