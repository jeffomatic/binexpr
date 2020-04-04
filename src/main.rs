#[derive(Debug, PartialEq)]
enum Node {
    Operator {
        op: String,
        left: Box<Node>,
        right: Box<Node>,
    },
    Leaf(String),
}

fn parse<T: AsRef<str>>(toks: &[T]) -> Node {
    let (first, rest) = toks.split_first().unwrap(); // TODO: convert this to an error

    if rest.len() > 0 {
        unimplemented!();
    }

    Node::Leaf(first.as_ref().to_string())
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    assert_eq!(parse(&["a"]), Node::Leaf("a".to_string()));
}
