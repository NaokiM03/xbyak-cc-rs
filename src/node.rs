use crate::{
    xbyak::{
        reg::{RAX, RDI},
        Pop, Push,
    },
    Add, Div, Mul, Sub, Tokens, Xbyak,
};

#[derive(Debug)]
pub enum NodeKind {
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Num(i32), // int
}

#[derive(Debug)]
pub struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Self {
        Node {
            kind,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
        }
    }

    fn new_num_node(n: i32) -> Self {
        Node {
            kind: NodeKind::Num(n),
            lhs: None,
            rhs: None,
        }
    }
}

impl Node {
    fn primary(tokens: &mut Tokens) -> Self {
        if tokens.consume('(') {
            let node = Self::expr(tokens);
            tokens.expect(')');
            return node;
        }

        Self::new_num_node(tokens.expect_number())
    }

    fn unary(tokens: &mut Tokens) -> Self {
        if tokens.consume('+') {
            return Self::primary(tokens);
        }
        if tokens.consume('-') {
            return Self::new_node(NodeKind::Sub, Self::new_num_node(0), Self::unary(tokens));
        }
        Self::primary(tokens)
    }

    fn mul(tokens: &mut Tokens) -> Self {
        let mut node = Self::unary(tokens);

        while tokens.is_not_end() {
            if tokens.consume('*') {
                node = Self::new_node(NodeKind::Mul, node, Self::unary(tokens));
            } else if tokens.consume('/') {
                node = Self::new_node(NodeKind::Div, node, Self::unary(tokens));
            } else {
                return node;
            }
        }
        node // if tokens.is_end()
    }

    pub fn expr(tokens: &mut Tokens) -> Self {
        let mut node = Self::mul(tokens);

        while tokens.is_not_end() {
            if tokens.consume('+') {
                node = Self::new_node(NodeKind::Add, node, Self::mul(tokens));
            } else if tokens.consume('-') {
                node = Self::new_node(NodeKind::Sub, node, Self::mul(tokens));
            } else {
                return node;
            }
        }
        node // if tokens.is_end()
    }

    pub fn gen(node: Node, xbyak: &mut Xbyak) {
        if let NodeKind::Num(n) = node.kind {
            xbyak.push(n);
            return;
        }

        Self::gen(*node.lhs.unwrap(), xbyak);
        Self::gen(*node.rhs.unwrap(), xbyak);

        xbyak.pop(RDI);
        xbyak.pop(RAX);

        match node.kind {
            NodeKind::Add => xbyak.add(),
            NodeKind::Sub => xbyak.sub(),
            NodeKind::Mul => xbyak.mul(),
            NodeKind::Div => xbyak.div(),
            _ => panic!(),
        }

        xbyak.push(RAX);
    }
}
