#[derive(Debug, Clone, PartialEq)]
/// [CSS Tokenization](https://www.w3.org/TR/CSS21/syndata.html#tokenization)
pub enum TokenType {
    Ident,        // {ident}
    Atkeyword,    // @{ident}
    String,       // {string}
    BadString,    // {badstring}
    BadUri,       // {baduri}
    BadComment,   // {badcomment}
    Hash,         // #{name}
    Number,       // {num}
    Percentage,   // {num}%
    Dimension,    // {num}{ident}
    Uri,          // url\({w}{string}{w}\) | |url\({w}([!#$%&*-\[\]-~]|{nonascii}|{escape})*{w}\)
    UnicodeRange, // u\+[0-9a-f?]{1,6}(-[0-9a-f]{1,6})?
    Cdo,          // <!--
    Cdc,          // -->
    Colon,        // :
    Semicolon,    // ;
    CurlyLeft,    // \{
    CurlyRight,   // \}
    ParenLeft,    // \(
    ParenRight,   // \)
    BracketLeft,  // \[
    BracketRight, // \]
    S,            // [ \t\r\n\f]+
    Comment,      // \/\*[^*]*\*+([^/*][^*]*\*+)*\/
    Function,     // {ident}\(
    Includes,     // ~=
    DashMatch,    // |=
    Delim, // any other character not matched by the above rules, and neither a single nor a double quote
    Eof,   // EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}
