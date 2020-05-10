use std::iter;

use crate::ast;
use crate::error::ParseError;
use crate::lexer;
use crate::n3;
use crate::token;

macro_rules! do_lalr_parsing {
    ($input: expr, $pat: ident, $tok: ident) => {{
        let lxr = lexer::make_tokenizer($input);
        let marker_token = (Default::default(), token::Tok::$tok, Default::default());
        let tokenizer = iter::once(Ok(marker_token)).chain(lxr);

        match n3::$pat::new().parse(tokenizer) {
            Err(err) => Err(ParseError::from(err)),
            Ok(tree) => Ok(tree),
        }
    }};
}

pub fn parse_file(source: &str) -> Result<ast::File, ParseError> {
    do_lalr_parsing!(source, InputFileParser, StartFile)
}
