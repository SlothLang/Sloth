use crate::native::NativeFunction;
use crate::Chunk;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Primitive {
    Integer(i128),
    Float(f64),
    Bool(bool),
    /// Pointer to an object living on heap
    Object(u32),
    Empty,
}

pub struct Object {
    /// If the object has been marked by the VM or not
    pub(crate) marked: bool,
    pub(crate) typ: ObjectType,
}

impl Object {
    pub fn new(typ: ObjectType) -> Self {
        Self { marked: false, typ }
    }
}

pub enum ObjectType {
    Box(Primitive),
    String(String),
    List(Vec<Primitive>),

    Function(Function),
    NativeFunction(NativeFunction),

    Free { next: usize },
}

pub struct Function {
    pub name: Option<String>,
    pub chunk: Chunk,
    pub arity: u8,
    pub returns_value: bool,
}

impl Function {
    pub fn root(chunk: Chunk) -> Self {
        Self {
            name: None,
            chunk,
            arity: 0,
            returns_value: false,
        }
    }
}
