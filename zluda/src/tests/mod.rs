use cuda_tests::api_test;
use cuda_tests::cuda::*;
use cuda_macros::generate_api_macro;

generate_api_macro!(implemented_test, cuda_tests::cuda::Api, TestApi);
cuda_macros::cuda_function_declarations!(implemented_test);

api_test!(init_check, TestApi);
