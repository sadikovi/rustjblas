/* RUST LIBRARY METHODS */

#ifdef __cplusplus
extern "C" {
#endif

/* static methods */
void* alloc_from_array(int, int, size_t, const double*);
void* alloc_rand(int, int);
void* alloc_zeros(int, int);
void* alloc_ones(int, int);

/* instance methods */
int matrix_rows(void*);
int matrix_cols(void*);
const char* matrix_pretty_string(void*, bool);
void matrix_dealloc(void*);

void* matrix_add_scalar(void*, double);
void* matrix_add_matrix(void*, void*);
void matrix_add_in_place_scalar(void*, double);
void matrix_add_in_place_matrix(void*, void*);

void* matrix_sub_scalar(void*, double);
void* matrix_sub_matrix(void*, void*);
void matrix_sub_in_place_scalar(void*, double);
void matrix_sub_in_place_matrix(void*, void*);

void* matrix_mul_scalar(void*, double);
void* matrix_mul_matrix(void*, void*);
void matrix_mul_in_place_scalar(void*, double);
void matrix_mul_in_place_matrix(void*, void*);

void* matrix_div_scalar(void*, double);
void* matrix_div_matrix(void*, void*);
void matrix_div_in_place_scalar(void*, double);
void matrix_div_in_place_matrix(void*, void*);

void* matrix_column_mins(void*);
void* matrix_column_maxs(void*);
void* matrix_column_means(void*);
void* matrix_column_sums(void*);

double matrix_min(void*);
double matrix_max(void*);
double matrix_sum(void*);
double matrix_norm1(void*);
double matrix_norm2(void*);

void* matrix_transpose(void*);
void* matrix_diag(void*);

#ifdef __cplusplus
}
#endif
