macro_rules! err {
    ($p:expr, $kind:ident) => {
        Err($p.error($p.pos, SexParserErrorKind::$kind))
    };
    ($p:expr, $kind:ident($($arg:tt)*)) => {
        Err($p.error($p.pos, SexParserErrorKind::$kind($($arg)*)))
    };
    ($p:expr, $kind:ident { $($arg:tt)* }) => {
        Err($p.error($p.pos, SexParserErrorKind::$kind { $($arg)* }))
    };
}

mod atom;
mod parser;
mod parser_data;
mod sex_trait;
mod view;

pub use atom::{Atom, AtomTy, List, Number, SexError, Text, TextTy};
pub use parser::{parse_expression_str, parse_expression_file, parse_expression_reader, parse_exprlist_str, parse_exprlist_file, parse_exprlist_reader};
pub use parser_data::{MalformedHexCode, Parser, Position, SexParserAtomError, SexParserError, SexParserErrorKind};
pub use sex_trait::FromSex;
pub use view::{AtomView, KeywordView};

#[cfg(feature = "derive")]
pub use sex_derive::Sex;
