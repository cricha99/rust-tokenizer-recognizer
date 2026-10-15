mod common;
use common::{TokenId, Lex};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process;
use std::sync::Mutex;
use std::sync::OnceLock;

static TOKENS: OnceLock<Mutex<Vec<Lex>>> = OnceLock::new(); //global vector to hold tokens
static INDEX: OnceLock<Mutex<usize>> = OnceLock::new(); //global index to track current token

fn main() {
    let args: Vec<String> = std::env::args().collect(); //collect command line arguments
    let input_path = &args[1]; //first argument is input file path
    let output_path = &args[2]; //second argument is output file path

    let file = File::open(input_path).unwrap(); //open input file
    let reader = BufReader::new(file); //create buffered reader for input file
    let mut tokens = Vec::new(); //create vector to hold tokens

    for line in reader.lines() { //interate through lines from input file
        let line = line.unwrap(); //unwraps line result
        let parts: Vec<&str> = line.split_whitespace().collect(); //split line into parts based on whitespace

        let token = match parts[0] { //match first part of line to determine token type
            "VARTYPE" => TokenId::VARTYPE,
            "RETURN_KEYWORD" => TokenId::RETURN_KEYWORD,
            "IDENTIFIER" => TokenId::IDENTIFIER,
            "NUMBER" => TokenId::NUMBER,
            "LEFT_PARENTHESIS" => TokenId::LEFT_PARENTHESIS,
            "RIGHT_PARENTHESIS" => TokenId::RIGHT_PARENTHESIS,
            "LEFT_BRACKET" => TokenId::LEFT_BRACKET,
            "RIGHT_BRACKET" => TokenId::RIGHT_BRACKET,
            "EOL" => TokenId::EOL,
            "EQUAL" => TokenId::EQUAL,
            "WHILE_KEYWORD" => TokenId::WHILE_KEYWORD,
            "BINOP" => TokenId::BINOP,
            "COMMA" => TokenId::COMMA,
            _ => panic!("Unknown token type"),
        };

        tokens.push(Lex { //create Lex struct for token and lexeme and add to tokens vector
            token,
            lexeme: parts[1].to_string(), //second part of line is the lexeme
        });
    }

    TOKENS.get_or_init(|| Mutex::new(tokens));
    INDEX.get_or_init(|| Mutex::new(0)); //set global TOKENS vector and reset INDEX

    let mut output = File::create(output_path).unwrap(); //create output file
    program(); //call program function to start parsing

    let len = TOKENS.get().unwrap().lock().unwrap().len(); //check if all tokens were used
    let index = *INDEX.get().unwrap().lock().unwrap();
    if index != len {
        writeln!(
            output,
            "Error: Only consumed {} of the {} given tokens",
            index,
            len
        ).unwrap(); //write error message to output file
        process::exit(0); //exit with error code
    }

    writeln!(output, "PARSED!!!").unwrap(); //write success message to output file
}

//helpers

fn current_token() -> TokenId { //returns the token type of the current token
    let index = *INDEX.get().unwrap().lock().unwrap();
    let tokens = TOKENS.get().unwrap().lock().unwrap();
    if index < tokens.len() {
        tokens[index].token.clone()
    } else {
        panic!("Index out of bounds");
    }
}

fn advance() { //advances to the next token
    let mut index = INDEX.get().unwrap().lock().unwrap();
    *index += 1; //increment global index to move to next token
}

fn expect(expected: TokenId, rule: &str) { //expects the current token to match expected token type
    if current_token() == expected {
        advance(); //if token matches expected, advance to next token
    } else {
        error_token(rule, expected); //if token does not match expected, call error function
    }
}

fn error_token(rule: &str, expected: TokenId) { //handles token errors
    let index = *INDEX.get().unwrap().lock().unwrap();
    let tokens = TOKENS.get().unwrap().lock().unwrap();
    let actual_token = if index < tokens.len() {
        tokens[index].token.clone()
    } else {
        panic!("Index out of bounds");
    };
    println!(
        "Error: In grammar rule {}, expected token #{} to be {:?} but was {:?}",
        rule,
        index,
        expected,
        actual_token //print error message with rule, expected token type, and actual token type
    );
    process::exit(0);
}

