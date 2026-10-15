mod common;
use common::{TokenId, Lex};

//standard library imports
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect(); //this collects the command line argument and turns it into a string vector
    let input_path = &args[1]; //first argument is the input file path,  (& is used as a sign that this is a reference to args)
    let output_path = &args[2]; //second argument is the output file path,   (the first index args[0] is the name of the program )

    let mut input_file = File::open(input_path).unwrap(); //opens input file "unwraps" the result
    let mut items = String::new();
    input_file.read_to_string(&mut items).unwrap(); //reads the contents of the file into string variable "items"

    let lexemes = tokenize(&items); //calls tokenize func to conver string into vector of lexemes
    let tokens = classify_lexemes(lexemes); //calls classify_lexemes func to convert vector of lexemes into vector of tokens

    let mut output_file = File::create(output_path).unwrap(); //creates output file
    for lex in tokens {
        writeln!(output_file, "{:?} {}", lex.token, lex.lexeme).unwrap(); //writes the token type and lexeme to output file
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let mut lexemes = Vec::new(); //creates empty vector to hold lexemes
    let mut current = String::new(); // creates empty string to build the current lexemes

    let chars: Vec<char> = input.chars().collect(); //convert input string into vector of chars
    let mut i = 0;

    while i < chars.len() { //while loop to iterate through chars
        let c = chars[i];

        if c.is_whitespace() {
            if !current.is_empty() {
                lexemes.push(current.clone()); //if current string is not empty add it to lexemes vector
                current.clear(); //clear current string for next lexeme
            }
            i += 1;
            continue; //skip whitespace
        }

        if is_symbol(c) { //handles single character symbols
            if !current.is_empty() {
                lexemes.push(current.clone()); //if current string is not empty add it to lexemes vector
                current.clear(); //clear current string for next lexeme
            }
            lexemes.push(c.to_string()); //add symbol as a lexeme
            i += 1;
            continue; //skip to next char
        }

        current.push(c); //add char to current string
        i += 1;
    }

    if !current.is_empty() {
        lexemes.push(current); //add remaining current string as lexeme
    }

    lexemes
}

fn is_symbol(c: char) -> bool {
    matches!(c, '(' | ')' | '{' | '}' | ';' | '=' | '+' | '-' | '*' | '/' | ',' | '!' | '%')
}

fn classify_lexemes(lexemes: Vec<String>) -> Vec<Lex> {
    let mut result = Vec::new(); //creates empty vector to hold classified lexemes

    for lexeme in lexemes {
        let token = match lexeme.as_str() { //matches lexeme string to determine token type
            "int" | "void" => TokenId::VARTYPE, //if lexeme is "int" or "void" it's a VARTYPE
            "return" => TokenId::RETURN_KEYWORD, //if lexeme is "return" it's a RETURN
            "while" => TokenId::WHILE_KEYWORD, //if lexeme is "while" it's a WHILE_KEY
            "(" => TokenId::LEFT_PARENTHESIS, //if lexeme is "(" it's a LEFT_PAREN
            ")" => TokenId::RIGHT_PARENTHESIS, //if lexeme is ")" it's a RIGHT_PAREN
            "{" => TokenId::LEFT_BRACKET, //if lexeme is "{" it's a LEFT_BRACKET
            "}" => TokenId::RIGHT_BRACKET, //if lexeme is "}" it's a RIGHT_BRACKET
            ";" => TokenId::EOL, //if lexeme is ";" it's a SEMI
            "=" => TokenId::EQUAL, //if lexeme is "=" it's an EQUAL
            "+" | "-" | "*" | "/" | "%" => TokenId::BINOP, //if lexeme is a binary operator it's a BINOP
            "," => TokenId::COMMA, //if lexeme is "," it's a COMMA
            _ if is_number(&lexeme) => TokenId::NUMBER, //if lexeme is a number it's a NUMBER
            _ => TokenId::IDENTIFIER, //otherwise it's an IDENTIFIER
        };

        result.push(Lex { token, lexeme }); //creates Lex struct with token type and lexeme string and sums to result vector
    }

    result
}

fn is_number(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false; //if any char is not numeric return false
        }
    }
    true //if all chars are numeric return true
}