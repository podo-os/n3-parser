#[derive(Debug, Default)]
pub struct Model {
    pub name: String,
    pub inner: ModelInner,
}

#[derive(Debug, Default)]
pub struct ModelInner {
    pub variables: Vec<Variable>,
    pub children: Vec<Model>,
    pub graph: Vec<Graph>,
}

#[derive(Debug, Default)]
pub struct Variable {
    pub name: String,
    pub description: String,
    pub default: Option<Value>,
}

#[derive(Debug)]
pub enum Value {
    Int(i64),
    UInt(u64),
    Real(f64),
}

#[derive(Debug, Default)]
pub struct Graph {
    pub id: u64,
    pub passes: Vec<GraphPass>,
    pub inline: Option<Model>,
    pub shape: Option<Shape>,
}

#[derive(Debug, Default)]
pub struct GraphPass {
    pub name: String,
    pub args: Vec<GraphPassArg>,
    pub repeat: u64,
}

#[derive(Debug)]
pub enum GraphPassArg {
    Node(Vec<u64>),
    Keyword { name: String, value: Value },
}

#[derive(Debug, Default)]
pub struct Shape(pub Vec<Dim>);

#[derive(Debug)]
pub enum Dim {
    Fixed(u64),
    Semantic(String),
    Expr {
        lhs: Box<Dim>,
        rhs: Box<Dim>,
        op: DimOp,
    },
}

#[derive(Debug)]
pub enum DimOp {
    Add,
    Sub,
    Mul,
    Div,
}
