use std::collections::BTreeMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct File {
    pub uses: Vec<Use>,
    pub model: Model,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Use {
    pub model: String,
    pub origin: UseOrigin,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum UseOrigin {
    Site(String),
    User(String),
    Local,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct Model {
    pub name: String,
    pub inner: ModelInner,
    pub is_extern: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct ModelInner {
    pub variables: Vec<Variable>,
    pub children: Vec<Model>,
    pub graph: Vec<Graph>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default)]
pub struct Variable {
    pub name: Option<String>,
    pub description: String,
    pub default: Option<Value>,
    pub is_model: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    UInt(u64),
    Real(f64),
    Model(String),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct Graph {
    pub id: Node,
    pub passes: Vec<GraphPass>,
    pub inline: Option<Model>,
    pub shapes: Option<Shapes>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct GraphPass {
    pub name: String,
    pub args: Vec<GraphPassArg>,
    pub repeat: u64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum GraphPassArg {
    NodeArg(Vec<NodeArg>),
    Keyword { name: String, value: Value },
}

impl GraphPassArg {
    pub fn is_named(&self, name: &str) -> bool {
        match self {
            Self::Keyword {
                name: k_name,
                value: _,
            } => name == k_name,
            _ => false,
        }
    }

    pub fn unwrap_name(&self) -> &str {
        match self {
            Self::Keyword { name, value: _ } => name,
            _ => unreachable!(),
        }
    }

    pub fn unwrap_value(&self) -> &Value {
        match self {
            Self::Keyword { name: _, value } => value,
            _ => unreachable!(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct Shapes(pub BTreeMap<Node, Shape>);

impl Shapes {
    pub(crate) fn one(shape: Shape) -> Self {
        Self(vec![(0u64, shape)].into_iter().collect())
    }

    pub(crate) fn many(shapes: Vec<(u64, Shape)>) -> Self {
        Self(shapes.into_iter().collect())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct Shape(pub Vec<Dim>);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub enum Dim {
    Fixed(u64),
    Semantic(String),
    Expr {
        lhs: Box<Dim>,
        rhs: Box<Dim>,
        op: DimOp,
    },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub enum DimOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub mod temp {
    pub struct VariableNameDesc {
        pub name: Option<String>,
        pub description: String,
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub struct NodeArg {
    pub node: Node,
    pub arg: Node,
}

pub type Node = u64;
