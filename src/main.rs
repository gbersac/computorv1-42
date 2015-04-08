#![feature(env)]
#![feature(core)]
#![feature(collections)]

#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::env;

mod tokenizer;
mod equation;
mod fc_string;
mod x_part;

fn get_equation() -> String
{
    let args : Vec<String> = env::args().collect();
    if args.len() != 2{
        panic!("Argument error");
    }
    args[1].clone()
}

fn main() 
{
    let equation_str = get_equation();
    println!("{}", equation_str);
}
