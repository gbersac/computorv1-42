use tokenizer::{Tokenizer, TokenInfo, Token};
use regex::{Regex};
use fc_string;

#[derive(PartialEq, Clone)]
enum TokenType
{
	NUMBER,
	MULTIPLY,
	X_OPERAND,
	ADD_SUB,
	EQUAL,
}

/// This is the representation of a "a * X^p"
/// a being multiply
/// p being the power
#[derive(Debug)]
struct XPart
{
	power:      f32,
	multiply:   f32,
}

impl XPart
{
	fn extract_power(xstr: &String) -> f32
	{
		for (i, c) in xstr .chars().enumerate() {
			match c {
				'0'...'9' => {
					let sstr = fc_string::sub_string(xstr, i, xstr.len() - i)
						.unwrap();
					println!("{:?}", sstr);
					return sstr.parse::<f32>().unwrap_or(1.);
				},
				_ => {}
			}
		}
		1.
	}

	fn from_tokens(tokens: &Vec<Token<TokenType>>) -> XPart
	{
		let mut pow = 1.;
		let mut mul = 1.;
		let mut is_negative = false;
		for tok in tokens.iter(){
			match *tok.get_type(){
				TokenType::ADD_SUB => {
					if tok.get_content().as_slice() == "-" {
						is_negative = true;
					}
				}
				TokenType::NUMBER => {
					mul = tok.get_content().parse::<f32>().unwrap();
					if is_negative{
						mul = - mul;
					}
				},
				TokenType::X_OPERAND => {
					pow = XPart::extract_power(tok.get_content());
				},
				_ => {},
			}
		}
		XPart{power: pow, multiply: mul}
	}
}

pub struct Equation;

impl Equation
{
	fn split(to_parse: &String) -> Vec<Token<TokenType>>
	{
		let token_types = vec![
			TokenInfo::new(TokenType::NUMBER, regex!("[0-9]+\\.?[0-9]*")),
			TokenInfo::new(TokenType::MULTIPLY, regex!("\\*")),
			TokenInfo::new(TokenType::X_OPERAND, regex!("X *\\^ *[0-9]")),
			TokenInfo::new(TokenType::ADD_SUB, regex!("[+-]")),
			TokenInfo::new(TokenType::EQUAL, regex!("=")),
		];
		let tokenizer = Tokenizer::new(token_types);
		tokenizer.split(to_parse)
	}

	fn split_tokens(tokens: Vec<Token<TokenType>>)
			-> (Vec<Token<TokenType>>, Vec<Token<TokenType>>)
	{
		let mut tok_buffer: Vec<Token<TokenType>> = Vec::new();
		// let to_return = Vec::new();
		let mut ltokens = Vec::new();
		let mut rtokens = Vec::new();
		let mut left = true;
		for tok in tokens{
			if *tok.get_type() == TokenType::EQUAL{
				left = false;
			}else if left{
				ltokens.push(tok);
			}else{
				rtokens.push(tok);
			}
		}
		(ltokens, rtokens)
	}

	fn to_xparts(tokens: Vec<Token<TokenType>>) -> Vec<XPart>
	{
		let mut tok_buffer = Vec::new();
		let mut to_return = Vec::new();
		for tok in tokens{
			if *tok.get_type() == TokenType::ADD_SUB{
				to_return.push(XPart::from_tokens(&tok_buffer));
				tok_buffer.push(tok);
			}else{
				tok_buffer.push(tok);
			}
		}
		to_return.push(XPart::from_tokens(&tok_buffer));
		to_return
	}

	pub fn parse(to_parse: &String)
	{
		//split string into tokens
		let tokens = Equation::split(to_parse);
		//split tokens into x_parts
		let (ltokens, rtokens) = Equation::split_tokens(tokens);
		let lx = Equation::to_xparts(ltokens);
		let rx = Equation::to_xparts(rtokens);
		//reduce equation (make it equal to zero)
	}

	pub fn solve()
	{

	}
}

#[test]
fn test_equation_tokenizer()
{
	//basic test
	let test1 = Equation::split(&"42 * X^0 = 0".to_string());
	println!("{:?}", test1);
	assert!(test1.len() == 5 &&
			*(test1[0].get_type()) == TokenType::NUMBER &&
			*(test1[1].get_type()) == TokenType::MULTIPLY &&
			*(test1[2].get_type()) == TokenType::X_OPERAND &&
			*(test1[3].get_type()) == TokenType::EQUAL &&
			*(test1[4].get_type()) == TokenType::NUMBER);

	//more complex test
	let test1 = Equation::split(&"1.0 * X^0 + 5.7 * X^1 = 0".to_string());
	println!("{:?}", test1);
	assert!(test1.len() == 9 &&
			*(test1[0].get_type()) == TokenType::NUMBER &&
			*(test1[1].get_type()) == TokenType::MULTIPLY &&
			*(test1[2].get_type()) == TokenType::X_OPERAND &&
			*(test1[3].get_type()) == TokenType::ADD_SUB &&
			*(test1[4].get_type()) == TokenType::NUMBER);
}

#[test]
fn test_solve()
{
	// 5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0
	// 5 * X^0 + 4 * X^1 = 4 * X^0
	// 42 ∗ X^0 = 42 ∗ X^0
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

#[test]
fn test_XPart()
{
	let test1 = XPart::from_tokens(&vec![
			Token::new(TokenType::NUMBER, "3".to_string())]);
	println!("{:?}", test1);
	assert!(test1.power == 1. && test1.multiply == 3.);

	let test2 = XPart::from_tokens(&vec![
			Token::new(TokenType::NUMBER, "3".to_string()),
			Token::new(TokenType::MULTIPLY, "*".to_string()),
			Token::new(TokenType::X_OPERAND, "X^5".to_string())]);
	println!("{:?}", test2);
	assert!(test2.power == 5. && test2.multiply == 3.);

	let test3 = XPart::from_tokens(&vec![
			Token::new(TokenType::X_OPERAND, "X^5".to_string()),
			Token::new(TokenType::MULTIPLY, "*".to_string()),
			Token::new(TokenType::NUMBER, "3".to_string())]);
	println!("{:?}", test3);
	assert!(test3.power == 5. && test2.multiply == 3.);
}

#[test]
fn test_to_xparts()
{
	let tokens1 = Equation::split(&"3 * X^5".to_string());
	let test1 = Equation::to_xparts(tokens1);
	println!("{:?}", test1);
	assert!(test1[0].power == 5. &&
			test1[0].multiply == 3.);

	let tokens2 = Equation::split(&"3 * X^4 + 5 * X^6".to_string());
	let test2 = Equation::to_xparts(tokens2);
	println!("{:?}", test2);
	assert!(test2[0].power == 4. &&
			test2[0].multiply == 3.);
	assert!(test2[1].power == 6. &&
			test2[1].multiply == 5.);

	let tokens3 = Equation::split(&"3 * X^4 - 5 * X^6".to_string());
	let test3 = Equation::to_xparts(tokens3);
	println!("{:?}", test3);
	assert!(test3[0].power == 4. &&
			test3[0].multiply == 3.);
	assert!(test3[1].power == 6. &&
			test3[1].multiply == -5.);
}



