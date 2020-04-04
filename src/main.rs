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
    let (first, rest) = toks.split_first().unwrap();

    if let Some(_) = Operator::maybe(first) {
        panic!("operator found at beginning of token stream. unary operators not supported.");
    }

    let leaf = Node::Leaf(first.to_string());

    if rest.len() == 0 {
        return leaf;
    }

    let (op, rest) = rest.split_first().unwrap();
    match Operator::maybe(op) {
        None => panic!("operator expected after leaf node"),
        Some(op) => Node::Operation {
            op,
            left: Box::new(leaf),
            right: Box::new(parse(rest)),
        },
    }
}

fn main() {
    println!("Hello, world!");
}

fn tokenize(str: &str) -> Vec<String> {
    str.split_whitespace().map(|s| s.to_string()).collect()
}

#[test]
fn test() {
    assert_eq!(parse(&tokenize("a")), Node::Leaf("a".to_string()));

    assert_eq!(
        parse(&tokenize("a + b")),
        Node::Operation {
            op: Operator::Add,
            left: Box::new(Node::Leaf("a".to_string())),
            right: Box::new(Node::Leaf("b".to_string())),
        }
    );

    assert_eq!(
        parse(&tokenize("a + b - c")),
        Node::Operation {
            op: Operator::Add,
            left: Box::new(Node::Leaf("a".to_string())),
            right: Box::new(Node::Operation {
                op: Operator::Sub,
                left: Box::new(Node::Leaf("b".to_string())),
                right: Box::new(Node::Leaf("c".to_string())),
            }),
        }
    );
}
