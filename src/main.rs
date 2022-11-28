use std::io;
use std::io::Write;
use std::process;

// Next step:
// How do I parse more complex expressions? -- Pratt Parsing
// After that:
// More functions

#[derive(Debug, Clone, Copy)]
enum Operators {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Num(f64),
    Op(Operators),
}

// More than one space between numbers crashed the system
fn lexer(input: &mut String) -> Result<Vec<Token>, &'static str> {
    match io::stdin().read_line(input) {
        Ok(_) => (),
        Err(e) => println!("Error occured: {}", e),
    };

    if input.trim_end() == String::from("quit") {
        process::exit(0);
    }
    let tokens: Vec<&str> = input.as_str().trim().split(" ").collect();

    let mut res: Vec<Token> = vec![];
    for token in tokens {
        let num_token = token.parse::<f64>();
        if !num_token.is_err() {
            res.push(Token::Num(num_token.unwrap() as f64));
        } else {
            let cur_token = match token {
                "+" => Token::Op(Operators::Add),
                "-" => Token::Op(Operators::Sub),
                "*" => Token::Op(Operators::Mul),
                "/" => Token::Op(Operators::Div),
                _ => return Err("Cannot parse value"),
            };
            res.push(cur_token);
        }
    }
    return Ok(res);
}

fn repl() -> Result<(), io::Error> {
    let mut input = String::new();
    loop {
        io::stdout().write_all(b"> ")?;
        io::stdout().flush()?;
        match lexer(&mut input) {
            Ok(vec) => println!("{:#?}", vec),
            Err(e) => println!("There was an error: {}", e),
        };
        input.clear();
    }
}

fn main() {
    match repl() {
        Ok(_) => return,
        Err(e) => {
            println!("Error occured: {}", e);
            return;
        }
    }
}
