#[allow(non_camel_case_types)] //stops warnings about enum variant naming (rust hates underscores)
#[derive(Debug, PartialEq, Clone)] //derives traits that allow printing copying and comparing TokenId values
pub enum TokenId { //defines enum for token types
    VARTYPE, //token for variable types (int, void)
    RETURN_KEYWORD, //token for return keyword
    WHILE_KEYWORD, //token for while keyword
    LEFT_PARENTHESIS, //token for left parenthesis
    RIGHT_PARENTHESIS, //token for right parenthesis
    LEFT_BRACKET, //token for left curly brace
    RIGHT_BRACKET, //token for right curly brace
    EOL, //token for semicolon
    EQUAL, //token for equal sign
    NUMBER, //token for numeric literals
    IDENTIFIER, //token for identifiers (variable names)
    BINOP, //token for binary operators (+ - * /)
    COMMA, //token for comma
}

#[derive(Debug, Clone)] //derives traits that allow printing and copying Lex values
#[allow(dead_code)] //allows unused fields in Lex struct without warnings
pub struct Lex { //defines struct for lexemes with token type and lexeme string
    pub token: TokenId, //field for token type
    pub lexeme: String, //field for lexeme string
}
