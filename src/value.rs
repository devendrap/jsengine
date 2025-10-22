// Runtime value types for JavaScript
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ast::Stmt;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
    Object(Rc<RefCell<HashMap<String, Value>>>),
    Array(Rc<RefCell<Vec<Value>>>),
    Function(Function),
    NativeFunction(NativeFunction),
}

#[derive(Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: Rc<RefCell<HashMap<String, Value>>>,
    pub constructor: Option<String>, // Name of constructor function if created via 'new'
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function({:?})", self.params)
    }
}

pub type NativeFn = fn(&[Value]) -> Result<Value, String>;

#[derive(Clone)]
pub struct NativeFunction {
    pub name: String,
    pub func: NativeFn,
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NativeFunction({})", self.name)
    }
}

impl Value {
    pub fn to_boolean(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Null | Value::Undefined => false,
            _ => true,
        }
    }

    pub fn to_number(&self) -> Result<f64, String> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Value::String(s) => s.parse().map_err(|_| format!("Cannot convert '{}' to number", s)),
            Value::Null => Ok(0.0),
            Value::Undefined => Ok(f64::NAN),
            _ => Ok(f64::NAN),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.is_nan() {
                    "NaN".to_string()
                } else if n.is_infinite() {
                    if *n > 0.0 { "Infinity" } else { "-Infinity" }.to_string()
                } else if n.fract() == 0.0 && n.abs() < 1e15 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Undefined => "undefined".to_string(),
            Value::Object(obj) => {
                let obj = obj.borrow();
                let props: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{ {} }}", props.join(", "))
            }
            Value::Array(arr) => {
                let arr = arr.borrow();
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Function(_) => "[Function]".to_string(),
            Value::NativeFunction(nf) => format!("[Function: {}]", nf.name),
        }
    }

    pub fn type_of(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Null => "object", // JavaScript quirk
            Value::Undefined => "undefined",
            Value::Object(_) => "object",
            Value::Array(_) => "object",
            Value::Function(_) | Value::NativeFunction(_) => "function",
        }
    }

    // Equality operations
    pub fn strict_equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if a.is_nan() || b.is_nan() {
                    false
                } else {
                    a == b
                }
            }
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Undefined, Value::Undefined) => true,
            _ => false,
        }
    }

    pub fn loose_equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Null, Value::Undefined) | (Value::Undefined, Value::Null) => true,
            _ => {
                // Simplified loose equality
                if std::mem::discriminant(self) == std::mem::discriminant(other) {
                    self.strict_equals(other)
                } else {
                    // Try numeric comparison
                    if let (Ok(a), Ok(b)) = (self.to_number(), other.to_number()) {
                        if a.is_nan() || b.is_nan() {
                            false
                        } else {
                            a == b
                        }
                    } else {
                        false
                    }
                }
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.strict_equals(other)
    }
}
