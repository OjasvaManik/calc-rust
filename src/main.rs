use std::io::{self, Write};
mod lexer;
mod tokens;

fn main() {
    print_logo();
    loop {
        let expr = take_input();
        let expr = lexer::lex(&expr);
        println!("\n{:?}", expr);
        if expr.contains(&tokens::Token::Exit) {
            println!("Exiting...");
            break;
        }
    }
}

fn print_logo() {
    let logo = r#"                                                       
      _____         _____    ____              _____   
  ___|\    \    ___|\    \  |    |         ___|\    \  
 /    /\    \  /    /\    \ |    |        /    /\    \ 
|    |  |    ||    |  |    ||    |       |    |  |    |
|    |  |____||    |__|    ||    |  ____ |    |  |____|
|    |   ____ |    .--.    ||    | |    ||    |   ____ 
|    |  |    ||    |  |    ||    | |    ||    |  |    |
|\ ___\/    /||____|  |____||____|/____/||\ ___\/    /|
| |   /____/ ||    |  |    ||    |     ||| |   /____/ |
 \|___|    | /|____|  |____||____|_____|/ \|___|    | /
   \( |____|/   \(      )/    \(    )/      \( |____|/ 
    '   )/       '      '      '    '        '   )/    
        '                                        '     "#;

    println!("{}\n", logo);
}

fn take_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Incorrect expression!");

    expr
}
