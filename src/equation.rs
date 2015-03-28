use tokenizer::{Tokenizer, TokenInfo, Token};
use regex::{Regex};

#[derive(PartialEq, Clone)]
enum TokenType
{
	NUMBER,
	MULTIPLY,
	X_OPERAND,
	ADD_SUB,
	EQUAL,
}

struct Equation;

impl Equation
{
	fn split(to_parse: &String) -> Vec<Token<TokenType>>
	{
		let token_types = vec![
			TokenInfo::new(TokenType::NUMBER, regex!("[0-9]+.?[0-9]*")),
			TokenInfo::new(TokenType::MULTIPLY, regex!("\\*")),
			TokenInfo::new(TokenType::X_OPERAND, regex!("X *\\^ *[0-9]")),
			TokenInfo::new(TokenType::ADD_SUB, regex!("[+-]")),
			TokenInfo::new(TokenType::EQUAL, regex!("=")),
		];
		let tokenizer = Tokenizer::new(token_types);
		tokenizer.split(to_parse)
	}

	pub fn parse(to_parse: &String)
	{

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
