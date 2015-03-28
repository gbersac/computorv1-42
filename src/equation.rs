use tokenizer::{Tokenizer, TokenInfo};
use regex::{Regex};

#[derive(Clone)]
enum TokenType
{
    NUMBER,
    MULTIPLY,
    NONE,
}

struct Equation;

impl Equation
{
    fn parse()
    {
        let token_types = vec![
            TokenInfo::new(TokenType::NUMBER, regex!("[0-1]+.?[0-1]+")),
        ];
        Tokenizer::new(token_types);
    }
}
