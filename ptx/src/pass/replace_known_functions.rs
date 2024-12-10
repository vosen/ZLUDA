use super::{GlobalStringIdentResolver2, NormalizedDirective2, SpirvWord};

pub(crate) fn run<'input>(
    resolver: &GlobalStringIdentResolver2<'input>,
    mut directives: Vec<NormalizedDirective2<'input>>,
) -> Vec<NormalizedDirective2<'input>> {
    for directive in directives.iter_mut() {
        match directive {
            NormalizedDirective2::Method(func) => {
                func.import_as =
                    replace_with_ptx_impl(resolver, &func.func_decl.name, func.import_as.take());
            }
            _ => {}
        }
    }
    directives
}

fn replace_with_ptx_impl<'input>(
    resolver: &GlobalStringIdentResolver2<'input>,
    fn_name: &ptx_parser::MethodName<'input, SpirvWord>,
    name: Option<String>,
) -> Option<String> {
    let known_names = ["__assertfail"];
    match name {
        Some(name) if known_names.contains(&&*name) => Some(format!("__zluda_ptx_impl_{}", name)),
        Some(name) => Some(name),
        None => match fn_name {
            ptx_parser::MethodName::Func(name) => match resolver.ident_map.get(name) {
                Some(super::IdentEntry {
                    name: Some(name), ..
                }) => Some(format!("__zluda_ptx_impl_{}", name)),
                _ => None,
            },
            ptx_parser::MethodName::Kernel(..) => None,
        },
    }
}
