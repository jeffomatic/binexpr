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
    Enclosure(Box<Node>),
    Leaf(String),
}

fn parse(toks: &[String]) -> Node {
    if toks.is_empty() {
        panic!("empty token stream");
    }

    let (first, rest) = toks.split_first().unwrap();
    let (left, rest) = match first.as_str() {
        _ if Operator::maybe(first).is_some() => {
            panic!("operator found at beginning of token stream. unary operators not supported.")
        }
        ")" => panic!("unmatched close brace"),
        "(" => parse_enclosed(rest),
        _ => (Node::Leaf(first.to_string()), rest),
    };

    if rest.is_empty() {
        return left;
    }

    let (op, rest) = rest.split_first().unwrap();
    match Operator::maybe(op) {
        None => panic!("operator expected after leaf node"),
        Some(op) => compose_with_precedence(op, left, parse(rest)),
    }
}

// Parses a token stream assuming that the preceding token was an open brace.
// Returns the first node in the stream (the parsed enclosed expression), plus
// any remaining tokens in the stream not part of the enclosed expression.
fn parse_enclosed(toks: &[String]) -> (Node, &[String]) {
    let mut open = 1;
    let mut close_pos = None;

    // search for matching closing brace
    for (i, s) in toks.iter().enumerate() {
        match s.as_str() {
            "(" => open += 1,
            ")" => {
                open -= 1;
                if open == 0 {
                    close_pos = Some(i);
                    break;
                }
            }
            _ => (),
        }
    }

    match close_pos {
        None => panic!("unmatched open brace"),
        Some(close_pos) => {
            let (inner, after) = toks.split_at(close_pos);
            (
                Node::Enclosure(Box::new(parse(inner))),
                after.split_first().unwrap().1, // skip closing brace
            )
        }
    }
}

fn compose_with_precedence(op: Operator, left: Node, into: Node) -> Node {
    match into {
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
        // leaves and enclosures
        _ => Node::Operation {
            op,
            left: Box::new(left),
            right: Box::new(into),
        },
    }
}

fn main() {
    println!("Hello, world!");
}

// TODO: allow adjacent tokens that have no whitespace between them, e.g. "(a+b)"
fn tokenize(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()
}

fn leafbox(s: &str) -> Box<Node> {
    Box::new(Node::Leaf(s.to_string()))
}

#[test]
fn test() {
    let cases = [
        ("a", *leafbox("a")),
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
            "a * b ^ c + d",
            Node::Operation {
                op: Operator::Add,
                left: Box::new(Node::Operation {
                    op: Operator::Mul,
                    left: leafbox("a"),
                    right: Box::new(Node::Operation {
                        op: Operator::Exp,
                        left: leafbox("b"),
                        right: leafbox("c"),
                    }),
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
        ("( a )", Node::Enclosure(leafbox("a"))),
        (
            "( ( a ) )",
            Node::Enclosure(Box::new(Node::Enclosure(leafbox("a")))),
        ),
        (
            "( a + b )",
            Node::Enclosure(Box::new(Node::Operation {
                op: Operator::Add,
                left: leafbox("a"),
                right: leafbox("b"),
            })),
        ),
        (
            "a * ( b + c )",
            Node::Operation {
                op: Operator::Mul,
                left: leafbox("a"),
                right: Box::new(Node::Enclosure(Box::new(Node::Operation {
                    op: Operator::Add,
                    left: leafbox("b"),
                    right: leafbox("c"),
                }))),
            },
        ),
        (
            "( a + b ) * c",
            Node::Operation {
                op: Operator::Mul,
                left: Box::new(Node::Enclosure(Box::new(Node::Operation {
                    op: Operator::Add,
                    left: leafbox("a"),
                    right: leafbox("b"),
                }))),
                right: leafbox("c"),
            },
        ),
        (
            "( a + b ) * c",
            Node::Operation {
                op: Operator::Mul,
                left: Box::new(Node::Enclosure(Box::new(Node::Operation {
                    op: Operator::Add,
                    left: leafbox("a"),
                    right: leafbox("b"),
                }))),
                right: leafbox("c"),
            },
        ),
        (
            "( a + b ) ^ ( c * d )",
            Node::Operation {
                op: Operator::Exp,
                left: Box::new(Node::Enclosure(Box::new(Node::Operation {
                    op: Operator::Add,
                    left: leafbox("a"),
                    right: leafbox("b"),
                }))),
                right: Box::new(Node::Enclosure(Box::new(Node::Operation {
                    op: Operator::Mul,
                    left: leafbox("c"),
                    right: leafbox("d"),
                }))),
            },
        ),
    ];

    for (input, want) in cases.iter() {
        assert_eq!(parse(&tokenize(input)), *want);
    }
}
