/* RUST LIBRARY METHODS */

#ifdef __cplusplus
extern "C" {
#endif

/* Pointer result for matrix, contains following fields:
 * `ptr` - raw pointer to the matrix
 * `err` - error message pointer.
 * If `err` pointer is NULL then result is evaluated to Ok, otherwise error occurred and exception
 * should thrown with error message.
 *
 * Methods that take external parameters besides pointer to current matrix should return PtrResult.
 */
typedef struct PtrResult {
  void *ptr;
  const char *err;
} PtrResult;

/* Void result for methods that can throw an exception, but return nothing.
 * Behaves similar to PtrResult, in the sense that if `err` is NULL, then no exception is thrown
 * and operation is successful, otherwise, exception should thrown with message `err`.
 */
typedef struct VoidResult {
  const char *err;
} VoidResult;

/* DoubleArray struct to return array of double values with its length.
 * Added mainly for convenience.
 */
typedef struct DoubleArray {
  int len;
  const double* data;
} DoubleArray;

/* static methods */
PtrResult alloc_from_array(int, int, size_t, const double*);
PtrResult alloc_rand(int, int);
PtrResult alloc_zeros(int, int);
PtrResult alloc_ones(int, int);

/* instance methods */
int matrix_rows(void*);
int matrix_cols(void*);
DoubleArray matrix_data_array(void*);
const char* matrix_pretty_string(void*);
void matrix_dealloc(void*);

PtrResult matrix_add_scalar(void*, double);
PtrResult matrix_add_matrix(void*, void*);
VoidResult matrix_add_in_place_scalar(void*, double);
VoidResult matrix_add_in_place_matrix(void*, void*);

PtrResult matrix_sub_scalar(void*, double);
PtrResult matrix_sub_matrix(void*, void*);
VoidResult matrix_sub_in_place_scalar(void*, double);
VoidResult matrix_sub_in_place_matrix(void*, void*);

PtrResult matrix_mul_scalar(void*, double);
PtrResult matrix_mul_matrix(void*, void*);
VoidResult matrix_mul_in_place_scalar(void*, double);
VoidResult matrix_mul_in_place_matrix(void*, void*);

PtrResult matrix_div_scalar(void*, double);
PtrResult matrix_div_matrix(void*, void*);
VoidResult matrix_div_in_place_scalar(void*, double);
VoidResult matrix_div_in_place_matrix(void*, void*);

void* matrix_column_mins(void*);
void* matrix_column_maxs(void*);
void* matrix_column_means(void*);
void* matrix_column_sums(void*);

void* matrix_row_mins(void*);
void* matrix_row_maxs(void*);
void* matrix_row_means(void*);
void* matrix_row_sums(void*);

double matrix_min(void*);
double matrix_max(void*);
double matrix_sum(void*);
double matrix_norm1(void*);
double matrix_norm2(void*);

void* matrix_transpose(void*);
PtrResult matrix_diag(void*);

#ifdef __cplusplus
}
#endif
