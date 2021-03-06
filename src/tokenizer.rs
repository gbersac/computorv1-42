use regex::{Regex};
use fc_string;
use std::fmt::{Formatter, Error, Debug};

pub struct Token<E: Clone>
{
	/// The type of the token. One of the item of the enum type E
	t_type:     E,

	/// String which apply for this token
	content:    String,
}

impl<E: Clone> Token<E>
{
	pub fn new(_t_type: E, _content: String) -> Token<E>
	{
		Token{
			t_type: _t_type,
			content: _content,
		}
	}

	pub fn get_type(&self) -> &E
	{
		&self.t_type
	}

	pub fn get_content(&self) -> &String
	{
		&self.content
	}
}

impl<E: Clone> Debug for Token<E>
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		let mut result = Ok(());
		result = result.and(write!(f, "{}", self.content));
		result
	}
}


/// A token info is a definition of a token type which can be generated
/// by a tokenizer.
///
/// The generic type E is an enum which list token type. An instance of TokenInfo
/// is a description of one of the item of E.
pub struct TokenInfo<E: Clone>
{
	/// The type of the item as listed in the enum E.
	token_type: E,

	/// The regex which define the form of this token.
	re:         Regex,
}

impl<E: Clone> TokenInfo<E>
{
	pub fn new(type_t: E, reg: Regex) -> TokenInfo<E>
	{
		TokenInfo{
			token_type: type_t,
			re: reg,
		}
	}

	/// Return the token corresponding to this type of token if the string to_match
	/// is corresponding to this type of token.
	///
	/// Also return to_match without the content of the token.
	pub fn is_match(&self, to_match: &String) -> Option<Token<E>>
	{
		if self.re.is_match(to_match){
			let pos = self.re.find(&to_match);
			if pos.is_some(){
				let (begin, end) = pos.unwrap();
				if begin == 0{
					let content = fc_string::sub_string(to_match, begin, end)
							.unwrap();
					let tok = Token::new(self.token_type.clone(),
							content.to_string());
					return Some(tok);
				}
			}
		}
		None
	}

}

/// This structure is initialized with the types of token to extract.
///
/// We can then use the function parse to split any given string in token
/// according to the description of the token types provided at the
/// initialization of the struct.
///
/// The generic type E is an enum which list of the types of token that can be
/// generated by this instance of tokenizer.
pub struct Tokenizer<E: Clone>
{
	/// This vector contain the definition of the differents type of token
	/// that can be generated by this tokenizer.
	types_info: Vec<TokenInfo<E>>,

	/// List of all the characters which have to be discarded.
	discards:   Regex,
}

impl<E: Clone> Tokenizer<E>
{
	pub fn new(infos: Vec<TokenInfo<E>>) -> Tokenizer<E>
	{
		Tokenizer{
			types_info:	infos,
			discards:	regex!("[ \t\n]+"),
		}
	}

	/// Check if the string to_split match any of the registered token type.
	/// At least one type of token should match this string.
	fn match_token(&self, to_split: &mut String) -> Option<Token<E>>
	{
		for tt in self.types_info.iter(){
			let tok = tt.is_match(to_split);
			if tok.is_some(){
				return Some(tok.unwrap());
			}
		}
		None
	}

	fn reduce_to_split(&self, to_split: &String, token_len: usize) -> String
	{
		let reduced = fc_string::sub_string(to_split, token_len, to_split.len() - token_len)
				.unwrap();
		self.discards.replace(&reduced, "")
	}

	/// This function will take the string to_parse and return the list of
	/// token corresponding.
	pub fn split(&self, to_split_model: &String) -> Vec<Token<E>>
	{
		let mut tokens = Vec::new();
		let mut to_split = to_split_model.clone();

		while !to_split.is_empty() {
			let tok = self.match_token(&mut to_split)
					.expect("this string does not match any token type");
			to_split = self.reduce_to_split(&to_split, tok.get_content().len());
			tokens.push(tok);
		}
		tokens
	}
}

#[test]
fn test_tokenizer()
{
	#[derive(Clone)]
	enum TokenType
	{
		AAA,
		BBB,
		NONE,
	}

	let token_types = vec![
		TokenInfo::new(TokenType::AAA, regex!("aaa")),
		TokenInfo::new(TokenType::BBB, regex!("bbb")),
		TokenInfo::new(TokenType::NONE, regex!("[^ \n\t]+")),
	];
	let tokenizer = Tokenizer::new(token_types);

	// basic
	let vec_token = tokenizer.split(&"aaa".to_string());
	println!("{:?}", vec_token);
	assert_eq!(vec_token.len(), 1);

	// multiple tokens
	let vec_token = tokenizer.split(&"aaabbb".to_string());
	println!("{:?}", vec_token);
	assert_eq!(vec_token.len(), 2);

	// discarded char
	let vec_token = tokenizer.split(&"aaa \n\t bbb".to_string());
	println!("{:?}", vec_token);
	assert_eq!(vec_token.len(), 2);

	// reverse order
	let vec_token = tokenizer.split(&"bbbaaa".to_string());
	println!("{:?}", vec_token);
	assert!(vec_token.len() == 2 &&
			vec_token[0].get_content() == "bbb" &&
			vec_token[1].get_content() == "aaa");

	// unknow token
	let vec_token = tokenizer.split(&"bbbaaac".to_string());
	println!("{:?}", vec_token);
	assert!(vec_token.len() == 3 &&
			vec_token[0].get_content() == "bbb" &&
			vec_token[1].get_content() == "aaa" &&
			vec_token[2].get_content() == "c");
}
