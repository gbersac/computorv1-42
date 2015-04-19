use tokenizer::{Tokenizer, TokenInfo, Token};
use regex::{Regex};
use x_part::{XPart};
use fc_string;

#[derive(PartialEq, Clone)]
pub enum TokenType
{
	NUMBER,
	MULTIPLY,
	X_OPERAND,
	ADD_SUB,
	EQUAL,
}

pub struct Parser;

impl Parser
{
	fn split(to_parse: &String) -> Vec<Token<TokenType>>
	{
		let token_types = vec![
			TokenInfo::new(TokenType::X_OPERAND, regex!("X *\\^ *[0-9]")),
			TokenInfo::new(TokenType::NUMBER, regex!("[0-9]+\\.?[0-9]*")),
			TokenInfo::new(TokenType::MULTIPLY, regex!("\\*")),
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

    /// Return a list of Xpart which has been changed so that rx is
    /// now equal to 0.
    fn reduce(lxs: &Vec<XPart>, rxs: &Vec<XPart>) -> Vec<XPart>
    {
        let mut to_return = lxs.clone();
        for rx in rxs{
            let mut found = false; //has power equivalent to rx been found
            //search for XPart in left which have the same power 
            //as rx to reduce
            for lx in &mut to_return{
                if lx.power == rx.power{
                    let buff = lx.multiply - rx.multiply;
                    lx.multiply = buff;
                    found = true;
                }
            }
            //if no power equivalent in left add new XPart in left
            if !found{
                let mut x = rx.clone();
                x.multiply = -x.multiply;
                to_return.push(x);
            }
        }
        to_return
    }

	/// Parse the string into an equation and reduce it.
	pub fn parse(to_parse: &String) -> Vec<XPart>
	{
		//split string into tokens
		let tokens = Parser::split(to_parse);
		//split tokens into x_parts
		let (ltokens, rtokens) = Parser::split_tokens(tokens);
		let lx = Parser::to_xparts(ltokens);
		let rx = Parser::to_xparts(rtokens);
		//reduce equation (make it equal to zero)
	    Parser::reduce(&lx, &rx)
	}
}

#[test]
fn test_equation_tokenizer()
{
	//basic test
	let test1 = Parser::split(&"42 * X^0 = 0".to_string());
	println!("{:?}", test1);
	assert!(test1.len() == 5 &&
			*(test1[0].get_type()) == TokenType::NUMBER &&
			*(test1[1].get_type()) == TokenType::MULTIPLY &&
			*(test1[2].get_type()) == TokenType::X_OPERAND &&
			*(test1[3].get_type()) == TokenType::EQUAL &&
			*(test1[4].get_type()) == TokenType::NUMBER);

	//more complex test
	let test1 = Parser::split(&"1.0 * X^0 + 5.7 * X^1 = 0".to_string());
	println!("{:?}", test1);
	assert!(test1.len() == 9 &&
			*(test1[0].get_type()) == TokenType::NUMBER &&
			*(test1[1].get_type()) == TokenType::MULTIPLY &&
			*(test1[2].get_type()) == TokenType::X_OPERAND &&
			*(test1[3].get_type()) == TokenType::ADD_SUB &&
			*(test1[4].get_type()) == TokenType::NUMBER);
}

#[test]
fn test_XPart()
{
	let test1 = XPart::from_tokens(&vec![
			Token::new(TokenType::NUMBER, "3".to_string())]);
	println!("{:?}", test1);
	assert!(test1.power == 0. && test1.multiply == 3.);

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
	let tokens1 = Parser::split(&"3 * X^5".to_string());
	let test1 = Parser::to_xparts(tokens1);
	println!("{:?}", test1);
	assert!(test1[0].test_values(3., 5.));

	let tokens2 = Parser::split(&"3 * X^4 + 5 * X^6".to_string());
	let test2 = Parser::to_xparts(tokens2);
	println!("{:?}", test2);
	assert!(test2[0].test_values(3., 4.));
	assert!(test2[1].test_values(5., 6.));

	let tokens3 = Parser::split(&"3 * X^4 - 5 * X^6".to_string());
	let test3 = Parser::to_xparts(tokens3);
	println!("{:?}", test3);
	assert!(test3[0].test_values(3., 4.));
	assert!(test3[1].test_values(-5., 6.));

	let tokens4 = Parser::split(&"2 - 3 * X^1".to_string());
	let test4 = Parser::to_xparts(tokens4);
	println!("{:?}", test4);
	assert!(test4[0].test_values(2., 0.) &&
	        test4[1].test_values(-3., 1.));
}

#[test]
fn test_reduce()
{
	let tokens1 = Parser::split(&"2 + 3 * X^1 + 4 * X^2".to_string());
	let x1      = Parser::to_xparts(tokens1);
	let tokens2 = Parser::split(&"0".to_string());
	let x2      = Parser::to_xparts(tokens2);
	let tokens3 = Parser::split(&"1 + 5 * X^1".to_string());
	let x3      = Parser::to_xparts(tokens3);
	let tokens4 = Parser::split(&"2 - 3 * X^1".to_string());
	let x4      = Parser::to_xparts(tokens4);

    let test1 = Parser::reduce(&x1, &x2);
	println!("{:?}", test1);
    assert!(test1.len() == 3 &&
           test1[0].test_values(2., 0.) &&
           test1[1].test_values(3., 1.) &&
           test1[2].test_values(4., 2.));

    let test2 = Parser::reduce(&x1, &x3);
	println!("{:?}", test2);
    assert!(test2.len() == 3 &&
           test2[0].test_values(1., 0.) &&
           test2[1].test_values(-2., 1.) &&
           test2[2].test_values(4., 2.));

    println!("###########################");
	println!("x4{:?}", x4);
    let test2 = Parser::reduce(&x1, &x4);
	println!("{:?}", test2);
    assert!(test2.len() == 3 &&
           test2[0].test_values(0., 0.) &&
           test2[1].test_values(6., 1.) &&
           test2[2].test_values(4., 2.));
}

