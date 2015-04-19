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
