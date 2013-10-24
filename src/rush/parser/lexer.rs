
pub enum Token
{
    String(~str, bool),
    Pipe,
    EOF,
}

pub enum LexerErr
{
    UnknownChar(char),
}

struct Lexer
{
    data: ~str,
    index: uint,
}

impl Lexer
{
    pub fn new(data: ~str) -> Lexer
    {
        Lexer {
            data: data,
            index: 0,
        }
    }

    pub fn consume() -> Result<Token, LexerErr>
    {
        Ok(EOF)
    }
}
