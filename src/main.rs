use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn precedence(&self) -> i64 {
        match self {
            Self::Add | Self::Sub => 10,
            Self::Mul | Self::Div => 100,
        }
    }

    fn maybe(s: &str) -> Option<Operator> {
        match s {
            "+" => Some(Self::Add),
            "-" => Some(Self::Sub),
            "*" => Some(Self::Mul),
            "/" => Some(Self::Div),
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

    if let Some(_) = Operator::maybe(first) {
        panic!("operator found at beginning of token stream. unary operators not supported.");
    }

    let left = Node::Leaf(first.to_string());

    if rest.len() == 0 {
        return left;
    }

    let (op, rest) = rest.split_first().unwrap();
    match Operator::maybe(op) {
        None => panic!("operator expected after leaf node"),
        Some(op) => {
            let right = parse(rest);
            match right {
                Node::Leaf(_) => Node::Operation {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
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
                        left: Box::new(Node::Operation {
                            op,
                            left: Box::new(left),
                            right: subnode_left,
                        }),
                        right: subnode_right,
                    },
                },
            }
        }
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
    assert_eq!(parse(&tokenize("a")), Node::Leaf("a".to_string()));

    assert_eq!(
        parse(&tokenize("a + b")),
        Node::Operation {
            op: Operator::Add,
            left: leafbox("a"),
            right: leafbox("b"),
        }
    );

    assert_eq!(
        parse(&tokenize("a + b - c")),
        Node::Operation {
            op: Operator::Add,
            left: leafbox("a"),
            right: Box::new(Node::Operation {
                op: Operator::Sub,
                left: leafbox("b"),
                right: leafbox("c"),
            }),
        }
    );

    assert_eq!(
        parse(&tokenize("a + b * c")),
        Node::Operation {
            op: Operator::Add,
            left: leafbox("a"),
            right: Box::new(Node::Operation {
                op: Operator::Mul,
                left: leafbox("b"),
                right: leafbox("c"),
            }),
        }
    );

    assert_eq!(
        parse(&tokenize("a * b + c")),
        Node::Operation {
            op: Operator::Add,
            left: Box::new(Node::Operation {
                op: Operator::Mul,
                left: leafbox("a"),
                right: leafbox("b"),
            }),
            right: leafbox("c"),
        }
    );

    assert_eq!(
        parse(&tokenize("a + b * c + d")),
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
                right: leafbox("d")
            })
        }
    );
}
