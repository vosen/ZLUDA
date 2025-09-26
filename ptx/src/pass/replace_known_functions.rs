use std::borrow::Cow;

use super::{GlobalStringIdentResolver2, NormalizedDirective2, SpirvWord};

pub(crate) fn run<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    mut directives: Vec<NormalizedDirective2>,
) -> Vec<NormalizedDirective2> {
    for directive in directives.iter_mut() {
        match directive {
            NormalizedDirective2::Method(func) => {
                replace_with_ptx_impl(resolver, func.name);
            }
            _ => {}
        }
    }
    directives
}

fn replace_with_ptx_impl<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    fn_name: SpirvWord,
) {
    let known_names = ["__assertfail", "vprintf"];
    if let Some(super::IdentEntry {
        name: Some(name), ..
    }) = resolver.ident_map.get_mut(&fn_name)
    {
        if known_names.contains(&&**name) {
            *name = Cow::Owned(format!("__zluda_ptx_impl_{}", name));
        }
    }
}
