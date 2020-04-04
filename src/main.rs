#[derive(Debug, PartialEq)]
enum Node {
    Operation {
        op: String,
        left: Box<Node>,
        right: Box<Node>,
    },
    Leaf(String),
}

fn maybe_operator(t: &str) -> Option<&str> {
    match t {
        "+" | "-" | "*" | "/" => Some(t),
        _ => None,
    }
}

fn parse(toks: &[String]) -> Node {
    let (first, rest) = toks.split_first().unwrap();

    if let Some(_) = maybe_operator(first) {
        panic!("operator found at beginning of token stream. unary operators not supported.");
    }

    let leaf = Node::Leaf(first.to_string());

    if rest.len() == 0 {
        return leaf;
    }

    let (op, rest) = rest.split_first().unwrap();
    match maybe_operator(op) {
        None => panic!("operator expected after leaf node"),
        Some(op) => Node::Operation {
            op: op.to_string(),
            left: Box::new(leaf),
            right: Box::new(parse(rest)),
        },
    }
}

fn main() {
    println!("Hello, world!");
}

fn to_stringvec(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test() {
    assert_eq!(parse(&to_stringvec(&["a"])), Node::Leaf("a".to_string()));
    assert_eq!(
        parse(&to_stringvec(&["a", "+", "b"])),
        Node::Operation {
            op: "+".to_string(),
            left: Box::new(Node::Leaf("a".to_string())),
            right: Box::new(Node::Leaf("b".to_string())),
        }
    );
    assert_eq!(
        parse(&to_stringvec(&["a", "+", "b", "-", "c"])),
        Node::Operation {
            op: "+".to_string(),
            left: Box::new(Node::Leaf("a".to_string())),
            right: Box::new(Node::Operation {
                op: "-".to_string(),
                left: Box::new(Node::Leaf("b".to_string())),
                right: Box::new(Node::Leaf("c".to_string())),
            }),
        }
    );
}
