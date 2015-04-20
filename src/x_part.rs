use tokenizer::{Token};
use parser::{TokenType};
use fc_string;

/// This is the representation of a "a * X^p"
/// a being multiply
/// p being the power
#[derive(Debug, Clone)]
pub struct XPart
{
	pub power:      f32,
	pub multiply:   f32,
}

impl XPart
{
	// pub fn new(mul: f32, pow: f32) -> XPart
	// {
	//     XPart{
	//         multiply: mul,
	//         power: pow,
	//     }
	// }

	fn extract_power(xstr: &String) -> f32
	{
		for (i, c) in xstr .chars().enumerate() {
			match c {
				'0'...'9' => {
					let sstr = fc_string::sub_string(xstr, i, xstr.len() - i)
						.unwrap();
					return sstr.parse::<f32>().unwrap_or(1.);
				},
				_ => {}
			}
		}
		1.
	}

	pub fn from_tokens(tokens: &Vec<Token<TokenType>>) -> XPart
	{
		let mut pow = 0.;
		let mut mul = 1.;
		let mut is_negative = false;
		for tok in tokens.iter(){
			match *tok.get_type(){
				TokenType::AddSub => {
					if tok.get_content() == "-" {
						is_negative = true;
					}
				}
				TokenType::Number => {
					mul = tok.get_content().parse::<f32>().unwrap();
					if is_negative{
						mul = - mul;
					}
				},
				TokenType::XOperand => {
					pow = XPart::extract_power(tok.get_content());
				},
				_ => {},
			}
		}
		XPart{power: pow, multiply: mul}
	}

    /// Used in tests.
    #[allow(dead_code)]
    pub fn test_values(&self, mul: f32, pow: f32) -> bool
    {
        self.multiply == mul && self.power == pow
    }
}

#[test]
fn test_extract_power()
{
	assert!(XPart::extract_power(&"X^3".to_string()) == 3.);
	assert!(XPart::extract_power(&"X^1".to_string()) == 1.);
	assert!(XPart::extract_power(&"X^100".to_string()) == 100.);
	assert!(XPart::extract_power(&"X".to_string()) == 1.);
	assert!(XPart::extract_power(&"X^".to_string()) == 1.);
}
