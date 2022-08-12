use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[a-zA-Z][a-zA-Z_0-9]*", |l| l.slice().to_owned())]
    Ident(String),

    #[regex(r"\d+(_\d+)*(\.\d+(_\d+))?", |l| l.slice().parse())]
    Number(f64),

    #[token("module")]
    KwModule,

    #[token("import")]
    KwImport,

    #[token("if")]
    KwIf,

    #[token("elif")]
    KwElif,

    #[token("else")]
    KwElse,

    #[token("for")]
    KwFor,

    #[token("in")]
    KwIn,

    #[token("while")]
    KwWhile,

    #[token("break")]
    KwBreak,

    #[token("continue")]
    KwContinue,

    #[token("return")]
    KwReturn,

    #[token("class")]
    KwClass,

    #[token("struct")]
    KwStruct,

    #[token("override")]
    KwOverride,

    #[token("mut")]
    KwMut,

    #[token("type")]
    KwType,

    #[token(";")]
    Semicolon,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Divide,

    #[token("%")]
    Modulo,

    #[token("=")]
    Assign,

    #[token("==")]
    Equals,

    #[token("!=")]
    NotEquals,

    #[token("<")]
    LAngle,

    #[token(">")]
    RAngle,

    #[token("<=")]
    LessEquals,

    #[token(">=")]
    GreaterEquals,

    #[token(":")]
    Colon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LSquare,

    #[token("]")]
    RSquare,

    #[token("{")]
    LCurly,

    #[token("}")]
    RCurly,

    #[token("+=")]
    PlusAssign,

    #[token("-=")]
    MinusAssign,

    #[token("*=")]
    MultiplyAssign,

    #[token("/=")]
    DivideAssign,

    #[token("%=")]
    ModuloAssign,

    #[token("++")]
    Increment,

    #[token("--")]
    Decrement,

    #[token("and")]
    KwAnd,

    #[token("or")]
    KwOr,

    #[token("is")]
    KwIs,

    #[token("isnot")]
    KwIsNot,

    #[token("not")]
    KwNot,

    #[token("clone")]
    KwClone,

    #[error]
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Error,
}

pub fn lex(text: &String) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(text);

    while let Some(tok) = lexer.next() {
        if tok == Token::Error {
            return Err(format!("{:?}: {:?}", tok, lexer.slice()));
        }
    }

    Ok(lexer.collect())
}
