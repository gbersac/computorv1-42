#![feature(env)]

use std::env;

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
