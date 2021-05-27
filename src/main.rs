#![allow(unused)]
#[macro_use]
mod macros;
mod functions;
pub mod environments;
pub mod error;
pub mod dimensions;
pub mod layout;
pub mod ast;
pub mod render;
pub mod font;
pub mod dev;

fn main() {
    // dev::dev();
    let res = ast::symbols::Symbol::from_name("overbrace").unwrap();
    println!("res: {:#?}", res);
}
