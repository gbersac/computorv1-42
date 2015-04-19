#![feature(env)]
#![feature(core)]
#![feature(collections)]

#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use parser::Parser;
use solver::Solver;

mod tokenizer;
mod parser;
mod fc_string;
mod x_part;
mod solver;
mod nbr_complex;
mod solution;

fn get_equation() -> String
{
    let args : Vec<String> = std::env::args().collect();
    if args.len() != 2{
        panic!("Argument error");
    }
    args[1].clone()
}

fn main() 
{
    let equation_str = get_equation();
    let solver = Solver::new(&equation_str);
    solver.print();
}
