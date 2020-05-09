#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate log;

pub mod ast;
pub mod error;
mod lexer;
mod location;
pub mod parser;
mod token;

lalrpop_mod!(n3); // synthesized by LALRPOP
