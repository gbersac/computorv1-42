use tokenizer::{Tokenizer, TokenInfo};

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
            TokenInfo::new(TokenType::NUMBER, &"[0-1]+.?[0-1]+".to_string()),
        ];
        Tokenizer::new(token_types);
    }
}
