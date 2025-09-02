#[macro_export]
macro_rules! api_trait {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        pub trait Api {
            fn new() -> Self;
            $(fn $fn_name(&self, $( $arg_id : $arg_type ),* ) -> $ret_type;)*
        }
    };
}

#[macro_export]
macro_rules! api_test {
    ($func:ident, $type:ty) => {
        paste::paste! {
            #[test]
            #[allow(non_snake_case)]
            fn [<$func _test>]() {
                unsafe { $func::<$type>(<$type>::new()) }
            }
        }
    };
}

pub mod cuda;
