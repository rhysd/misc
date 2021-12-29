use ast::{Node, Expr};

pub fn evaluate(expr: &Expr) -> f64 {
    match **expr {
        Node::Add(ref l, ref r) => evaluate(l) + evaluate(r),
        Node::Sub(ref l, ref r) => evaluate(l) - evaluate(r),
        Node::Mul(ref l, ref r) => evaluate(l) * evaluate(r),
        Node::Div(ref l, ref r) => evaluate(l) / evaluate(r),
        Node::Neg(ref e)        => -evaluate(e),
        Node::Val(ref n)        => *n,
    }
}
