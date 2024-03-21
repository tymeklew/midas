pub type BNode = Box<Node>;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Literals {
    Int(i64),
}


#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Type(Type),
    Literal(Literals),

    Initialize {
        name: String,
        node: BNode,
        type_: Type,
    },
}

