pub type Expr = Box<Node>;

#[derive(Debug)]
pub enum Node {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Neg(Expr),
    Val(f64),
}

