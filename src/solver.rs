use solution::Solution;
use parser::Parser;
use std::fmt::{Formatter, Display, Error};
use std::fmt::Write;
use std;
use std::cmp::Ordering;
use x_part::{XPart};
use nbr_complex::NbrComplex;

pub struct Solver
{
    pub xs:             Vec<XPart>,
    pub discriminant:   Option<f32>,
    pub sol:            Solution,
    pub degree:         f32,
}

impl Solver
{
    pub fn new(equ_str: &str) -> Solver
    {
        let _xs = Parser::parse(&equ_str.to_string());
        let mut solver = Solver{
            xs: _xs,
            discriminant: None,
            sol: Solution::NoSolution,
            degree: 0.,
        };
        solver.solve();
        solver
    }

    fn analyze_xparts(&mut self) -> (f32, f32, f32)
    {
        let (mut a, mut b, mut c) = (0., 0., 0.);
        for x in &self.xs {
            match x.power {
                0. => c = x.multiply,
                1. => b = x.multiply,
                2. => a = x.multiply,
                _  => self.sol = Solution::NotComputable,
            };
            if self.degree < x.power{
                self.degree = x. power;
            }
        }
        (a, b, c)
    }

    fn solve_degree_0(&mut self, a: f32, b: f32, c: f32)
    {
        if c != 0.{
            self.sol = Solution::NoSolution
        }else{
            self.sol = Solution::Infinite
        }
    }

    fn solve_degree_1(&mut self, a: f32, b: f32, c: f32)
    {
        self.sol = Solution::Simple(-c / b);
    }

    fn solve_degree_2(&mut self, a: f32, b: f32, c: f32)
    {
        let discriminant = b * b - 4. * a * c;
        self.discriminant = Some(discriminant);
        match discriminant.partial_cmp(&0.).unwrap(){
            Ordering::Equal     => {
                let x = -b / (2. * a);
                self.sol = Solution::Simple(x);
            },
            Ordering::Greater   => {
                let x1 = (-b - discriminant.sqrt()) / (2. * a);
                let x2 = (-b + discriminant.sqrt()) / (2. * a);
                self.sol = Solution::Double(x1, x2);
            },
            Ordering::Less      => {
                let absolute = discriminant.abs().sqrt();
                self.sol = Solution::Complex(
                        NbrComplex::new(-b, absolute, 2. * a),
                        NbrComplex::new(-b, -absolute, 2. * a),
                        );
            },
        }
    }

    pub fn solve(&mut self)
    {
        let (a, b, c) = self.analyze_xparts();
        if self.degree == 0.{
            self.solve_degree_0(a, b, c);
        }else if self.degree == 1.{
            self.solve_degree_1(a, b, c);
        }else if self.degree == 2.{
            self.solve_degree_2(a, b, c);
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

    fn print_discriminant(&self)
    {
        if self.discriminant.is_none(){
            return ;
        }
        let discriminant = self.discriminant.unwrap();
        match discriminant.partial_cmp(&0.).unwrap(){
            Ordering::Equal     => {
                println!("Discriminant is equal to 0, the solution is:");
            },
            Ordering::Greater   => {
                println!("Discriminant is strictly positive, the two solutions are:");
            },
            Ordering::Less      => {
                println!("Discriminant is strictly negative, the two solutions are:");
            },
        }
    }

    /// Function to print a list of xparts as an  equation equaling 0.
    /// Return only for the tests
    pub fn print(&self) -> String
    {
        let mut degree = 0.;
        let mut to_return = String::new();
        let mut is_first = true;
        for x in &self.xs{
            if x.multiply != 0.{
                to_return.push_str(&Solver::xpart_to_string(&x, is_first));
                is_first = false;
            }
        }
        println!("{} = 0", to_return);
        println!("Polynomial degree: {}", self.degree);
        self.print_discriminant();
        println!("{}", self.sol);
        to_return
    }
}

#[cfg(test)]
mod test
{
    use super::*;
	use parser::Parser;
    use nbr_complex::NbrComplex;
    use solution::Solution;

    fn cmp_solve(equation: &str, sol: Solution)
    {
	    let solver = Solver::new(equation);
	    println!("{:?}", solver.xs);
	    println!("{:?} -> {:?} must be {:?}", 
	             equation, solver.sol, sol);
        assert!(solver.sol == sol);
    }

    #[test]
    fn test_solve()
    {
        // degree 0
	    cmp_solve("42 * X^0 = 42 * X^0", Solution::Infinite);
	    cmp_solve("4 * X^0 = 8 * X^0", Solution::NoSolution);
        // degree 1
        cmp_solve("10 * X^0 = 4 * X^0 + 3 * X^1", Solution::Simple(2.));
        cmp_solve("5 * X^0 + 4 * X^1 = 4 * X^0", Solution::Simple(-0.25));
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
        // degree 3
        cmp_solve("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0",
                  Solution::NotComputable);
    }

    fn cmp_print(l: &str, r: &str)
    {
        let solver = Solver::new(l);
        let s = solver.print();
        println!("result: {}", s);
        assert!(s == r);
    }

    #[test]
    fn test_print()
    {
        cmp_print("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0",
                         "4 + 4 * X + -9.3 * X^2");
        cmp_print("5 * X^0 + 4 * X^1 = 4 * X^0",
                         "1 + 4 * X");
        cmp_print("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0",
                         "5 + -6 * X + -5.6 * X^3");
    }
}
