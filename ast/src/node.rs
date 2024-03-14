#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOpKind {
    Eq,
    NotEq,
    LessThen,
    LessThenEq,
    GreaterThen,
    GreaterThenEq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComparisonOp {
    pub op: ComparisonOpKind,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    String(String),
    Char(char),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub name: String,
    pub _type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Literal(Literal),
    Variable(String),
    ComparisonOp(ComparisonOp),

    Assignment {
        name: String,
        value: Box<Node>,
    },

    If {
        // Base condition to check
        base_condition: ComparisonOp,
        // If the base condition is true execute this node
        base_node: Box<Node>,
        // Else if
        // a vector of the comparison and the node to execute if the comparison is true
        else_if: Vec<(ComparisonOp, Box<Node>)>,
        // Else
        _else: Option<Box<Node>>,
    },

    Fn {
        name: String,
        args: Vec<Arg>,
        body: Vec<Node>,
    },

    BinOp {
        op: BinaryOpKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Literal::Int(i)
    }
}
