use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

impl Operator {
    fn precedence(&self) -> i64 {
        match self {
            Self::Add | Self::Sub => 10,
            Self::Mul | Self::Div => 100,
            Self::Exp => 1000,
        }
    }

    fn maybe(s: &str) -> Option<Operator> {
        match s {
            "+" => Some(Self::Add),
            "-" => Some(Self::Sub),
            "*" => Some(Self::Mul),
            "/" => Some(Self::Div),
            "^" => Some(Self::Exp),
            _ => None,
        }
    }

    fn cmp_precedence(&self, other: &Self) -> Ordering {
        self.precedence().cmp(&other.precedence())
    }
}

#[derive(Debug, PartialEq)]
enum Node {
    Operation {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
    Leaf(String),
}

fn parse(toks: &[String]) -> Node {
    if toks.is_empty() {
        panic!("empty token stream");
    }

    let (first, rest) = toks.split_first().unwrap();
    if Operator::maybe(first).is_some() {
        panic!("operator found at beginning of token stream. unary operators not supported.");
    }

    let left = Node::Leaf(first.to_string());
    if rest.is_empty() {
        return left;
    }

    let (op, rest) = rest.split_first().unwrap();
    match Operator::maybe(op) {
        None => panic!("operator expected after leaf node"),
        Some(op) => compose_with_precedence(op, left, parse(rest)),
    }
}

fn compose_with_precedence(op: Operator, left: Node, into: Node) -> Node {
    match into {
        Node::Leaf(_) => Node::Operation {
            op,
            left: Box::new(left),
            right: Box::new(into),
        },
        Node::Operation {
            op: subnode_op,
            left: subnode_left,
            right: subnode_right,
        } => match op.cmp_precedence(&subnode_op) {
            Ordering::Less | Ordering::Equal => Node::Operation {
                op,
                left: Box::new(left),
                right: Box::new(Node::Operation {
                    op: subnode_op,
                    left: subnode_left,
                    right: subnode_right,
                }),
            },
            Ordering::Greater => Node::Operation {
                op: subnode_op,
                left: Box::new(compose_with_precedence(op, left, *subnode_left)),
                right: subnode_right,
            },
        },
    }
}

fn main() {
    println!("Hello, world!");
}

fn tokenize(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()
}

fn leafbox(s: &str) -> Box<Node> {
    Box::new(Node::Leaf(s.to_string()))
}

#[test]
fn test() {
    let cases = [
        ("a", Node::Leaf("a".to_string())),
        (
            "a + b",
            Node::Operation {
                op: Operator::Add,
                left: leafbox("a"),
                right: leafbox("b"),
            },
        ),
        (
            "a + b - c",
            Node::Operation {
                op: Operator::Add,
                left: leafbox("a"),
                right: Box::new(Node::Operation {
                    op: Operator::Sub,
                    left: leafbox("b"),
                    right: leafbox("c"),
                }),
            },
        ),
        (
            "a + b * c",
            Node::Operation {
                op: Operator::Add,
                left: leafbox("a"),
                right: Box::new(Node::Operation {
                    op: Operator::Mul,
                    left: leafbox("b"),
                    right: leafbox("c"),
                }),
            },
        ),
        (
            "a * b + c",
            Node::Operation {
                op: Operator::Add,
                left: Box::new(Node::Operation {
                    op: Operator::Mul,
                    left: leafbox("a"),
                    right: leafbox("b"),
                }),
                right: leafbox("c"),
            },
        ),
        (
            "a + b * c + d",
            Node::Operation {
                op: Operator::Add,
                left: leafbox("a"),
                right: Box::new(Node::Operation {
                    op: Operator::Add,
                    left: Box::new(Node::Operation {
                        op: Operator::Mul,
                        left: leafbox("b"),
                        right: leafbox("c"),
                    }),
                    right: leafbox("d"),
                }),
            },
        ),
        (
            "a ^ b * c + d",
            Node::Operation {
                op: Operator::Add,
                left: Box::new(Node::Operation {
                    op: Operator::Mul,
                    left: Box::new(Node::Operation {
                        op: Operator::Exp,
                        left: leafbox("a"),
                        right: leafbox("b"),
                    }),
                    right: leafbox("c"),
                }),
                right: leafbox("d"),
            },
        ),
        (
            "a * b + c ^ d",
            Node::Operation {
                op: Operator::Add,
                left: Box::new(Node::Operation {
                    op: Operator::Mul,
                    left: leafbox("a"),
                    right: leafbox("b"),
                }),
                right: Box::new(Node::Operation {
                    op: Operator::Exp,
                    left: leafbox("c"),
                    right: leafbox("d"),
                }),
            },
        ),
    ];

    for (input, want) in cases.iter() {
        assert_eq!(parse(&tokenize(input)), *want);
    }
}
