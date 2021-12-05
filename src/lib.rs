mod xbyak;
use xbyak::{reg::*, Add, Cmp, Div, Mul, Pop, Ret, Sub, Xbyak};

mod code;
use code::Code;

mod token;
use token::Tokens;

mod node;
use node::Node;

pub fn cc(s: &str) -> i32 {
    let mut code = Code::new(s);
    let mut tokens = Tokens::tokenize(&mut code);
    let node = Node::expr(&mut tokens);

    let mut xbyak = Xbyak::new();

    Node::gen(node, &mut xbyak);
    xbyak.pop(RAX);

    xbyak.ret();
    let result = xbyak.gen_code();
    xbyak.delete();
    result
}

#[test]
fn test() {
    assert_eq!(cc("0"), 0);
    assert_eq!(cc("42"), 42);
    assert_eq!(cc("5+20-4"), 21);
    assert_eq!(cc(" 12 + 34 - 5"), 41);
    assert_eq!(cc("5+6*7"), 47);
    assert_eq!(cc("5*(9-6)"), 15);
    assert_eq!(cc("(3+5)/2"), 4);
    assert_eq!(cc("-10+20"), 10);
    assert_eq!(cc("- -10"), 10);
    assert_eq!(cc("- - +10"), 10);

    assert_eq!(cc("0==1"), 0);
    assert_eq!(cc("42==42"), 1);
    assert_eq!(cc("0!=1"), 1);
    assert_eq!(cc("42!=42"), 0);

    assert_eq!(cc("0<1"), 1);
    assert_eq!(cc("1<1"), 0);
    assert_eq!(cc("2<1"), 0);
    assert_eq!(cc("0<=1"), 1);
    assert_eq!(cc("1<=1"), 1);
    assert_eq!(cc("2<=1"), 0);

    assert_eq!(cc("1>0"), 1);
    assert_eq!(cc("1>1"), 0);
    assert_eq!(cc("1>2"), 0);
    assert_eq!(cc("1>=0"), 1);
    assert_eq!(cc("1>=1"), 1);
    assert_eq!(cc("1>=2"), 0);
}
