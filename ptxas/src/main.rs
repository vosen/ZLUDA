use bpaf::{any, choice, doc::Style, literal, Bpaf, Parser};

#[derive(Debug, Clone, Bpaf)]
#[allow(dead_code)]
#[bpaf(options, version("Cuda compilation tools, release 12.8, V12.8.0"))]
pub struct Options {
    #[bpaf(short, long)]
    output: String,
    warn_on_spills: bool,
    #[bpaf(short, long)]
    verbose: bool,
    #[bpaf(external)]
    lineinfo: bool,
    #[bpaf(external)]
    gpu_name: String,
    #[bpaf(long, short('O'), fallback(3))]
    opt_level: usize,
    #[bpaf(positional)]
    input: String,
}

fn lineinfo() -> impl Parser<bool> {
    choice(["-lineinfo", "--lineinfo"].into_iter().map(|s| {
        literal(s)
            .anywhere()
            .optional()
            .map(|_| true)
            .fallback(false)
            .boxed()
    }))
}

// #[bpaf(long, long("gpu_name"), fallback_with(default_arch))]
fn gpu_name() -> impl Parser<String> {
    any("", move |s: String| {
        Some(
            s.strip_prefix("-arch=")
                .or_else(|| s.strip_prefix("--gpu-name="))?
                .to_owned(),
        )
    })
    .metavar(&[("--gpu-name=", Style::Literal), ("SM", Style::Metavar)])
    .anywhere()
    .fallback_with(|| Ok::<String, &'static str>("sm_52".to_string()))
}

fn main() {
    let options = options().run();
    std::fs::copy(&options.input, &options.output).unwrap();
}
