#include "fixpoint.h"

#define N 3

typedef int DT;

typedef struct
{
	DT m[N*N]; // (1)
} InputMatrix;

typedef struct
{
	DT b[N]; // (1)
} InputVector;

typedef struct
{
	DT res[N];
} Output;

void memcpy(int* destination, int* source, int size) {
	for (int i = 0; i < size; i++) {
		destination[i] = source[i];
	}
}

DT abs(DT val) {
	if(val < 0) {
		return -val;
	} else {
		return val;
	}
}

void identity(DT* OUTPUT_m) {
	for(int i = 0; i<N; i++) {
		for(int j = 0; j<N; j++) {
			if(i==j) {
				OUTPUT_m[i*N+j] = 1;
			 } else{
				OUTPUT_m[i*N+j] = 0; 
			 }	
		}
	}
	//return I;
}

/**
 * Recomputes the result once LU decomposition is completed
 */
void solve_backtracking(DT *m, DT *b, DT *OUTPUT_res) {
	for(int i = 0; i < N;i++) {
		OUTPUT_res[i] = 0;
	}
	OUTPUT_res[N-1]= fixedpt_div(b[N-1], m[N*N-1]);
	for(int i = N-2; i >=0; i--) {
		DT tmp = 0;
		for(int j = i+1; j < N; j++) {
			tmp += fixedpt_mul(OUTPUT_res[j], m[i*N+j]);
		}
		OUTPUT_res[i] = fixedpt_div((b[i] - tmp), m[i*N+i]);
	}	
}

void swap(DT* m, DT* v, DT* OUTPUT_m, DT* OUTPUT_v, int from, int to) {
	if(from!=to) {
		// Iterate over columns)
		for(int j = from; j < N; j++) {
			DT tmp = m[from*N+j];
			m[from*N+j] = m[to*N+j];
			m[to*N+j] = tmp;
		}
		DT tmp = v[from];
		v[from] = v[to];
		v[to] = tmp;
	}
	for (int i = 0; i < N*N*sizeof(DT); i++) {
		OUTPUT_m[i] = m[i];
	}
	for (int i = 0; i < N*sizeof(DT); i++) {
		OUTPUT_v[i] = v[i];
	}
}

/**
 * Performs the propagating swap for LU decomposition
 */
void pivot_swap(DT *m, DT *b, DT *OUTPUT_m, DT *OUTPUT_b, int i) {
	for (int i = 0; i < N*N*sizeof(DT); i++) {
		OUTPUT_m[i] = m[i];
	}
	for (int i = 0; i < sizeof(DT)*N; i++) {
		OUTPUT_b[i] = b[i];
	}
	for(int k=i+1; k < N; k++) {
		if(m[k*N+i] > m[i*N+i]) {
			swap(m, b, OUTPUT_m, OUTPUT_b, i, k);
			for (int i = 0; i < N*N*sizeof(DT); i++) {
				OUTPUT_m[i] = m[i];
			}
			for (int i = 0; i < sizeof(DT)*N; i++) {
				OUTPUT_b[i] = b[i];
			}
		}
	}
}

/**
 *  Guassian with propagating pivot for fix point computations
 */
void gaussj_D(DT *m, DT *b, DT *OUTPUT_res) {
	InputMatrix L;
	identity(L.m);
	// Iterations
	for(int i= 0; i < N-1; i++) {
		// Swap
		DT m_tmp[N*N];
		DT b_tmp[N];

		// pivot_swap(m, b, m_tmp, b_tmp, i, N);
		// memcpy(OUTPUT_m, m, sizeof(DT)*N*N);
		for (int j = 0; j < N*N; j++) {
			m_tmp[j] = m[j];
		}
		// memcpy(OUTPUT_b, b, sizeof(DT)*N);
		for (int j = 0; j < N; j++) {
			b_tmp[j] = b[j];
		}
		
		for(int k=i+1; k < N; k++) {
			if(m[k*N+i] > m[i*N+i]) {
				// swap(m, b, m_tmp, b_tmp, i, k);
				
				if(i!=k) {
					// Iterate over columns)
					for(int j = i; j < N; j++) {
						DT tmp = m[i*N+j];
						m[i*N+j] = m[k*N+j];
						m[k*N+j] = tmp;
					}
					DT tmp = b[i];
					b[i] = b[k];
					b[k] = tmp;
				}

				// memcpy(OUTPUT_m, m, N*N*sizeof(DT));
				for (int j = 0; j < N*N; j++) {
					m_tmp[j] = m[j];
				}

				// memcpy(OUTPUT_v, v, N*sizeof(DT));
				for (int j = 0; j < N; j++) {
					b_tmp[j] = b[j];
				}

				// end swap

				// memcpy(m, OUTPUT_m, sizeof(DT)*N*N);
				for (int j = 0; j < N*N; j++) {
					m[j] = m_tmp[j];
				}
				// memcpy(b, OUTPUT_b, sizeof(DT)*N);
				for (int j = 0; j < N; j++) {
					b[j] = b_tmp[j];
				}
			}
		}

		// memcpy(m, m_tmp, sizeof(DT)*N*N);
		for (int j = 0; j < N*N; j++) {
			m[j] = m_tmp[j];
		}
		// memcpy(b, b_tmp, sizeof(DT)*N);
		for (int j = 0; j < N; j++) {
			b[j] = b_tmp[j];
		}


		// Iterate over rows in remainder
		for(int k=i+1; k < N; k++) {
			// L.m[k*N+i] = a.m[k*N+i] / a.m[i*N+i]; // TODO need div-zero check
			L.m[k*N+i] = fixedpt_div(m[k*N+i], m[i*N+i]);
			// Iterates over columns in remainder
			for(int j = i; j < N; j++) {
				// Berechnung von R
				// R(k,j) := R(k,j) - L(k,i) * R(i,j)
				//a.m[k*N+j] = a.m[k*N+j] - L.m[k*N+i] * a.m[i*N+j];
				m[k*N+j] = m[k*N+j] - fixedpt_mul(L.m[k*N+i],m[i*N+j]);
			}
			// b.b[k] = b.b[k] - L.m[k*N+i] * b.b[i];
			b[k] = b[k] - fixedpt_mul(L.m[k*N+i],b[i]);
		}	
	}

	// Output
	solve_backtracking(m, b, OUTPUT_res);
	
	// // return out;
}

Output main(__attribute__((private(0))) int a[N*N], __attribute__((private(1))) int b[N]) {
	InputMatrix INPUT_A_m;
	InputVector INPUT_B_b;
	for (int i = 0; i < N * N; i++) {
		INPUT_A_m.m[i] = a[i];
	}
	for (int i = 0; i < N; i++) {
		INPUT_B_b.b[i] = b[i];
	}

	Output OUTPUT_res;
	gaussj_D(INPUT_A_m.m, INPUT_B_b.b, OUTPUT_res.res);
	return OUTPUT_res;
}