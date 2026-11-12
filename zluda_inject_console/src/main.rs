use bpaf::Parser;

fn main() {
    zluda_inject::run(zluda_inject::args::arguments().to_options().run(), false).unwrap();
}
