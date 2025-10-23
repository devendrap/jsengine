// Abstract Syntax Tree definitions for JavaScript
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,

    // Identifiers
    Identifier(String),
    This,

    // Binary operations
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },

    // Unary operations
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    // Assignment
    Assignment {
        target: String,
        value: Box<Expr>,
    },

    // Member assignment (obj.prop = value or obj[prop] = value)
    MemberAssignment {
        object: Box<Expr>,
        property: Box<Expr>,
        computed: bool,
        value: Box<Expr>,
    },

    // Compound assignment (x += 5)
    CompoundAssignment {
        target: String,
        op: BinOp,
        value: Box<Expr>,
    },

    // Logical assignment (x &&= y, x ||= y)
    LogicalAssignment {
        target: String,
        op: LogicalAssignOp,
        value: Box<Expr>,
    },

    // Compound member assignment (obj.x += 5)
    CompoundMemberAssignment {
        object: Box<Expr>,
        property: Box<Expr>,
        computed: bool,
        op: BinOp,
        value: Box<Expr>,
    },

    // Logical member assignment (obj.x &&= y)
    LogicalMemberAssignment {
        object: Box<Expr>,
        property: Box<Expr>,
        computed: bool,
        op: LogicalAssignOp,
        value: Box<Expr>,
    },

    // Update expression (++x, x++, --x, x--)
    Update {
        expr: Box<Expr>,
        op: UpdateOp,
        prefix: bool, // true for ++x, false for x++
    },

    // Function call
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    // Member access (obj.prop or obj[prop])
    Member {
        object: Box<Expr>,
        property: Box<Expr>,
        computed: bool, // true for obj[prop], false for obj.prop
    },

    // Array literal
    Array(Vec<Expr>),

    // Object literal
    Object(Vec<(String, Expr)>),

    // Function expression
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },

    // Arrow function
    ArrowFunction {
        params: Vec<String>,
        body: Box<Expr>, // For now, simplified to expression body
    },

    // Conditional (ternary)
    Conditional {
        test: Box<Expr>,
        consequent: Box<Expr>,
        alternate: Box<Expr>,
    },

    // New expression (constructor call)
    New {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // Expression statement
    Expression(Expr),

    // Variable declaration
    VarDecl {
        name: String,
        init: Option<Expr>,
        kind: VarKind,
    },

    // Block statement
    Block(Vec<Stmt>),

    // If statement
    If {
        test: Expr,
        consequent: Box<Stmt>,
        alternate: Option<Box<Stmt>>,
    },

    // While loop
    While {
        test: Expr,
        body: Box<Stmt>,
    },

    // For loop
    For {
        init: Option<Box<Stmt>>,
        test: Option<Expr>,
        update: Option<Expr>,
        body: Box<Stmt>,
    },

    // For-in loop
    ForIn {
        variable: String,
        object: Expr,
        body: Box<Stmt>,
    },

    // Function declaration
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },

    // Return statement
    Return(Option<Expr>),

    // Break and Continue
    Break,
    Continue,

    // Throw statement
    Throw(Expr),

    // Try-Catch-Finally
    Try {
        block: Vec<Stmt>,
        catch_param: Option<String>,
        catch_block: Option<Vec<Stmt>>,
        finally_block: Option<Vec<Stmt>>,
    },

    // Empty statement
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    // Comparison
    Eq,
    Ne,
    StrictEq,
    StrictNe,
    Lt,
    Le,
    Gt,
    Ge,

    // Logical
    And,
    Or,

    // Relational
    InstanceOf,
    In,

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    UShr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    BitNot,
    TypeOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateOp {
    Increment, // ++
    Decrement, // --
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalAssignOp {
    And, // &&=
    Or,  // ||=
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Pow => "**",
            BinOp::Eq => "==",
            BinOp::Ne => "!=",
            BinOp::StrictEq => "===",
            BinOp::StrictNe => "!==",
            BinOp::Lt => "<",
            BinOp::Le => "<=",
            BinOp::Gt => ">",
            BinOp::Ge => ">=",
            BinOp::And => "&&",
            BinOp::Or => "||",
            BinOp::InstanceOf => "instanceof",
            BinOp::In => "in",
            BinOp::BitAnd => "&",
            BinOp::BitOr => "|",
            BinOp::BitXor => "^",
            BinOp::Shl => "<<",
            BinOp::Shr => ">>",
            BinOp::UShr => ">>>",
        };
        write!(f, "{}", s)
    }
}
