use super::symbols::Symbols;

struct Node {
    occurence: usize,
    content: Content,
}

enum Content {
    Leaf(i8),
    Parent {
        left: Box<Node>,
        right: Box<Node>,
    },
}

type Tree = Node;

// impl Tree {
//     fn from_symbols(symbols: Symbols) -> Tree {
//         symbols.0
//     }
// }
