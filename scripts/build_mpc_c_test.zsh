#!/usr/bin/env zsh

set -ex

disable -r time

# cargo build --release --features c --example circ 

BIN=./target/release/examples/circ
export CARGO_MANIFEST_DIR=$(pwd)

case "$OSTYPE" in 
    darwin*)
        alias measure_time="gtime --format='%e seconds %M kB'"
    ;;
    linux*)
        alias measure_time="time --format='%e seconds %M kB'"
    ;;
esac

function mpc_test {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical" --selection-scheme "smart_lp" --part-size 8000 --mut-level 2 --mut-step-size 1 --graph-type 0
}

function mpc_test_wan {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical_wan" --selection-scheme "smart_lp" --part-size 8000 --mut-level 2 --mut-step-size 1 --graph-type 0
}

function mpc_test_glp_lan  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical" --selection-scheme "smart_glp"
}

function mpc_test_g_y  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical" --selection-scheme "smart_g_y"
}

function mpc_test_g_b  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical" --selection-scheme "smart_g_b"
}

function mpc_test_g_ay  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical" --selection-scheme "smart_g_a+y"
}

function mpc_test_g_ab  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical" --selection-scheme "smart_g_a+b"
}

function mpc_test_glp_wan  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical_wan" --selection-scheme "smart_glp"
}

function mpc_test_css {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical" --selection-scheme "css" --part-size 8000 --mut-level 2 --mut-step-size 1 --graph-type 0
}

function mpc_test_css_wan {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical_wan" --selection-scheme "css" --part-size 8000 --mut-level 2 --mut-step-size 1 --graph-type 0
}

function mpc_test_opa_ay  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical" --selection-scheme "opa_ay"
}

function mpc_test_opa_ab  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical" --selection-scheme "opa_ab"
}

function mpc_test_opa_by  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "empirical" --selection-scheme "opa_by"
}

function mpc_test_bool  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "hycc" --selection-scheme "b"
}

function mpc_test_yao  {
    parties=$1
    cpath=$2
    RUST_BACKTRACE=1 measure_time $BIN --parties $parties $cpath mpc --cost-model "hycc" --selection-scheme "y"
}

# mpc_test_2 2 ./examples/C/mpc/playground.c

# build mpc arithmetic tests
mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_add.c
mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_sub.c
mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_mult.c
mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_mult_add_pub.c
mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_mod.c
mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_add_unsigned.c

mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_int_equals.c
mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_int_greater_than.c
mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_int_greater_equals.c
mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_int_less_than.c
mpc_test 2 ./examples/C/mpc/unit_tests/arithmetic_tests/2pc_int_less_equals.c

# # build nary arithmetic tests
mpc_test 2 ./examples/C/mpc/unit_tests/nary_arithmetic_tests/2pc_nary_arithmetic_add.c

# # build bitwise tests
mpc_test 2 ./examples/C/mpc/unit_tests/bitwise_tests/2pc_bitwise_and.c
mpc_test 2 ./examples/C/mpc/unit_tests/bitwise_tests/2pc_bitwise_or.c
mpc_test 2 ./examples/C/mpc/unit_tests/bitwise_tests/2pc_bitwise_xor.c

# # build boolean tests
mpc_test 2 ./examples/C/mpc/unit_tests/boolean_tests/2pc_boolean_and.c
mpc_test 2 ./examples/C/mpc/unit_tests/boolean_tests/2pc_boolean_or.c
mpc_test 2 ./examples/C/mpc/unit_tests/boolean_tests/2pc_boolean_equals.c

# # build nary boolean tests
mpc_test 2 ./examples/C/mpc/unit_tests/nary_boolean_tests/2pc_nary_boolean_and.c

# # build const tests
mpc_test 2 ./examples/C/mpc/unit_tests/const_tests/2pc_const_arith.c
mpc_test 2 ./examples/C/mpc/unit_tests/const_tests/2pc_const_bool.c

# build if statement tests
mpc_test 2 ./examples/C/mpc/unit_tests/ite_tests/2pc_ite_ret_bool.c
mpc_test 2 ./examples/C/mpc/unit_tests/ite_tests/2pc_ite_ret_int.c
mpc_test 2 ./examples/C/mpc/unit_tests/ite_tests/2pc_ite_only_if.c

