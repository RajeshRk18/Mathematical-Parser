mod expression;
mod macros;
mod format;

use expression::*;

fn main() {

    let tokens: Vec<Token> = Lexer::from_str("x=y^2+5".chars()).collect();

    println!("{:?}", &tokens);

    let parsed: Vec<Exp> = Exp::parse(&mut tokens.into_iter());

    for (i, exp) in parsed.iter().enumerate() {
        if i == parsed.len() - 1{
            println!("{}", exp);
        } else {
            print!("{}", exp);
        }
    }

}
