use super::lexer::*;

pub enum Command
{
    Simple(~[~str]),
    NoCommand,
}

pub enum ParserErr
{
    SyntaxError(~str),
    LexerError(LexerErr),
}

struct Parser
{
    command: Option<Command>
}

impl Parser
{
    pub fn new() -> Parser
    {
        Parser {
            command: None
        }
    }

    pub fn parse(&self, line: ~str) -> Result<Command, ParserErr>
    {
        let lexer = Lexer::new(line);

        Ok(NoCommand)
    }
}
