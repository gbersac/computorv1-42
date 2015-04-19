use nbr_complex::NbrComplex;
use std::fmt::{Formatter, Display, Error};

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
    /// When degree > 2
    NotComputable,
}

impl Display for Solution
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        let mut result = Ok(());
        match *self{
            Solution::Simple(ref x)       =>{
                result = result.and(write!(f, "The solution is: {}", x));
            },
            Solution::Double(ref x1, ref x2)  =>{
                result = result.and(write!(f, "The two solutions are: {}, {}", 
                                           x1, x2));
            },
            Solution::Complex(ref x1, ref x2) =>{
                result = result.and(write!(f, "The two solutions are: {}, {}", 
                                           x1, x2));
            },
            Solution::Infinite        =>{
                result = result.and(write!(f, "There is an infinite number of solution."));
            },
            Solution::NoSolution      =>{
                result = result.and(write!(f, "There is no solution."));
            }
            Solution::NotComputable   =>{
                result = result.and(write!(f, 
                        "The polynomial degree is stricly greater than 2, I can't solve."));
            }
        }
        result
    }
}