# build shift tests
mpc_test 2 ./examples/C/mpc/unit_tests/shift_tests/2pc_lhs.c
mpc_test 2 ./examples/C/mpc/unit_tests/shift_tests/2pc_rhs.c

# build div tests
mpc_test 2 ./examples/C/mpc/unit_tests/div_tests/2pc_div.c

# build array tests
mpc_test 2 ./examples/C/mpc/unit_tests/array_tests/2pc_array_sum.c
mpc_test 2 ./examples/C/mpc/unit_tests/array_tests/2pc_array_index.c
mpc_test 2 ./examples/C/mpc/unit_tests/array_tests/2pc_array_index_2.c
mpc_test 2 ./examples/C/mpc/unit_tests/array_tests/2pc_array_index_3.c

# build circ/compiler array tests
mpc_test 2 ./examples/C/mpc/unit_tests/c_array_tests/2pc_array.c
mpc_test 2 ./examples/C/mpc/unit_tests/c_array_tests/2pc_array_1.c
mpc_test 2 ./examples/C/mpc/unit_tests/c_array_tests/2pc_array_2.c
mpc_test 2 ./examples/C/mpc/unit_tests/c_array_tests/2pc_array_3.c
mpc_test 2 ./examples/C/mpc/unit_tests/c_array_tests/2pc_array_sum_c.c

# build function tests
mpc_test 2 ./examples/C/mpc/unit_tests/function_tests/2pc_function_add.c


# build struct tests 
mpc_test 2 ./examples/C/mpc/unit_tests/struct_tests/2pc_struct_add.c
mpc_test 2 ./examples/C/mpc/unit_tests/struct_tests/2pc_struct_array_add.c

# build matrix tests
mpc_test 2 ./examples/C/mpc/unit_tests/matrix_tests/2pc_matrix_add.c
mpc_test 2 ./examples/C/mpc/unit_tests/matrix_tests/2pc_matrix_assign_add.c
mpc_test 2 ./examples/C/mpc/unit_tests/matrix_tests/2pc_matrix_ptr_add.c

# build ptr tests
mpc_test 2 ./examples/C/mpc/unit_tests/ptr_tests/2pc_ptr_add.c

# build misc tests
mpc_test 2 ./examples/C/mpc/unit_tests/misc_tests/2pc_millionaires.c
mpc_test 2 ./examples/C/mpc/unit_tests/misc_tests/2pc_multi_var.c

# build hycc benchmarks
mpc_test 2 ./examples/C/mpc/benchmarks/biomatch/biomatch.c
mpc_test 2 ./examples/C/mpc/benchmarks/kmeans/2pc_kmeans_.c
# mpc_test 2 ./examples/C/mpc/benchmarks/db/db_join2.c
# mpc_test 2 ./examples/C/mpc/benchmarks/gauss/2pc_gauss_inline.c
# mpc_test_css 2 ./examples/C/mpc/benchmarks/cryptonets/cryptonets.c
# mpc_test_css 2 ./examples/C/mpc/benchmarks/mnist/mnist.c



# ilp benchmarks
# mpc_test 2 ./examples/C/mpc/ilp_benchmarks/2pc_ilp_bench_1.c
# mpc_test 2 ./examples/C/mpc/ilp_benchmarks/2pc_ilp_bench_2.c
# mpc_test 2 ./examples/C/mpc/ilp_benchmarks/2pc_ilp_bench_3.c
# mpc_test 2 ./examples/C/mpc/ilp_benchmarks/2pc_ilp_bench_4.c
# mpc_test 2 ./examples/C/mpc/ilp_benchmarks/2pc_ilp_bench_5.c
# mpc_test 2 ./examples/C/mpc/ilp_benchmarks/2pc_ilp_bench_6.c
# mpc_test 2 ./examples/C/mpc/ilp_benchmarks/2pc_ilp_bench_7.c
# mpc_test 2 ./examples/C/mpc/ilp_benchmarks/2pc_ilp_bench_8.c
# mpc_test 2 ./examples/C/mpc/ilp_benchmarks/2pc_ilp_bench_9.c
# mpc_test 2 ./examples/C/mpc/ilp_benchmarks/2pc_ilp_bench.c
