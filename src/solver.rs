use std::fmt::Write;
// use std::io::Write;
use std;
use std::cmp::Ordering;
use x_part::{XPart};
use nbr_complex::NbrComplex;

pub struct Solver;

#[derive(PartialEq, Clone, Debug)]
pub enum Solution
{
    /// If discriminant = 0
    Simple(f32),
    /// If discriminant > 0
    Double(f32, f32),
    /// If discriminant < 0
    Complex(NbrComplex, NbrComplex),
    /// If 0 = 0
    Infinite,
    /// If a = b (where a and b are scalar)
    NoSolution,
}

impl Solver
{
    fn analyze_xparts(xs: &Vec<XPart>) -> (f32, f32, f32)
    {
        let (mut a, mut b, mut c) = (0., 0., 0.);
        for x in xs {
            match x.power {
                0. => c = x.multiply,
                1. => b = x.multiply,
                2. => a = x.multiply,
                _ => panic!("This equation is more than degree 2"),
            };
        }
        (a, b, c)
    }

    fn solve_degree_0(a: f32, b: f32, c: f32) -> Solution
    {
        if c != 0.{
            Solution::NoSolution
        }else{
            Solution::Infinite
        }
    }

    fn solve_degree_1(a: f32, b: f32, c: f32) -> Solution
    {
        Solution::Simple(c / b)
    }

    fn solve_degree_2(a: f32, b: f32, c: f32) -> Solution
    {
        let discriminant = b * b - 4. * a * c;
        match discriminant.partial_cmp(&0.).unwrap(){
            Ordering::Equal     => {
                let x = -b / (2. * a);
                Solution::Simple(x)
            },
            Ordering::Greater   => {
                let x1 = (-b - discriminant.sqrt()) / (2. * a);
                let x2 = (-b + discriminant.sqrt()) / (2. * a);
                Solution::Double(x1, x2)
            },
            Ordering::Less      => {
                let absolute = discriminant.abs().sqrt();
                Solution::Complex(
                        NbrComplex::new(-b, absolute, 2. * a),
                        NbrComplex::new(-b, -absolute, 2. * a),
                        )
            },
        }
    }

    pub fn solve(xs: &Vec<XPart>) -> Solution
    {
        let (a, b, c) = Solver::analyze_xparts(xs);
        println!("a {}, b {}, c {}", a, b, c);
        if a == 0. && b == 0.{
            Solver::solve_degree_0(a, b, c)
        }else if a == 0.{
            Solver::solve_degree_1(a, b, c)
        }else{
            Solver::solve_degree_2(a, b, c)
        }
    }

    fn xpart_to_string(x: &XPart, is_first: bool) -> String
    {
        let mut to_return = String::new();
        if !is_first{
            write!(&mut to_return, " + ");
        }
        match x.power{
            0. => {
                write!(&mut to_return, "{}", x.multiply);
            },
            1. => {
                write!(&mut to_return, "{} * X", x.multiply);
            },
            _  => {
                write!(&mut to_return, "{} * X^{}", x. multiply, x.power);
            },
        }
        to_return
    }

    /// Function to print a list of xparts as an  equation equaling 0.
    /// Return only for the tests
    pub fn print_xparts(xs: &Vec<XPart>) -> String
    {
        let mut degree = 0;
        let mut to_return = String::new();
        let mut is_first = true;
        for x in xs{
            if x.multiply != 0.{
                to_return.push_str(&Solver::xpart_to_string(x, is_first));
                is_first = false;
            }
        }
        println!("{} = 0", to_return);
        println!("Polynomial degree: {}", degree);
        to_return
    }
}

#[cfg(test)]
mod test
{
    use super::*;
	use parser::Parser;
    use nbr_complex::NbrComplex;

    fn cmp_solve(equation: &str, sol: Solution)
    {
	    let x = Parser::parse(&equation.to_string());
	    let result = Solver::solve(&x);
	    println!("{:?} -> {:?} must be {:?}", 
	             equation, result, sol);
        assert!(result == sol);
    }

    #[test]
    fn test_solve()
    {
        // degree 0
	    cmp_solve("42 * X^0 = 42 * X^0", Solution::Infinite);
	    cmp_solve("4 * X^0 = 8 * X^0", Solution::NoSolution);
        // degree 1
        cmp_solve("10 * X^0 = 4 * X^0 + 3 * X^1", Solution::Simple(2.));
        // degree 2
        cmp_solve("6 + 1 * X^1 - 1 * X^2 = 0", Solution::Double(3., -2.));
        cmp_solve("6 * X^0 + 11 * X^1 + 5 * X^2 = 1 * X^0 + 1 * X^1", Solution::Simple(-1.));
        // cmp_solve("5 * X^0 + 13 * X^1 + 3 * X^2 = 1 * X^0 + 1 * X^1", 
        //         Solution::Double(-3.632993, -0.367007));
        cmp_solve("3 * X^0 + 6 * X^1 + 5 * X^2 = 1 * X^0",
                 Solution::Complex(
                     NbrComplex::new(-6., 2., 10.),
                     NbrComplex::new(-6., -2., 10.),
                     )
                 );
    }

    fn cmp_print_xparts(l: &str, r: &str)
    {
        let xs = Parser::parse(&l.to_string());
        let s = Solver::print_xparts(&xs);
        assert!(s == r);
    }

    #[test]
    fn test_print_xparts()
    {
        cmp_print_xparts("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0",
                         "4 + 4 * X + -9.3 * X^2");
        cmp_print_xparts("5 * X^0 + 4 * X^1 = 4 * X^0",
                         "1 + 4 * X");
        cmp_print_xparts("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0",
                         "5 + -6 * X + -5.6 * X^3");
    }

    #[test]
    #[should_panic]
    fn degree3()
    {
        cmp_solve("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0",
                 Solution::NoSolution);
    }
}
