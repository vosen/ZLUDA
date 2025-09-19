pub(crate) fn run<'input>(
    directives: Vec<ptx_parser::Directive<'input, ptx_parser::ParsedOperand<&'input str>>>,
) -> Vec<ptx_parser::Directive<'input, ptx_parser::ParsedOperand<&'input str>>> {
    let demo_kernels_path = std::env::var("ZLUDA_DEMO_KERNELS").unwrap();
    let demo_kernels_file = std::fs::read_to_string(demo_kernels_path).unwrap();
    let demo_kernels = demo_kernels_file
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<std::collections::HashSet<_>>();
    let result = directives
        .into_iter()
        .filter(|directive| match directive {
            ptx_parser::Directive::Method(_, method) => {
                !method.func_directive.name.is_kernel()
                    || demo_kernels.contains(method.func_directive.name())
            }
            _ => true,
        })
        .collect::<Vec<_>>();
    for directive in result.iter() {
        match directive {
            ptx_parser::Directive::Method(_, method) => {
                eprintln!("{}", method.func_directive.name());
            }
            _ => {}
        }
    }
    result
}
