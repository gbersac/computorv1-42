use std::fmt::{Formatter, Display, Error};

#[derive(PartialEq, Clone, Debug)]
pub struct NbrComplex
{
    pub scalar: f32,
    pub i_mul:  f32,
    pub div:    f32,
}

impl NbrComplex
{
    pub fn new(s: f32, i: f32, div: f32) -> NbrComplex
    {
        NbrComplex{
            scalar: s,
            i_mul:  i,
            div:    div,
        }
    }
}

impl Display for NbrComplex
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
    {
        let mut result = Ok(());
        result = result.and(write!(f, "({} + {}i) / {}", 
                                   self.scalar, self.i_mul, self.div));
        result
    }
}
