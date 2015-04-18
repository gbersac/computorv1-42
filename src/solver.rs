use std::fmt::Write;
// use std::io::Write;
use std;
use std::cmp::Ordering;
use x_part::{XPart};

pub struct Solver;

#[derive(PartialEq, Clone)]
pub enum Solution
{
    /// If discriminant = 0
    Simple(f32),
    /// If discriminant > 0
    Double(f32, f32),
    /// If discriminant < 0
    Complex(f32, f32),
    /// If 0 = 0
    Infinite,
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

    pub fn solve(xs: &Vec<XPart>) -> Solution
    {
        let (a, b, c) = Solver::analyze_xparts(xs);
        if a == 0. && b == 0. && c == 0.{
            return Solution::Infinite;
        }
        let discriminant = b * b - 4. * a * c;
        match discriminant.partial_cmp(&0.).unwrap(){
            Ordering::Equal     => {
                Solution::Simple(b * b - 4. * a * c)
            },
            Ordering::Greater   => {
                let x1 = (-b - discriminant.sqrt()) / (2. * a);
                let x2 = (-b + discriminant.sqrt()) / (2. * a);
                Solution::Double(x1, x2)
            },
            Ordering::Less      => {
                Solution::Complex(0., 0.)
            },
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

    /// Function to print a list of xparts as an  equation equaling 0.\
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
	use equation::Equation;

    fn get_solution(equation: &str) -> Solution
    {
	    let x = Equation::parse(&equation.to_string());
	    Solver::solve(&x)
    }

    #[test]
    fn test_solve()
    {
        let test1 = get_solution("6 + 1 * X^1 - 1 * X^2 = 0");
        assert!(test1 == Solution::Double(3., -2.));
        let test2 = get_solution("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0");
        // assert!(test2 == );
	    let test2 = get_solution("5 * X^0 + 4 * X^1 = 4 * X^0");
	    let test3 = get_solution("42 * X^0 = 42 * X^0");
        assert!(test3 == Solution::Infinite);
    }

    fn cmp_print_xparts(l: &str, r: &str)
    {
        let xs = Equation::parse(&l.to_string());
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
}
