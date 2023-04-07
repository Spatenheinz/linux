#![allow(warnings, unused)]
//! Virtual Device Module
use kernel::prelude::*;
use kernel::Module;
// use kernel::{chrdev, file};
// use alloc::alloc::Layout;
use core::cell;
use kernel::sync::Ref;
extern crate nom;

use nom::{
    IResult,
    bytes::complete::tag,
};

#[macro_use]
extern crate alloc;


module! {
    type: Checker,
    name: "checker",
    author: "Jacob Herbst",
    description: "an in kernel proof checker for eBPF",
    license: "GPL",
}

struct Checker {
    numbers: Vec<i32>,
}

impl Module for Checker {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        // Print a banner to make sure our module is working
        pr_info!("------------------------\n");
        pr_info!("starting checker virtual device!\n");
        pr_info!("------------------------\n");
        let mut nums = Ref::try_new(Vec::new());
        let mut trynums = try_vec![1,2,3];
        // let nums = vec![];
        nums.try_push(42)?;
        nums.try_push(1337)?;
        let nomtest : IResult<&str, &str> = tag("#")("#nice");
        pr_info!("{:?}", &nums);
        pr_info!("{:?}", &trynums);
        pr_info!("{:?}", nomtest);
        Ok(Checker { numbers: nums })
    }
}

impl Drop for Checker {
    fn drop(&mut self) {
        pr_info!("Rust printing macros sample (exit)\n");
        pr_info!("{:?}", &self.numbers)
    }
}

// type MResult<T,E> = core::result::Result<T,E>;

// #[derive(Debug)]
// struct Lexer<'a> {
//     chars: core::iter::Peekable<core::str::Chars<'a>>,
//     path: Option<String>,
//     line: usize,
//     col: usize,
// }

// impl<'a> Lexer<'a> {
//     fn new(chars: &str, path: Option<String>) -> Self {
//         Self {
//             chars: chars.chars().peekable(),
//             path,
//             line: 0,
//             col: 0,
//         }
//     }

//     fn loc(&self) -> Location {
//         Location {
//             row: self.line + 1,
//             col: self.col,
//         }
//     }

//     fn skip_whitespace(&mut self) {
//         while let Some(x) = self.chars.peek() {
//             if !x.is_whitespace() {
//                 break;
//             }
//                self.consume_char();
//         }
//     }
//     fn consume_char(&mut self) -> Option<char> {
//         match self.chars.next() {
//             Some(c) => {
//                 self.col += 1;
//                 if c == '\n' {
//                     self.col = 1;
//                     self.line += 1;
//                 }
//                 Some(c)
//             },
//             None => None
//         }
//     }

//     fn next_token(&mut self) -> MResult<Token, LexerError> {
//         self.skip_whitespace();
//         if let Some(c) = self.consume_char() {
//             self.to_token(c)
//         }
//         Ok(Token{kind: TokenKind::EOF, raw: "".to_string(), location: self.loc() })
//     }

//     fn parse_number(&mut self, initial: char) -> Result<Token, LexerError> {
//         let radix = 10;
//         /*
//          * We can parse either naturals or rationals
//          */
//         let mut numerator = initial.to_string();
//         let mut denominator = "".to_string();
//         let mut rational = false;

//         loop {
//             match self.chars.peek() {
//                 Some(c) if c.is_digit(radix) => {
//                     if rational {
//                         denominator.push(c);
//                     } else {
//                         numerator.push(c);
//                     }
//                     self.consume_char()
//                 },
//                 Some(c) if c == '/' && !rational => {
//                     rational = true;
//                     self.consume_char()
//                 },
//                 Some(c) if !c.is_whitespace() => Err,
//                 _ => break,
//             }
//         }
//         if rational {

//         } else {

//         }

//     }

//     fn to_token(&mut self, c: char) -> MResult<Token, LexerError> {
//         match c {
//             '(' | ')' | '%' | '!' |
//             '#' | '@' | ':' | '\'' |
//             '^' | '_' | '~' => Ok(Token{ kind: char_to_symbol(&c),
//                                          raw: c,
//                                          location: self.loc() }),
//             '0' ..= '9' => parse_number(s, negative)
//         }
//     }

//     // fn drop_line(&mut self) {
//     //     while let Some(c) = self.consume_char() {
//     //         if c == '\n' {
//     //             return
//     //         }
//     //     }
//     // }
// }

// fn char_to_symbol(c: &char) -> SymbolKind {
//     match c {
//     '('  => SymbolKind::LParen,
//     ')'  => SymbolKind::RParen,
//     '%'  => SymbolKind::Percent,
//     '!'  => SymbolKind::Bang,
//     '#'  => SymbolKind::Pound,
//     '@'  => SymbolKind::At,
//     ':'  => SymbolKind::Colon,
//     '\'' => SymbolKind::BackSlash,
//     '^'  => SymbolKind::Caret,
//     '_'  => SymbolKind::Hole,
//     '~'  => SymbolKind::Tilde
//     }
// }

// #[derive(Debug)]
// struct Token {
//     kind: TokenKind,
//     raw: String,
//     location: Location,
// }

// #[derive(Debug)]
// struct Location {
//     col: usize,
//     row: usize,
// }

// enum SymbolKind {
//     LParen,
//     RParen,
//     Percent,
//     Bang,
//     Pound,
//     At,
//     Colon,
//     BackSlash,
//     Caret,
//     Hole,
//     Tilde,

// }


// impl ToString for SymbolKind {
//     fn to_string(&self) -> String {
//         match self {
//             LParen => "(",
//             RParent => ")",
//             Percent => "%",
//             Bang => "!",
//             Pound => "#",
//             At => "@",
//             Colon => ":",
//             BackSlash => "\\",
//             Caret => "^",
//             Hole => "_",
//             Tilde => "~"
//         }.to_string()
//     }
// }

// #[derive(Debug)]
// enum TokenKind {
//     Symbol(SymbolKind),
//     // keywords
//     Declare,
//     Define,
//     Check,
//     Program,
//     Function,
//     Opaque,
//     Run,

//     Type,

//     Let,
//     Do,
//     Match,
//     Default,
//     Mpz,
//     Mpq,
//     MpAdd,
//     MpNeg, // should be called sub?
//     MpDiv,
//     MpMul,
//     MpIfNeg,
//     MpIfZ,
//     MpzToMpq,
//     Compare,
//     IfEq,
//     Fail,

//     Provided,
//     DeclareRule,
//     DeclareType,
//     DefineConst,
//     Pi,
//     Arrow,
//     Lam,
//     CheckAssuming,
//     MarkVar,
//     IfMarked,
//     Natural,
//     Rational,

//     EOF,
// }

// enum LexerError {
//     TmpErr
//     // MissingSymbol { expected: TokenType, found: Token },
// }
