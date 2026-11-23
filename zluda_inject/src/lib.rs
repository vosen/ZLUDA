#[macro_use]

mod win;
pub mod args;
mod bin;

use std::error::Error;

pub fn run(args: args::Arguments, detached: bool) -> Result<(), Box<dyn Error>> {
    bin::main_impl(args, detached)
}
