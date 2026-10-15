# rust-tokenizer-recognizer
A two-stage programming languages project written in Rust. It reads a source file written in a small C-like language, converts the source into tokens, and then checks whether the token sequence follows the project's grammar.

tokenizer.rs reads input text, splits it into lexemes, assigns each lexeme a TokenId, and writes token/lexeme pairs to a file.
recognizer.rs reads that token file and applies grammar functions such as header, arg_decl, body, statement, expression, and term.
If the recognized structure consumes the full token sequence, the program writes PARSED!!! to the output file. Invalid input may produce an error message or terminate early.
