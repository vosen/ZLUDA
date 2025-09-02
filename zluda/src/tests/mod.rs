use cuda_macros::generate_api_macro;
use cuda_tests::api_test;
use cuda_tests::cuda::*;

generate_api_macro!(impl cuda_tests::cuda::Api for TestApi use implemented_test);
cuda_macros::cuda_function_declarations!(implemented_test);

api_test!(init_check, TestApi);