fn program() {
    header(); //call header function to parse header
    body(); //call body function to parse body
}

fn header() {
    expect(TokenId::VARTYPE, "header"); //expect first token to be VARTYPE
    expect(TokenId::IDENTIFIER, "header"); //expect second token to be IDENTIFIER
    expect(TokenId::LEFT_PARENTHESIS, "header"); //expect third token to be LEFT_PAREN
    if current_token() == TokenId::VARTYPE { //if next token is VARTYPE, parse parameters
        arg_decl();
    }
    expect(TokenId::RIGHT_PARENTHESIS, "header"); //expect next token to be RIGHT_PAREN
}

fn arg_decl() {
    expect(TokenId::VARTYPE, "arg_decl"); //expect first token to be VARTYPE
    expect(TokenId::IDENTIFIER, "arg_decl"); //expect second token to be IDENTIFIER
    while current_token() == TokenId::COMMA { //while next token is COMMA, parse additional parameters
        advance(); //consume COMMA
        expect(TokenId::VARTYPE, "arg_decl"); //expect next token to be VARTYPE
        expect(TokenId::IDENTIFIER, "arg_decl"); //expect next token to be IDENTIFIER
    }
}

fn body() {
    expect(TokenId::LEFT_BRACKET, "body"); //expect first token to be LEFT_BRACKET
    while current_token() != TokenId::RIGHT_BRACKET { //while next token is not RIGHT_BRACKET, parse statements
        statement();
    }
    expect(TokenId::RIGHT_BRACKET, "body"); //expect next token to be RIGHT_BRACKET
}

fn statement() {
    match current_token() { //match current token to determine statement type
        TokenId::WHILE_KEYWORD => while_loop(), //if token is WHILE_KEY,parse while loop
        TokenId::RETURN_KEYWORD => return_stmt(), //if token is RETURN, parse return statement
        TokenId::IDENTIFIER => assignment(), //if token is IDENTIFIER, parse assignment statement
        _ => error_token("statement", TokenId::IDENTIFIER), //if token does not match any statement type, call error function
    }
}

fn while_loop() {
    expect(TokenId::WHILE_KEYWORD, "while_loop"); //expect first token to be WHILE_KEY
    expect(TokenId::LEFT_PARENTHESIS, "while_loop"); //expect second token to be LEFT_PAREN
    expression(); //parse expression 
    expect(TokenId::RIGHT_PARENTHESIS, "while_loop"); //expect next token to be RIGHT_PAREN
    body(); //parse body of while loop
}

fn return_stmt() {
    expect(TokenId::RETURN_KEYWORD, "return_stmt"); //expect first token to be RETURN
    expression(); //parse expression to return
    expect(TokenId::EOL, "return_stmt"); //expect next token to be SEMI
}

fn assignment() {
    expect(TokenId::IDENTIFIER, "assignment"); //expect first token to be IDENTIFIER
    expect(TokenId::EQUAL, "assignment"); //expect second token to be EQUAL
    expression(); //parse expression to assign
    expect(TokenId::EOL, "assignment"); //expect next token to be SEMI
}

fn expression() {
    if current_token() == TokenId::LEFT_PARENTHESIS { //if next token is LEFT_PAREN, parse binary operation
        advance(); //consume LEFT_PAREN
        expression(); //parse left operand
        expect(TokenId::RIGHT_PARENTHESIS, "expression"); //expect next token to be RIGHT_PAREN
    } else {
        term();

        while current_token() == TokenId::BINOP { //while next token is BINOP, parse additional terms
            advance(); //consume BINOP
            term(); //parse next term
        }
    }
}

fn term() {
    match current_token() {
        TokenId::IDENTIFIER => advance(), //if token is IDENTIFIER, consume it
        TokenId::NUMBER => advance(), //if token is NUMBER, consume it
        _ => error_token("term", TokenId::IDENTIFIER), //if token does not match expected types, call error function
    }
}
