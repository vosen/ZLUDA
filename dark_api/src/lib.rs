use uuid::uuid;

const HEAP_ACCESS: uuid::Uuid = uuid!("{7330F811-F47F-41BC-A4FF-E792D073F41F}");

macro_rules! dark_api {
    ($(
        $guid:expr => $name:ident [$len:literal] {
            $( [$index:literal] = $fn_:ident $( ( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty )? ),*
        }
    ),+) => {

    };
}

dark_api! {
    "{7330F811-F47F-41BC-A4FF-E792D073F41F}" => CUDART_INTERFACE[10] {
        [0] = SIZE_OF,
        [1] = cudart_interface_fn1() -> (),
        [2] = get_module_from_cubin() -> (),
        [6] = get_module_from_cubin_ext1() -> (),
        [7] = cudart_interface_fn6() -> (),
        [8] = get_module_from_cubin_ext2() -> ()
    },
    "{7330F811-F47F-41BC-A4FF-E792D073F41F}" => CONTEXT_LOCAL_STORAGE_INTERFACE_V0301[4] {
        [0] = context_local_storage_ctor()->(),
        [1] = context_local_storage_dtor() -> (),
        [2] = context_local_storage_get_state() -> ()
    }
}
