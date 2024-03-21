#[allow(dead_code)]
mod node;

#[cfg(test)]
mod tests {
    use lexer::Token;

    use crate::node::{Literals, Node};

    #[test]
    fn assignment() {
        // assignment statment
        // let a : int = 10;
        let tokens = vec![
            Token::Identifier("let".into()),
            Token::Identifier("a".into()),
            Token::Colon,
            Token::Identifier("int".into()),
            Token::Assign,
            Token::Lit(Literals::Int(10)),
        ];

        let ast = Node::Initialize {
            name: "a".into(),
            node: Box::new(Node::Literal(Literals::Int(10))),
            type_: crate::node::Type::Int,
        };
    }
}
