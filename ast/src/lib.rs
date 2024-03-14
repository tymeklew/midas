#[allow(dead_code)]
mod node;

#[cfg(test)]
mod tests {
    use crate::node::{ComparisonOp, ComparisonOpKind, Literal, Node};

    #[test]
    fn real() {
        // if (1 == 1) {
        //      real = 1;
        // }
        let if_statment = Node::If {
            base_condition: ComparisonOp {
                op: ComparisonOpKind::Eq,
                lhs: Node::Variable(String::from("1")),
                rhs: Box::new(Node::Literal(Literal::Int(1))),
            },
            base_node: Box::new(Node::Assignment {
                name: String::from("real"),
                value: Box::new(Node::Literal(Literal::Int(1))),
            }),
            else_if: Vec::new(),
            _else: None,
        };

        println!("{:?}", if_statment);
    }
}
