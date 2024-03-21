#[allow(dead_code)]
mod lexer;
mod token;

pub use token::Token;

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    #[test]
    fn check_math() {
        let input = "let five = 5 * 10 + ( 2 + 4) * 25 / 2 - 56;";

        let tokens = Lexer::new(input).tokenize();

        println!("{:?}", tokens)
    }

    #[test]
    fn function() {
        let input = "
        fn add(a : i32 , b : i32) : i32 {
            let real = 10;
            let other = 'a';
            let real = \"String\";
            print(real);
        }";

        let tokens = Lexer::new(input).tokenize();

        println!("{:?}", tokens)
    }
}
