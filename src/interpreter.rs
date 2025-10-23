// Tree-walking interpreter for executing JavaScript AST
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ast::*;
use crate::ast::{UpdateOp, LogicalAssignOp};
use crate::value::{Value, Function, NativeFunction, NativeFn};

pub struct Interpreter {
    global_scope: Rc<RefCell<HashMap<String, Value>>>,
    scopes: Vec<Rc<RefCell<HashMap<String, Value>>>>,
    this_value: Value,
    in_loop: bool,
    return_value: Option<Value>,
    break_flag: bool,
    continue_flag: bool,
}

pub enum ControlFlow {
    None,
    Return(Value),
    Break,
    Continue,
    Throw(Value),
}

impl Interpreter {
    pub fn new() -> Self {
        let global_scope = Rc::new(RefCell::new(HashMap::new()));
        let mut interpreter = Interpreter {
            global_scope: global_scope.clone(),
            scopes: vec![global_scope],
            this_value: Value::Undefined,
            in_loop: false,
            return_value: None,
            break_flag: false,
            continue_flag: false,
        };

        interpreter.init_globals();
        interpreter
    }

    fn init_globals(&mut self) {
        // console.log
        let console_log: NativeFn = Rc::new(|args| {
            let output: Vec<String> = args.iter().map(|v| v.to_string()).collect();
            println!("{}", output.join(" "));
            Ok(Value::Undefined)
        });

        let console = Rc::new(RefCell::new(HashMap::new()));
        console.borrow_mut().insert(
            "log".to_string(),
            Value::NativeFunction(NativeFunction {
                name: "log".to_string(),
                func: console_log,
            }),
        );

        self.set_variable("console", Value::Object(console));

        // Global functions
        let parse_int: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::NAN));
            }
            let s = args[0].to_string();
            match s.trim().parse::<f64>() {
                Ok(n) => Ok(Value::Number(n.trunc())),
                Err(_) => Ok(Value::Number(f64::NAN)),
            }
        });

        let parse_float: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::NAN));
            }
            match args[0].to_number() {
                Ok(n) => Ok(Value::Number(n)),
                Err(_) => Ok(Value::Number(f64::NAN)),
            }
        });

        let is_nan: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Boolean(true));
            }
            match args[0].to_number() {
                Ok(n) => Ok(Value::Boolean(n.is_nan())),
                Err(_) => Ok(Value::Boolean(true)),
            }
        });

        self.set_variable("parseInt", Value::NativeFunction(NativeFunction {
            name: "parseInt".to_string(),
            func: parse_int,
        }));

        self.set_variable("parseFloat", Value::NativeFunction(NativeFunction {
            name: "parseFloat".to_string(),
            func: parse_float,
        }));

        self.set_variable("isNaN", Value::NativeFunction(NativeFunction {
            name: "isNaN".to_string(),
            func: is_nan,
        }));

        // Math object
        let math_obj = Rc::new(RefCell::new(HashMap::new()));

        // Math.floor
        let floor_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::NAN));
            }
            match args[0].to_number() {
                Ok(n) => Ok(Value::Number(n.floor())),
                Err(_) => Ok(Value::Number(f64::NAN)),
            }
        });
        math_obj.borrow_mut().insert("floor".to_string(), Value::NativeFunction(NativeFunction {
            name: "floor".to_string(),
            func: floor_fn,
        }));

        // Math.ceil
        let ceil_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::NAN));
            }
            match args[0].to_number() {
                Ok(n) => Ok(Value::Number(n.ceil())),
                Err(_) => Ok(Value::Number(f64::NAN)),
            }
        });
        math_obj.borrow_mut().insert("ceil".to_string(), Value::NativeFunction(NativeFunction {
            name: "ceil".to_string(),
            func: ceil_fn,
        }));

        // Math.round
        let round_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::NAN));
            }
            match args[0].to_number() {
                Ok(n) => Ok(Value::Number(n.round())),
                Err(_) => Ok(Value::Number(f64::NAN)),
            }
        });
        math_obj.borrow_mut().insert("round".to_string(), Value::NativeFunction(NativeFunction {
            name: "round".to_string(),
            func: round_fn,
        }));

        // Math.abs
        let abs_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::NAN));
            }
            match args[0].to_number() {
                Ok(n) => Ok(Value::Number(n.abs())),
                Err(_) => Ok(Value::Number(f64::NAN)),
            }
        });
        math_obj.borrow_mut().insert("abs".to_string(), Value::NativeFunction(NativeFunction {
            name: "abs".to_string(),
            func: abs_fn,
        }));

        // Math.sqrt
        let sqrt_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::NAN));
            }
            match args[0].to_number() {
                Ok(n) => Ok(Value::Number(n.sqrt())),
                Err(_) => Ok(Value::Number(f64::NAN)),
            }
        });
        math_obj.borrow_mut().insert("sqrt".to_string(), Value::NativeFunction(NativeFunction {
            name: "sqrt".to_string(),
            func: sqrt_fn,
        }));

        // Math.pow
        let pow_fn: NativeFn = Rc::new(|args| {
            if args.len() < 2 {
                return Ok(Value::Number(f64::NAN));
            }
            match (args[0].to_number(), args[1].to_number()) {
                (Ok(base), Ok(exp)) => Ok(Value::Number(base.powf(exp))),
                _ => Ok(Value::Number(f64::NAN)),
            }
        });
        math_obj.borrow_mut().insert("pow".to_string(), Value::NativeFunction(NativeFunction {
            name: "pow".to_string(),
            func: pow_fn,
        }));

        // Math.min
        let min_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::INFINITY));
            }
            let mut min = f64::INFINITY;
            for arg in args {
                match arg.to_number() {
                    Ok(n) => {
                        if n.is_nan() {
                            return Ok(Value::Number(f64::NAN));
                        }
                        min = min.min(n);
                    }
                    Err(_) => return Ok(Value::Number(f64::NAN)),
                }
            }
            Ok(Value::Number(min))
        });
        math_obj.borrow_mut().insert("min".to_string(), Value::NativeFunction(NativeFunction {
            name: "min".to_string(),
            func: min_fn,
        }));

        // Math.max
        let max_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::NEG_INFINITY));
            }
            let mut max = f64::NEG_INFINITY;
            for arg in args {
                match arg.to_number() {
                    Ok(n) => {
                        if n.is_nan() {
                            return Ok(Value::Number(f64::NAN));
                        }
                        max = max.max(n);
                    }
                    Err(_) => return Ok(Value::Number(f64::NAN)),
                }
            }
            Ok(Value::Number(max))
        });
        math_obj.borrow_mut().insert("max".to_string(), Value::NativeFunction(NativeFunction {
            name: "max".to_string(),
            func: max_fn,
        }));

        // Math.random
        let random_fn: NativeFn = Rc::new(|_args| {
            use std::collections::hash_map::RandomState;
            use std::hash::{BuildHasher, Hasher};
            let s = RandomState::new();
            let mut hasher = s.build_hasher();
            hasher.write_u64(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64);
            let hash = hasher.finish();
            Ok(Value::Number((hash as f64) / (u64::MAX as f64)))
        });
        math_obj.borrow_mut().insert("random".to_string(), Value::NativeFunction(NativeFunction {
            name: "random".to_string(),
            func: random_fn,
        }));

        // Math constants
        math_obj.borrow_mut().insert("PI".to_string(), Value::Number(std::f64::consts::PI));
        math_obj.borrow_mut().insert("E".to_string(), Value::Number(std::f64::consts::E));

        self.set_variable("Math", Value::Object(math_obj));

        // Object methods
        let object_obj = Rc::new(RefCell::new(HashMap::new()));

        // Object.keys
        let keys_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Err("Object.keys requires an argument".to_string());
            }
            match &args[0] {
                Value::Object(map) => {
                    let keys: Vec<Value> = map.borrow().keys()
                        .map(|k| Value::String(k.clone()))
                        .collect();
                    Ok(Value::Array(Rc::new(RefCell::new(keys))))
                }
                Value::Array(arr) => {
                    let keys: Vec<Value> = (0..arr.borrow().len())
                        .map(|i| Value::String(i.to_string()))
                        .collect();
                    Ok(Value::Array(Rc::new(RefCell::new(keys))))
                }
                _ => Ok(Value::Array(Rc::new(RefCell::new(vec![]))))
            }
        });
        object_obj.borrow_mut().insert("keys".to_string(), Value::NativeFunction(NativeFunction {
            name: "keys".to_string(),
            func: keys_fn,
        }));

        // Object.values
        let values_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Err("Object.values requires an argument".to_string());
            }
            match &args[0] {
                Value::Object(map) => {
                    let values: Vec<Value> = map.borrow().values().cloned().collect();
                    Ok(Value::Array(Rc::new(RefCell::new(values))))
                }
                Value::Array(arr) => {
                    Ok(Value::Array(Rc::new(RefCell::new(arr.borrow().clone()))))
                }
                _ => Ok(Value::Array(Rc::new(RefCell::new(vec![]))))
            }
        });
        object_obj.borrow_mut().insert("values".to_string(), Value::NativeFunction(NativeFunction {
            name: "values".to_string(),
            func: values_fn,
        }));

        // Object.entries
        let entries_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Err("Object.entries requires an argument".to_string());
            }
            match &args[0] {
                Value::Object(map) => {
                    let entries: Vec<Value> = map.borrow().iter()
                        .map(|(k, v)| {
                            Value::Array(Rc::new(RefCell::new(vec![
                                Value::String(k.clone()),
                                v.clone()
                            ])))
                        })
                        .collect();
                    Ok(Value::Array(Rc::new(RefCell::new(entries))))
                }
                Value::Array(arr) => {
                    let entries: Vec<Value> = arr.borrow().iter().enumerate()
                        .map(|(i, v)| {
                            Value::Array(Rc::new(RefCell::new(vec![
                                Value::String(i.to_string()),
                                v.clone()
                            ])))
                        })
                        .collect();
                    Ok(Value::Array(Rc::new(RefCell::new(entries))))
                }
                _ => Ok(Value::Array(Rc::new(RefCell::new(vec![]))))
            }
        });
        object_obj.borrow_mut().insert("entries".to_string(), Value::NativeFunction(NativeFunction {
            name: "entries".to_string(),
            func: entries_fn,
        }));

        // Object.assign
        let assign_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Err("Object.assign requires at least one argument".to_string());
            }
            let target = match &args[0] {
                Value::Object(map) => map.clone(),
                _ => return Err("Object.assign target must be an object".to_string()),
            };

            for i in 1..args.len() {
                if let Value::Object(source) = &args[i] {
                    for (k, v) in source.borrow().iter() {
                        target.borrow_mut().insert(k.clone(), v.clone());
                    }
                }
            }
            Ok(Value::Object(target))
        });
        object_obj.borrow_mut().insert("assign".to_string(), Value::NativeFunction(NativeFunction {
            name: "assign".to_string(),
            func: assign_fn,
        }));

        // Object.create
        let create_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::Object(Rc::new(RefCell::new(HashMap::new()))));
            }
            // For now, just create an empty object (simplified - doesn't handle prototype chain)
            Ok(Value::Object(Rc::new(RefCell::new(HashMap::new()))))
        });
        object_obj.borrow_mut().insert("create".to_string(), Value::NativeFunction(NativeFunction {
            name: "create".to_string(),
            func: create_fn,
        }));

        self.set_variable("Object", Value::Object(object_obj));

        // JSON object
        let json_obj = Rc::new(RefCell::new(HashMap::new()));

        // JSON.stringify
        let stringify_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Ok(Value::String("undefined".to_string()));
            }
            Ok(Value::String(value_to_json(&args[0])))
        });
        json_obj.borrow_mut().insert("stringify".to_string(), Value::NativeFunction(NativeFunction {
            name: "stringify".to_string(),
            func: stringify_fn,
        }));

        // JSON.parse
        let parse_fn: NativeFn = Rc::new(|args| {
            if args.is_empty() {
                return Err("JSON.parse requires a string argument".to_string());
            }
            let json_str = args[0].to_string();
            parse_json(&json_str)
        });
        json_obj.borrow_mut().insert("parse".to_string(), Value::NativeFunction(NativeFunction {
            name: "parse".to_string(),
            func: parse_fn,
        }));

        self.set_variable("JSON", Value::Object(json_obj));
    }

    fn current_scope(&self) -> Rc<RefCell<HashMap<String, Value>>> {
        self.scopes.last().unwrap().clone()
    }

    fn push_scope(&mut self) {
        let new_scope = Rc::new(RefCell::new(HashMap::new()));
        self.scopes.push(new_scope);
    }

    fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    fn get_variable(&self, name: &str) -> Result<Value, String> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.borrow().get(name) {
                return Ok(value.clone());
            }
        }
        Err(format!("Undefined variable: {}", name))
    }

    fn set_variable(&mut self, name: &str, value: Value) {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if scope.borrow().contains_key(name) {
                scope.borrow_mut().insert(name.to_string(), value);
                return;
            }
        }

        // If not found, create in current scope
        self.current_scope().borrow_mut().insert(name.to_string(), value);
    }

    fn declare_variable(&mut self, name: &str, value: Value) {
        self.current_scope().borrow_mut().insert(name.to_string(), value);
    }

    pub fn eval(&mut self, statements: &[Stmt]) -> Result<Value, String> {
        let last_value = Value::Undefined;

        for stmt in statements {
            if self.return_value.is_some() || self.break_flag || self.continue_flag {
                break;
            }

            match self.eval_statement(stmt)? {
                ControlFlow::Return(val) => {
                    self.return_value = Some(val.clone());
                    return Ok(val);
                }
                ControlFlow::Break => {
                    self.break_flag = true;
                    break;
                }
                ControlFlow::Continue => {
                    self.continue_flag = true;
                    break;
                }
                ControlFlow::Throw(val) => {
                    return Err(format!("Uncaught exception: {}", val.to_string()));
                }
                ControlFlow::None => {}
            }
        }

        Ok(last_value)
    }

    fn eval_statement(&mut self, stmt: &Stmt) -> Result<ControlFlow, String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.eval_expression(expr)?;
                Ok(ControlFlow::None)
            }

            Stmt::VarDecl { name, init, kind: _ } => {
                let value = if let Some(init_expr) = init {
                    self.eval_expression(init_expr)?
                } else {
                    Value::Undefined
                };
                self.declare_variable(name, value);
                Ok(ControlFlow::None)
            }

            Stmt::Block(statements) => {
                self.push_scope();
                for stmt in statements {
                    if self.return_value.is_some() || self.break_flag || self.continue_flag {
                        break;
                    }
                    match self.eval_statement(stmt)? {
                        ControlFlow::None => {}
                        flow @ (ControlFlow::Return(_) | ControlFlow::Break | ControlFlow::Continue | ControlFlow::Throw(_)) => {
                            self.pop_scope();
                            return Ok(flow);
                        }
                    }
                }
                self.pop_scope();
                Ok(ControlFlow::None)
            }

            Stmt::If { test, consequent, alternate } => {
                let test_value = self.eval_expression(test)?;
                if test_value.to_boolean() {
                    self.eval_statement(consequent)
                } else if let Some(alt) = alternate {
                    self.eval_statement(alt)
                } else {
                    Ok(ControlFlow::None)
                }
            }

            Stmt::While { test, body } => {
                let was_in_loop = self.in_loop;
                self.in_loop = true;

                loop {
                    let test_value = self.eval_expression(test)?;
                    if !test_value.to_boolean() {
                        break;
                    }

                    match self.eval_statement(body)? {
                        ControlFlow::Break => {
                            self.break_flag = false;
                            break;
                        }
                        ControlFlow::Continue => {
                            self.continue_flag = false;
                            continue;
                        }
                        ControlFlow::Return(val) => {
                            self.in_loop = was_in_loop;
                            return Ok(ControlFlow::Return(val));
                        }
                        ControlFlow::Throw(val) => {
                            self.in_loop = was_in_loop;
                            return Ok(ControlFlow::Throw(val));
                        }
                        ControlFlow::None => {}
                    }
                }

                self.in_loop = was_in_loop;
                Ok(ControlFlow::None)
            }

            Stmt::ForIn { variable, object, body } => {
                self.push_scope();
                let was_in_loop = self.in_loop;
                self.in_loop = true;

                let obj = self.eval_expression(object)?;

                // Get keys to iterate over
                let keys: Vec<String> = match &obj {
                    Value::Object(map) => map.borrow().keys().cloned().collect(),
                    Value::Array(arr) => {
                        // For arrays, iterate over indices
                        (0..arr.borrow().len()).map(|i| i.to_string()).collect()
                    }
                    _ => Vec::new(),
                };

                for key in keys {
                    self.declare_variable(variable, Value::String(key));

                    match self.eval_statement(body)? {
                        ControlFlow::Break => {
                            self.break_flag = false;
                            break;
                        }
                        ControlFlow::Continue => {
                            self.continue_flag = false;
                            continue;
                        }
                        ControlFlow::Return(val) => {
                            self.in_loop = was_in_loop;
                            self.pop_scope();
                            return Ok(ControlFlow::Return(val));
                        }
                        ControlFlow::Throw(val) => {
                            self.in_loop = was_in_loop;
                            self.pop_scope();
                            return Ok(ControlFlow::Throw(val));
                        }
                        ControlFlow::None => {}
                    }
                }

                self.in_loop = was_in_loop;
                self.pop_scope();
                Ok(ControlFlow::None)
            }

            Stmt::For { init, test, update, body } => {
                self.push_scope();
                let was_in_loop = self.in_loop;
                self.in_loop = true;

                if let Some(init_stmt) = init {
                    self.eval_statement(init_stmt)?;
                }

                loop {
                    if let Some(test_expr) = test {
                        let test_value = self.eval_expression(test_expr)?;
                        if !test_value.to_boolean() {
                            break;
                        }
                    }

                    match self.eval_statement(body)? {
                        ControlFlow::Break => {
                            self.break_flag = false;
                            break;
                        }
                        ControlFlow::Continue => {
                            self.continue_flag = false;
                        }
                        ControlFlow::Return(val) => {
                            self.in_loop = was_in_loop;
                            self.pop_scope();
                            return Ok(ControlFlow::Return(val));
                        }
                        ControlFlow::Throw(val) => {
                            self.in_loop = was_in_loop;
                            self.pop_scope();
                            return Ok(ControlFlow::Throw(val));
                        }
                        ControlFlow::None => {}
                    }

                    if let Some(update_expr) = update {
                        self.eval_expression(update_expr)?;
                    }
                }

                self.in_loop = was_in_loop;
                self.pop_scope();
                Ok(ControlFlow::None)
            }

            Stmt::FunctionDecl { name, params, body } => {
                let func = Function {
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.current_scope(),
                    constructor: None,
                };
                self.declare_variable(name, Value::Function(func));
                Ok(ControlFlow::None)
            }

            Stmt::Return(expr) => {
                let value = if let Some(expr) = expr {
                    self.eval_expression(expr)?
                } else {
                    Value::Undefined
                };
                Ok(ControlFlow::Return(value))
            }

            Stmt::Break => {
                if !self.in_loop {
                    return Err("Break outside of loop".to_string());
                }
                Ok(ControlFlow::Break)
            }

            Stmt::Continue => {
                if !self.in_loop {
                    return Err("Continue outside of loop".to_string());
                }
                Ok(ControlFlow::Continue)
            }

            Stmt::Throw(expr) => {
                let value = self.eval_expression(expr)?;
                Ok(ControlFlow::Throw(value))
            }

            Stmt::Try { block, catch_param, catch_block, finally_block } => {
                // Execute try block
                let mut try_result = Ok(ControlFlow::None);
                for stmt in block {
                    match self.eval_statement(stmt) {
                        Ok(ControlFlow::None) => {}
                        Ok(ControlFlow::Throw(err_val)) => {
                            // Exception thrown, handle in catch
                            if let Some(catch_stmts) = catch_block {
                                self.push_scope();

                                // Bind error to catch parameter
                                if let Some(param) = catch_param {
                                    self.declare_variable(param, err_val);
                                }

                                // Execute catch block
                                for catch_stmt in catch_stmts {
                                    match self.eval_statement(catch_stmt)? {
                                        ControlFlow::None => {}
                                        flow => {
                                            self.pop_scope();
                                            try_result = Ok(flow);
                                            break;
                                        }
                                    }
                                }

                                self.pop_scope();
                                try_result = Ok(ControlFlow::None);
                            } else {
                                // No catch block, re-throw
                                try_result = Ok(ControlFlow::Throw(err_val));
                            }
                            break;
                        }
                        Ok(flow) => {
                            try_result = Ok(flow);
                            break;
                        }
                        Err(e) => {
                            // Runtime error, handle as exception
                            if let Some(catch_stmts) = catch_block {
                                self.push_scope();

                                if let Some(param) = catch_param {
                                    self.declare_variable(param, Value::String(e.clone()));
                                }

                                for catch_stmt in catch_stmts {
                                    match self.eval_statement(catch_stmt)? {
                                        ControlFlow::None => {}
                                        flow => {
                                            self.pop_scope();
                                            try_result = Ok(flow);
                                            break;
                                        }
                                    }
                                }

                                self.pop_scope();
                                try_result = Ok(ControlFlow::None);
                            } else {
                                try_result = Err(e);
                            }
                            break;
                        }
                    }
                }

                // Execute finally block
                if let Some(finally_stmts) = finally_block {
                    for finally_stmt in finally_stmts {
                        match self.eval_statement(finally_stmt)? {
                            ControlFlow::None => {}
                            flow => return Ok(flow), // Finally overrides try/catch result
                        }
                    }
                }

                try_result
            }

            Stmt::Empty => Ok(ControlFlow::None),
        }
    }

    fn eval_expression(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Null => Ok(Value::Null),
            Expr::Undefined => Ok(Value::Undefined),

            Expr::Identifier(name) => self.get_variable(name),
            Expr::This => Ok(self.this_value.clone()),

            Expr::BinaryOp { left, op, right } => {
                let left_val = self.eval_expression(left)?;

                // Short-circuit evaluation for logical operators
                if *op == BinOp::And {
                    if !left_val.to_boolean() {
                        return Ok(left_val);
                    }
                    return self.eval_expression(right);
                }

                if *op == BinOp::Or {
                    if left_val.to_boolean() {
                        return Ok(left_val);
                    }
                    return self.eval_expression(right);
                }

                let right_val = self.eval_expression(right)?;
                self.eval_binary_op(&left_val, *op, &right_val)
            }

            Expr::UnaryOp { op, expr } => {
                let val = self.eval_expression(expr)?;
                self.eval_unary_op(*op, &val)
            }

            Expr::Assignment { target, value } => {
                let val = self.eval_expression(value)?;
                self.set_variable(target, val.clone());
                Ok(val)
            }

            Expr::CompoundAssignment { target, op, value } => {
                let current = self.get_variable(target)?;
                let rhs = self.eval_expression(value)?;
                let new_val = self.eval_binary_op(&current, *op, &rhs)?;
                self.set_variable(target, new_val.clone());
                Ok(new_val)
            }

            Expr::LogicalAssignment { target, op, value } => {
                let current = self.get_variable(target)?;

                // Short-circuit evaluation
                let should_assign = match op {
                    LogicalAssignOp::And => current.to_boolean(), // &&=: assign if current is truthy
                    LogicalAssignOp::Or => !current.to_boolean(), // ||=: assign if current is falsy
                };

                if should_assign {
                    let new_val = self.eval_expression(value)?;
                    self.set_variable(target, new_val.clone());
                    Ok(new_val)
                } else {
                    Ok(current)
                }
            }

            Expr::Update { expr, op, prefix } => {
                // Get the current value
                let name = match expr.as_ref() {
                    Expr::Identifier(n) => n,
                    _ => return Err("Update expression requires an identifier".to_string()),
                };

                let current = self.get_variable(name)?;
                let current_num = current.to_number()?;

                let new_num = match op {
                    UpdateOp::Increment => current_num + 1.0,
                    UpdateOp::Decrement => current_num - 1.0,
                };

                let new_val = Value::Number(new_num);
                self.set_variable(name, new_val.clone());

                // Return old value for postfix, new value for prefix
                if *prefix {
                    Ok(new_val)
                } else {
                    Ok(current)
                }
            }

            Expr::MemberAssignment { object, property, computed, value } => {
                let obj = self.eval_expression(object)?;
                let prop_name = if *computed {
                    self.eval_expression(property)?.to_string()
                } else {
                    match property.as_ref() {
                        Expr::String(s) => s.clone(),
                        _ => return Err("Invalid property access".to_string()),
                    }
                };
                let val = self.eval_expression(value)?;

                match obj {
                    Value::Object(map) => {
                        map.borrow_mut().insert(prop_name, val.clone());
                        Ok(val)
                    }
                    Value::Array(arr) => {
                        if let Ok(idx) = prop_name.parse::<usize>() {
                            let mut arr_mut = arr.borrow_mut();
                            // Extend array if necessary
                            if idx >= arr_mut.len() {
                                arr_mut.resize(idx + 1, Value::Undefined);
                            }
                            arr_mut[idx] = val.clone();
                            Ok(val)
                        } else {
                            Err(format!("Cannot set property '{}' on array", prop_name))
                        }
                    }
                    _ => Err("Cannot set property on non-object".to_string()),
                }
            }

            Expr::CompoundMemberAssignment { object, property, computed, op, value } => {
                let obj = self.eval_expression(object)?;
                let prop_name = if *computed {
                    self.eval_expression(property)?.to_string()
                } else {
                    match property.as_ref() {
                        Expr::String(s) => s.clone(),
                        _ => return Err("Invalid property access".to_string()),
                    }
                };

                // Get current value
                let current = match &obj {
                    Value::Object(map) => {
                        map.borrow().get(&prop_name).cloned().unwrap_or(Value::Undefined)
                    }
                    Value::Array(arr) => {
                        if let Ok(idx) = prop_name.parse::<usize>() {
                            arr.borrow().get(idx).cloned().unwrap_or(Value::Undefined)
                        } else {
                            return Err(format!("Cannot get property '{}' on array", prop_name));
                        }
                    }
                    _ => return Err("Cannot access property on non-object".to_string()),
                };

                // Evaluate new value
                let rhs = self.eval_expression(value)?;
                let new_val = self.eval_binary_op(&current, *op, &rhs)?;

                // Set the property
                match obj {
                    Value::Object(map) => {
                        map.borrow_mut().insert(prop_name, new_val.clone());
                        Ok(new_val)
                    }
                    Value::Array(arr) => {
                        if let Ok(idx) = prop_name.parse::<usize>() {
                            let mut arr_mut = arr.borrow_mut();
                            if idx >= arr_mut.len() {
                                arr_mut.resize(idx + 1, Value::Undefined);
                            }
                            arr_mut[idx] = new_val.clone();
                            Ok(new_val)
                        } else {
                            Err(format!("Cannot set property '{}' on array", prop_name))
                        }
                    }
                    _ => Err("Cannot set property on non-object".to_string()),
                }
            }

            Expr::LogicalMemberAssignment { object, property, computed, op, value } => {
                let obj = self.eval_expression(object)?;
                let prop_name = if *computed {
                    self.eval_expression(property)?.to_string()
                } else {
                    match property.as_ref() {
                        Expr::String(s) => s.clone(),
                        _ => return Err("Invalid property access".to_string()),
                    }
                };

                // Get current value
                let current = match &obj {
                    Value::Object(map) => {
                        map.borrow().get(&prop_name).cloned().unwrap_or(Value::Undefined)
                    }
                    Value::Array(arr) => {
                        if let Ok(idx) = prop_name.parse::<usize>() {
                            arr.borrow().get(idx).cloned().unwrap_or(Value::Undefined)
                        } else {
                            return Err(format!("Cannot get property '{}' on array", prop_name));
                        }
                    }
                    _ => return Err("Cannot access property on non-object".to_string()),
                };

                // Short-circuit evaluation
                let new_val = match op {
                    LogicalAssignOp::And => {
                        if !current.to_boolean() {
                            current
                        } else {
                            self.eval_expression(value)?
                        }
                    }
                    LogicalAssignOp::Or => {
                        if current.to_boolean() {
                            current
                        } else {
                            self.eval_expression(value)?
                        }
                    }
                };

                // Set the property
                match obj {
                    Value::Object(map) => {
                        map.borrow_mut().insert(prop_name, new_val.clone());
                        Ok(new_val)
                    }
                    Value::Array(arr) => {
                        if let Ok(idx) = prop_name.parse::<usize>() {
                            let mut arr_mut = arr.borrow_mut();
                            if idx >= arr_mut.len() {
                                arr_mut.resize(idx + 1, Value::Undefined);
                            }
                            arr_mut[idx] = new_val.clone();
                            Ok(new_val)
                        } else {
                            Err(format!("Cannot set property '{}' on array", prop_name))
                        }
                    }
                    _ => Err("Cannot set property on non-object".to_string()),
                }
            }

            Expr::Call { callee, args } => {
                // Special handling for array iteration methods (map, filter, reduce)
                if let Expr::Member { object, property, computed } = callee.as_ref() {
                    let obj = self.eval_expression(object)?;
                    let prop_name = if *computed {
                        self.eval_expression(property)?.to_string()
                    } else {
                        match property.as_ref() {
                            Expr::String(s) => s.clone(),
                            _ => String::new(),
                        }
                    };

                    if let Value::Array(arr) = &obj {
                        match prop_name.as_str() {
                            "map" => {
                                if args.is_empty() {
                                    return Err("map requires a callback function".to_string());
                                }
                                let callback = self.eval_expression(&args[0])?;
                                let arr_borrow = arr.borrow();
                                let mut result = vec![];
                                for (i, val) in arr_borrow.iter().enumerate() {
                                    let call_args = vec![val.clone(), Value::Number(i as f64), Value::Array(arr.clone())];
                                    let mapped = match &callback {
                                        Value::Function(f) => self.call_function(f, &call_args, Value::Undefined)?,
                                        Value::NativeFunction(nf) => (nf.func)(&call_args)?,
                                        _ => return Err("map callback must be a function".to_string()),
                                    };
                                    result.push(mapped);
                                }
                                return Ok(Value::Array(Rc::new(RefCell::new(result))));
                            }
                            "filter" => {
                                if args.is_empty() {
                                    return Err("filter requires a callback function".to_string());
                                }
                                let callback = self.eval_expression(&args[0])?;
                                let arr_borrow = arr.borrow();
                                let mut result = vec![];
                                for (i, val) in arr_borrow.iter().enumerate() {
                                    let call_args = vec![val.clone(), Value::Number(i as f64), Value::Array(arr.clone())];
                                    let keep = match &callback {
                                        Value::Function(f) => self.call_function(f, &call_args, Value::Undefined)?,
                                        Value::NativeFunction(nf) => (nf.func)(&call_args)?,
                                        _ => return Err("filter callback must be a function".to_string()),
                                    };
                                    if keep.to_boolean() {
                                        result.push(val.clone());
                                    }
                                }
                                return Ok(Value::Array(Rc::new(RefCell::new(result))));
                            }
                            "reduce" => {
                                if args.is_empty() {
                                    return Err("reduce requires a callback function".to_string());
                                }
                                let callback = self.eval_expression(&args[0])?;
                                let arr_borrow = arr.borrow();
                                if arr_borrow.is_empty() && args.len() < 2 {
                                    return Err("reduce of empty array with no initial value".to_string());
                                }
                                let mut accumulator = if args.len() > 1 {
                                    self.eval_expression(&args[1])?
                                } else {
                                    arr_borrow[0].clone()
                                };
                                let start_idx = if args.len() > 1 { 0 } else { 1 };
                                for (i, val) in arr_borrow.iter().enumerate().skip(start_idx) {
                                    let call_args = vec![accumulator, val.clone(), Value::Number(i as f64), Value::Array(arr.clone())];
                                    accumulator = match &callback {
                                        Value::Function(f) => self.call_function(f, &call_args, Value::Undefined)?,
                                        Value::NativeFunction(nf) => (nf.func)(&call_args)?,
                                        _ => return Err("reduce callback must be a function".to_string()),
                                    };
                                }
                                return Ok(accumulator);
                            }
                            "find" => {
                                if args.is_empty() {
                                    return Err("find requires a callback function".to_string());
                                }
                                let callback = self.eval_expression(&args[0])?;
                                let arr_borrow = arr.borrow();
                                for (i, val) in arr_borrow.iter().enumerate() {
                                    let call_args = vec![val.clone(), Value::Number(i as f64), Value::Array(arr.clone())];
                                    let found = match &callback {
                                        Value::Function(f) => self.call_function(f, &call_args, Value::Undefined)?,
                                        Value::NativeFunction(nf) => (nf.func)(&call_args)?,
                                        _ => return Err("find callback must be a function".to_string()),
                                    };
                                    if found.to_boolean() {
                                        return Ok(val.clone());
                                    }
                                }
                                return Ok(Value::Undefined);
                            }
                            "findIndex" => {
                                if args.is_empty() {
                                    return Err("findIndex requires a callback function".to_string());
                                }
                                let callback = self.eval_expression(&args[0])?;
                                let arr_borrow = arr.borrow();
                                for (i, val) in arr_borrow.iter().enumerate() {
                                    let call_args = vec![val.clone(), Value::Number(i as f64), Value::Array(arr.clone())];
                                    let found = match &callback {
                                        Value::Function(f) => self.call_function(f, &call_args, Value::Undefined)?,
                                        Value::NativeFunction(nf) => (nf.func)(&call_args)?,
                                        _ => return Err("findIndex callback must be a function".to_string()),
                                    };
                                    if found.to_boolean() {
                                        return Ok(Value::Number(i as f64));
                                    }
                                }
                                return Ok(Value::Number(-1.0));
                            }
                            "some" => {
                                if args.is_empty() {
                                    return Err("some requires a callback function".to_string());
                                }
                                let callback = self.eval_expression(&args[0])?;
                                let arr_borrow = arr.borrow();
                                for (i, val) in arr_borrow.iter().enumerate() {
                                    let call_args = vec![val.clone(), Value::Number(i as f64), Value::Array(arr.clone())];
                                    let result = match &callback {
                                        Value::Function(f) => self.call_function(f, &call_args, Value::Undefined)?,
                                        Value::NativeFunction(nf) => (nf.func)(&call_args)?,
                                        _ => return Err("some callback must be a function".to_string()),
                                    };
                                    if result.to_boolean() {
                                        return Ok(Value::Boolean(true));
                                    }
                                }
                                return Ok(Value::Boolean(false));
                            }
                            "every" => {
                                if args.is_empty() {
                                    return Err("every requires a callback function".to_string());
                                }
                                let callback = self.eval_expression(&args[0])?;
                                let arr_borrow = arr.borrow();
                                for (i, val) in arr_borrow.iter().enumerate() {
                                    let call_args = vec![val.clone(), Value::Number(i as f64), Value::Array(arr.clone())];
                                    let result = match &callback {
                                        Value::Function(f) => self.call_function(f, &call_args, Value::Undefined)?,
                                        Value::NativeFunction(nf) => (nf.func)(&call_args)?,
                                        _ => return Err("every callback must be a function".to_string()),
                                    };
                                    if !result.to_boolean() {
                                        return Ok(Value::Boolean(false));
                                    }
                                }
                                return Ok(Value::Boolean(true));
                            }
                            "forEach" => {
                                if args.is_empty() {
                                    return Err("forEach requires a callback function".to_string());
                                }
                                let callback = self.eval_expression(&args[0])?;
                                let arr_borrow = arr.borrow();
                                for (i, val) in arr_borrow.iter().enumerate() {
                                    let call_args = vec![val.clone(), Value::Number(i as f64), Value::Array(arr.clone())];
                                    match &callback {
                                        Value::Function(f) => { self.call_function(f, &call_args, Value::Undefined)?; },
                                        Value::NativeFunction(nf) => { (nf.func)(&call_args)?; },
                                        _ => return Err("forEach callback must be a function".to_string()),
                                    };
                                }
                                return Ok(Value::Undefined);
                            }
                            _ => {}
                        }
                    }
                }

                // Regular function call
                let func = self.eval_expression(callee)?;
                let arg_values: Result<Vec<Value>, String> =
                    args.iter().map(|arg| self.eval_expression(arg)).collect();
                let arg_values = arg_values?;

                // Determine 'this' value for method calls
                let this_val = if let Expr::Member { object, .. } = callee.as_ref() {
                    // Method call: obj.method()
                    self.eval_expression(object)?
                } else {
                    Value::Undefined
                };

                match func {
                    Value::Function(f) => self.call_function(&f, &arg_values, this_val),
                    Value::NativeFunction(nf) => (nf.func)(&arg_values),
                    _ => Err("Not a function".to_string()),
                }
            }

            Expr::New { callee, args } => {
                let func = self.eval_expression(callee)?;
                let arg_values: Result<Vec<Value>, String> =
                    args.iter().map(|arg| self.eval_expression(arg)).collect();
                let arg_values = arg_values?;

                // Get constructor name if available
                let constructor_name = if let Expr::Identifier(name) = callee.as_ref() {
                    Some(name.clone())
                } else {
                    None
                };

                match func {
                    Value::Function(f) => {
                        // Create new instance
                        let instance = Rc::new(RefCell::new(HashMap::new()));

                        // Store constructor name for instanceof checks
                        if let Some(name) = constructor_name {
                            instance.borrow_mut().insert("__constructor__".to_string(), Value::String(name));
                        }

                        let this_val = Value::Object(instance);

                        // Call constructor with new instance as 'this'
                        let result = self.call_function(&f, &arg_values, this_val.clone())?;

                        // If constructor returns an object, use it; otherwise use the instance
                        match result {
                            Value::Object(_) => Ok(result),
                            _ => Ok(this_val),
                        }
                    }
                    _ => Err("Constructor must be a function".to_string()),
                }
            }

            Expr::Member { object, property, computed } => {
                let obj = self.eval_expression(object)?;
                let prop_name = if *computed {
                    self.eval_expression(property)?.to_string()
                } else {
                    match property.as_ref() {
                        Expr::String(s) => s.clone(),
                        _ => return Err("Invalid property access".to_string()),
                    }
                };

                match obj {
                    Value::Object(map) => {
                        // Check for hasOwnProperty method
                        if prop_name == "hasOwnProperty" {
                            let map_clone = map.clone();
                            let has_own_fn: NativeFn = Rc::new(move |args| {
                                if args.is_empty() {
                                    return Ok(Value::Boolean(false));
                                }
                                let key = args[0].to_string();
                                Ok(Value::Boolean(map_clone.borrow().contains_key(&key)))
                            });
                            return Ok(Value::NativeFunction(NativeFunction {
                                name: "hasOwnProperty".to_string(),
                                func: has_own_fn,
                            }));
                        }
                        Ok(map.borrow().get(&prop_name).cloned().unwrap_or(Value::Undefined))
                    }
                    Value::Array(arr) => {
                        if prop_name == "length" {
                            Ok(Value::Number(arr.borrow().len() as f64))
                        } else if let Ok(idx) = prop_name.parse::<usize>() {
                            Ok(arr.borrow().get(idx).cloned().unwrap_or(Value::Undefined))
                        } else {
                            // Array methods
                            match prop_name.as_str() {
                                "push" => {
                                    let arr_clone = arr.clone();
                                    let push_fn: NativeFn = Rc::new(move |args| {
                                        for arg in args {
                                            arr_clone.borrow_mut().push(arg.clone());
                                        }
                                        Ok(Value::Number(arr_clone.borrow().len() as f64))
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "push".to_string(),
                                        func: push_fn,
                                    }))
                                }
                                "pop" => {
                                    let arr_clone = arr.clone();
                                    let pop_fn: NativeFn = Rc::new(move |_args| {
                                        Ok(arr_clone.borrow_mut().pop().unwrap_or(Value::Undefined))
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "pop".to_string(),
                                        func: pop_fn,
                                    }))
                                }
                                "shift" => {
                                    let arr_clone = arr.clone();
                                    let shift_fn: NativeFn = Rc::new(move |_args| {
                                        let mut arr_mut = arr_clone.borrow_mut();
                                        if arr_mut.is_empty() {
                                            Ok(Value::Undefined)
                                        } else {
                                            Ok(arr_mut.remove(0))
                                        }
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "shift".to_string(),
                                        func: shift_fn,
                                    }))
                                }
                                "unshift" => {
                                    let arr_clone = arr.clone();
                                    let unshift_fn: NativeFn = Rc::new(move |args| {
                                        let mut arr_mut = arr_clone.borrow_mut();
                                        for (i, arg) in args.iter().enumerate() {
                                            arr_mut.insert(i, arg.clone());
                                        }
                                        Ok(Value::Number(arr_mut.len() as f64))
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "unshift".to_string(),
                                        func: unshift_fn,
                                    }))
                                }
                                "indexOf" => {
                                    let arr_clone = arr.clone();
                                    let indexof_fn: NativeFn = Rc::new(move |args| {
                                        if args.is_empty() {
                                            return Ok(Value::Number(-1.0));
                                        }
                                        let search = &args[0];
                                        let arr_borrow = arr_clone.borrow();
                                        for (i, val) in arr_borrow.iter().enumerate() {
                                            if val.strict_equals(search) {
                                                return Ok(Value::Number(i as f64));
                                            }
                                        }
                                        Ok(Value::Number(-1.0))
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "indexOf".to_string(),
                                        func: indexof_fn,
                                    }))
                                }
                                "slice" => {
                                    let arr_clone = arr.clone();
                                    let slice_fn: NativeFn = Rc::new(move |args| {
                                        let arr_borrow = arr_clone.borrow();
                                        let len = arr_borrow.len() as i32;
                                        let start = if args.is_empty() {
                                            0
                                        } else {
                                            let s = args[0].to_number().unwrap_or(0.0) as i32;
                                            if s < 0 { (len + s).max(0) } else { s.min(len) }
                                        };
                                        let end = if args.len() < 2 {
                                            len
                                        } else {
                                            let e = args[1].to_number().unwrap_or(len as f64) as i32;
                                            if e < 0 { (len + e).max(0) } else { e.min(len) }
                                        };
                                        let result: Vec<Value> = arr_borrow[start as usize..end.max(start) as usize].to_vec();
                                        Ok(Value::Array(Rc::new(RefCell::new(result))))
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "slice".to_string(),
                                        func: slice_fn,
                                    }))
                                }
                                "splice" => {
                                    let arr_clone = arr.clone();
                                    let splice_fn: NativeFn = Rc::new(move |args| {
                                        let mut arr_mut = arr_clone.borrow_mut();
                                        if args.is_empty() {
                                            return Ok(Value::Array(Rc::new(RefCell::new(vec![]))));
                                        }
                                        let start = args[0].to_number().unwrap_or(0.0) as usize;
                                        let delete_count = if args.len() < 2 {
                                            arr_mut.len().saturating_sub(start)
                                        } else {
                                            args[1].to_number().unwrap_or(0.0) as usize
                                        };
                                        let mut removed = vec![];
                                        for _ in 0..delete_count.min(arr_mut.len().saturating_sub(start)) {
                                            if start < arr_mut.len() {
                                                removed.push(arr_mut.remove(start));
                                            }
                                        }
                                        for (i, item) in args[2..].iter().enumerate() {
                                            arr_mut.insert(start + i, item.clone());
                                        }
                                        Ok(Value::Array(Rc::new(RefCell::new(removed))))
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "splice".to_string(),
                                        func: splice_fn,
                                    }))
                                }
                                "join" => {
                                    let arr_clone = arr.clone();
                                    let join_fn: NativeFn = Rc::new(move |args| {
                                        let separator = if args.is_empty() {
                                            ","
                                        } else {
                                            &args[0].to_string()
                                        };
                                        let joined = arr_clone.borrow().iter()
                                            .map(|v| v.to_string())
                                            .collect::<Vec<_>>()
                                            .join(separator);
                                        Ok(Value::String(joined))
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "join".to_string(),
                                        func: join_fn,
                                    }))
                                }
                                "reverse" => {
                                    let arr_clone = arr.clone();
                                    let reverse_fn: NativeFn = Rc::new(move |_args| {
                                        let mut arr_mut = arr_clone.borrow_mut();
                                        arr_mut.reverse();
                                        Ok(Value::Array(arr_clone.clone()))
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "reverse".to_string(),
                                        func: reverse_fn,
                                    }))
                                }
                                "concat" => {
                                    let arr_clone = arr.clone();
                                    let concat_fn: NativeFn = Rc::new(move |args| {
                                        let mut result = arr_clone.borrow().clone();
                                        for arg in args {
                                            match arg {
                                                Value::Array(other_arr) => {
                                                    result.extend(other_arr.borrow().clone());
                                                }
                                                _ => result.push(arg.clone()),
                                            }
                                        }
                                        Ok(Value::Array(Rc::new(RefCell::new(result))))
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "concat".to_string(),
                                        func: concat_fn,
                                    }))
                                }
                                "includes" => {
                                    let arr_clone = arr.clone();
                                    let includes_fn: NativeFn = Rc::new(move |args| {
                                        if args.is_empty() {
                                            return Ok(Value::Boolean(false));
                                        }
                                        let search = &args[0];
                                        for val in arr_clone.borrow().iter() {
                                            if val.strict_equals(search) {
                                                return Ok(Value::Boolean(true));
                                            }
                                        }
                                        Ok(Value::Boolean(false))
                                    });
                                    Ok(Value::NativeFunction(NativeFunction {
                                        name: "includes".to_string(),
                                        func: includes_fn,
                                    }))
                                }
                                _ => Ok(Value::Undefined),
                            }
                        }
                    }
                    Value::String(s) => {
                        // String methods
                        match prop_name.as_str() {
                            "length" => Ok(Value::Number(s.len() as f64)),
                            "toLowerCase" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |_args| {
                                    Ok(Value::String(s_clone.to_lowercase()))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "toLowerCase".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "toUpperCase" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |_args| {
                                    Ok(Value::String(s_clone.to_uppercase()))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "toUpperCase".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "split" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.is_empty() {
                                        return Ok(Value::Array(Rc::new(RefCell::new(vec![Value::String(s_clone.clone())]))));
                                    }
                                    let separator = args[0].to_string();
                                    let parts: Vec<Value> = if separator.is_empty() {
                                        s_clone.chars().map(|c| Value::String(c.to_string())).collect()
                                    } else {
                                        s_clone.split(&separator).map(|s| Value::String(s.to_string())).collect()
                                    };
                                    Ok(Value::Array(Rc::new(RefCell::new(parts))))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "split".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "substring" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    let len = s_clone.len();
                                    let start = if args.is_empty() {
                                        0
                                    } else {
                                        (args[0].to_number().unwrap_or(0.0) as usize).min(len)
                                    };
                                    let end = if args.len() < 2 {
                                        len
                                    } else {
                                        (args[1].to_number().unwrap_or(len as f64) as usize).min(len)
                                    };
                                    let (start, end) = if start > end { (end, start) } else { (start, end) };
                                    Ok(Value::String(s_clone.chars().skip(start).take(end - start).collect()))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "substring".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "slice" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    let len = s_clone.len() as i32;
                                    let start = if args.is_empty() {
                                        0
                                    } else {
                                        let s = args[0].to_number().unwrap_or(0.0) as i32;
                                        if s < 0 { (len + s).max(0) } else { s.min(len) }
                                    };
                                    let end = if args.len() < 2 {
                                        len
                                    } else {
                                        let e = args[1].to_number().unwrap_or(len as f64) as i32;
                                        if e < 0 { (len + e).max(0) } else { e.min(len) }
                                    };
                                    let result: String = s_clone.chars().skip(start as usize).take((end - start).max(0) as usize).collect();
                                    Ok(Value::String(result))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "slice".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "indexOf" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.is_empty() {
                                        return Ok(Value::Number(-1.0));
                                    }
                                    let search = args[0].to_string();
                                    match s_clone.find(&search) {
                                        Some(idx) => Ok(Value::Number(idx as f64)),
                                        None => Ok(Value::Number(-1.0)),
                                    }
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "indexOf".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "charAt" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.is_empty() {
                                        return Ok(Value::String(String::new()));
                                    }
                                    let idx = args[0].to_number().unwrap_or(0.0) as usize;
                                    let ch = s_clone.chars().nth(idx).map(|c| c.to_string()).unwrap_or_default();
                                    Ok(Value::String(ch))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "charAt".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "charCodeAt" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.is_empty() {
                                        return Ok(Value::Number(f64::NAN));
                                    }
                                    let idx = args[0].to_number().unwrap_or(0.0) as usize;
                                    match s_clone.chars().nth(idx) {
                                        Some(ch) => Ok(Value::Number(ch as u32 as f64)),
                                        None => Ok(Value::Number(f64::NAN)),
                                    }
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "charCodeAt".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "trim" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |_args| {
                                    Ok(Value::String(s_clone.trim().to_string()))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "trim".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "replace" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.len() < 2 {
                                        return Ok(Value::String(s_clone.clone()));
                                    }
                                    let search = args[0].to_string();
                                    let replace_with = args[1].to_string();
                                    // Simple replace - only replaces first occurrence
                                    let result = s_clone.replacen(&search, &replace_with, 1);
                                    Ok(Value::String(result))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "replace".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "startsWith" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.is_empty() {
                                        return Ok(Value::Boolean(false));
                                    }
                                    let search = args[0].to_string();
                                    Ok(Value::Boolean(s_clone.starts_with(&search)))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "startsWith".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "endsWith" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.is_empty() {
                                        return Ok(Value::Boolean(false));
                                    }
                                    let search = args[0].to_string();
                                    Ok(Value::Boolean(s_clone.ends_with(&search)))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "endsWith".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "includes" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.is_empty() {
                                        return Ok(Value::Boolean(false));
                                    }
                                    let search = args[0].to_string();
                                    Ok(Value::Boolean(s_clone.contains(&search)))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "includes".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "repeat" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.is_empty() {
                                        return Ok(Value::String(String::new()));
                                    }
                                    let count = args[0].to_number().unwrap_or(0.0) as usize;
                                    Ok(Value::String(s_clone.repeat(count)))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "repeat".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "padStart" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.is_empty() {
                                        return Ok(Value::String(s_clone.clone()));
                                    }
                                    let target_len = args[0].to_number().unwrap_or(0.0) as usize;
                                    let pad_str = if args.len() > 1 {
                                        args[1].to_string()
                                    } else {
                                        " ".to_string()
                                    };
                                    let current_len = s_clone.chars().count();
                                    if current_len >= target_len {
                                        return Ok(Value::String(s_clone.clone()));
                                    }
                                    let pad_len = target_len - current_len;
                                    let padding = pad_str.repeat((pad_len + pad_str.len() - 1) / pad_str.len());
                                    let padding: String = padding.chars().take(pad_len).collect();
                                    Ok(Value::String(format!("{}{}", padding, s_clone)))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "padStart".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            "padEnd" => {
                                let s_clone = s.clone();
                                let fn_impl: NativeFn = Rc::new(move |args| {
                                    if args.is_empty() {
                                        return Ok(Value::String(s_clone.clone()));
                                    }
                                    let target_len = args[0].to_number().unwrap_or(0.0) as usize;
                                    let pad_str = if args.len() > 1 {
                                        args[1].to_string()
                                    } else {
                                        " ".to_string()
                                    };
                                    let current_len = s_clone.chars().count();
                                    if current_len >= target_len {
                                        return Ok(Value::String(s_clone.clone()));
                                    }
                                    let pad_len = target_len - current_len;
                                    let padding = pad_str.repeat((pad_len + pad_str.len() - 1) / pad_str.len());
                                    let padding: String = padding.chars().take(pad_len).collect();
                                    Ok(Value::String(format!("{}{}", s_clone, padding)))
                                });
                                Ok(Value::NativeFunction(NativeFunction {
                                    name: "padEnd".to_string(),
                                    func: fn_impl,
                                }))
                            }
                            _ => Ok(Value::Undefined),
                        }
                    }
                    _ => Ok(Value::Undefined),
                }
            }

            Expr::Array(elements) => {
                let values: Result<Vec<Value>, String> =
                    elements.iter().map(|e| self.eval_expression(e)).collect();
                Ok(Value::Array(Rc::new(RefCell::new(values?))))
            }

            Expr::Object(properties) => {
                let mut map = HashMap::new();
                for (key, value_expr) in properties {
                    let value = self.eval_expression(value_expr)?;
                    map.insert(key.clone(), value);
                }
                Ok(Value::Object(Rc::new(RefCell::new(map))))
            }

            Expr::Function { params, body } => {
                let func = Function {
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.current_scope(),
                    constructor: None,
                };
                Ok(Value::Function(func))
            }

            Expr::ArrowFunction { params, body } => {
                // Simplified: treat expression body as return statement
                let return_stmt = Stmt::Return(Some(body.as_ref().clone()));
                let func = Function {
                    params: params.clone(),
                    body: vec![return_stmt],
                    closure: self.current_scope(),
                    constructor: None,
                };
                Ok(Value::Function(func))
            }

            Expr::Conditional { test, consequent, alternate } => {
                let test_val = self.eval_expression(test)?;
                if test_val.to_boolean() {
                    self.eval_expression(consequent)
                } else {
                    self.eval_expression(alternate)
                }
            }
        }
    }

    fn call_function(&mut self, func: &Function, args: &[Value], this_val: Value) -> Result<Value, String> {
        // Save current scopes and 'this' value
        let saved_scopes = self.scopes.clone();
        let saved_this = self.this_value.clone();

        // Set 'this' for this function call
        self.this_value = this_val;

        // Set up scope chain: global + closure + new function scope
        self.scopes = vec![self.global_scope.clone(), func.closure.clone()];
        self.push_scope();

        // Bind parameters
        for (i, param) in func.params.iter().enumerate() {
            let value = args.get(i).cloned().unwrap_or(Value::Undefined);
            self.declare_variable(param, value);
        }

        // Execute function body
        let mut result = Value::Undefined;
        for stmt in &func.body {
            match self.eval_statement(stmt)? {
                ControlFlow::Return(val) => {
                    result = val;
                    break;
                }
                ControlFlow::Break | ControlFlow::Continue => {
                    return Err("Break/Continue outside of loop".to_string());
                }
                ControlFlow::Throw(val) => {
                    // Restore scopes and 'this' before propagating exception
                    self.scopes = saved_scopes;
                    self.this_value = saved_this;
                    return Err(format!("Uncaught exception: {}", val.to_string()));
                }
                ControlFlow::None => {}
            }
        }

        // Clear return flag
        self.return_value = None;

        // Restore scopes and 'this' value
        self.scopes = saved_scopes;
        self.this_value = saved_this;

        Ok(result)
    }

    fn eval_binary_op(&self, left: &Value, op: BinOp, right: &Value) -> Result<Value, String> {
        match op {
            BinOp::Add => {
                // String concatenation or addition
                if matches!(left, Value::String(_)) || matches!(right, Value::String(_)) {
                    Ok(Value::String(format!("{}{}", left.to_string(), right.to_string())))
                } else {
                    Ok(Value::Number(left.to_number()? + right.to_number()?))
                }
            }
            BinOp::Sub => Ok(Value::Number(left.to_number()? - right.to_number()?)),
            BinOp::Mul => Ok(Value::Number(left.to_number()? * right.to_number()?)),
            BinOp::Div => Ok(Value::Number(left.to_number()? / right.to_number()?)),
            BinOp::Mod => Ok(Value::Number(left.to_number()? % right.to_number()?)),
            BinOp::Pow => Ok(Value::Number(left.to_number()?.powf(right.to_number()?))),

            BinOp::Eq => Ok(Value::Boolean(left.loose_equals(right))),
            BinOp::Ne => Ok(Value::Boolean(!left.loose_equals(right))),
            BinOp::StrictEq => Ok(Value::Boolean(left.strict_equals(right))),
            BinOp::StrictNe => Ok(Value::Boolean(!left.strict_equals(right))),
            BinOp::Lt => Ok(Value::Boolean(left.to_number()? < right.to_number()?)),
            BinOp::Le => Ok(Value::Boolean(left.to_number()? <= right.to_number()?)),
            BinOp::Gt => Ok(Value::Boolean(left.to_number()? > right.to_number()?)),
            BinOp::Ge => Ok(Value::Boolean(left.to_number()? >= right.to_number()?)),

            BinOp::And | BinOp::Or => {
                // These should be handled with short-circuit evaluation in eval_expression
                unreachable!("Logical operators should be short-circuited")
            }

            BinOp::InstanceOf => {
                // Check if left is an instance of right (constructor function)
                match (left, right) {
                    (Value::Object(obj), Value::Function(_)) => {
                        // For now, simplified: check if object has __constructor__ property
                        // In full JavaScript, this would check the prototype chain
                        if let Some(Value::String(_constructor_name)) = obj.borrow().get("__constructor__") {
                            // Get the constructor function name from the right side
                            // This is simplified - proper implementation needs prototype chain
                            Ok(Value::Boolean(true)) // Simplified: assume true if object was created with 'new'
                        } else {
                            Ok(Value::Boolean(false))
                        }
                    }
                    _ => Ok(Value::Boolean(false)),
                }
            }

            BinOp::In => {
                // Check if property exists in object
                match (left, right) {
                    (Value::String(prop), Value::Object(obj)) => {
                        Ok(Value::Boolean(obj.borrow().contains_key(prop)))
                    }
                    (Value::Number(n), Value::Array(arr)) => {
                        let idx = *n as usize;
                        Ok(Value::Boolean(idx < arr.borrow().len()))
                    }
                    _ => Ok(Value::Boolean(false)),
                }
            }

            BinOp::BitAnd => {
                let l = left.to_number()? as i32;
                let r = right.to_number()? as i32;
                Ok(Value::Number((l & r) as f64))
            }
            BinOp::BitOr => {
                let l = left.to_number()? as i32;
                let r = right.to_number()? as i32;
                Ok(Value::Number((l | r) as f64))
            }
            BinOp::BitXor => {
                let l = left.to_number()? as i32;
                let r = right.to_number()? as i32;
                Ok(Value::Number((l ^ r) as f64))
            }
            BinOp::Shl => {
                let l = left.to_number()? as i32;
                let r = right.to_number()? as i32;
                Ok(Value::Number((l << r) as f64))
            }
            BinOp::Shr => {
                let l = left.to_number()? as i32;
                let r = right.to_number()? as i32;
                Ok(Value::Number((l >> r) as f64))
            }
            BinOp::UShr => {
                let l = left.to_number()? as u32;
                let r = right.to_number()? as u32;
                Ok(Value::Number((l >> r) as f64))
            }
        }
    }

    fn eval_unary_op(&self, op: UnaryOp, val: &Value) -> Result<Value, String> {
        match op {
            UnaryOp::Plus => Ok(Value::Number(val.to_number()?)),
            UnaryOp::Minus => Ok(Value::Number(-val.to_number()?)),
            UnaryOp::Not => Ok(Value::Boolean(!val.to_boolean())),
            UnaryOp::BitNot => {
                let n = val.to_number()? as i32;
                Ok(Value::Number((!n) as f64))
            }
            UnaryOp::TypeOf => Ok(Value::String(val.type_of().to_string())),
        }
    }
}

// JSON stringify helper
fn value_to_json(val: &Value) -> String {
    match val {
        Value::Null => "null".to_string(),
        Value::Undefined => "null".to_string(), // JSON doesn't have undefined
        Value::Boolean(b) => b.to_string(),
        Value::Number(n) => {
            if n.is_finite() {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            } else {
                "null".to_string()
            }
        }
        Value::String(s) => {
            // Escape special characters
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t");
            format!("\"{}\"", escaped)
        }
        Value::Array(arr) => {
            let elements: Vec<String> = arr.borrow().iter()
                .map(|v| value_to_json(v))
                .collect();
            format!("[{}]", elements.join(","))
        }
        Value::Object(map) => {
            let pairs: Vec<String> = map.borrow().iter()
                .map(|(k, v)| format!("\"{}\":{}", k, value_to_json(v)))
                .collect();
            format!("{{{}}}", pairs.join(","))
        }
        Value::Function(_) | Value::NativeFunction(_) => "null".to_string(),
    }
}

// JSON parse helper - simplified implementation
fn parse_json(json: &str) -> Result<Value, String> {
    let trimmed = json.trim();

    // null
    if trimmed == "null" {
        return Ok(Value::Null);
    }

    // boolean
    if trimmed == "true" {
        return Ok(Value::Boolean(true));
    }
    if trimmed == "false" {
        return Ok(Value::Boolean(false));
    }

    // number
    if let Ok(n) = trimmed.parse::<f64>() {
        return Ok(Value::Number(n));
    }

    // string
    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        let content = &trimmed[1..trimmed.len()-1];
        // Unescape
        let unescaped = content
            .replace("\\\"", "\"")
            .replace("\\\\", "\\")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t");
        return Ok(Value::String(unescaped));
    }

    // array
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        let content = &trimmed[1..trimmed.len()-1].trim();
        if content.is_empty() {
            return Ok(Value::Array(Rc::new(RefCell::new(vec![]))));
        }

        let mut elements = vec![];
        let mut depth = 0;
        let mut current = String::new();
        let mut in_string = false;
        let mut escape_next = false;

        for ch in content.chars() {
            if escape_next {
                current.push(ch);
                escape_next = false;
                continue;
            }

            if ch == '\\' && in_string {
                current.push(ch);
                escape_next = true;
                continue;
            }

            if ch == '"' {
                in_string = !in_string;
                current.push(ch);
                continue;
            }

            if !in_string {
                if ch == '[' || ch == '{' {
                    depth += 1;
                } else if ch == ']' || ch == '}' {
                    depth -= 1;
                } else if ch == ',' && depth == 0 {
                    elements.push(parse_json(current.trim())?);
                    current.clear();
                    continue;
                }
            }

            current.push(ch);
        }

        if !current.trim().is_empty() {
            elements.push(parse_json(current.trim())?);
        }

        return Ok(Value::Array(Rc::new(RefCell::new(elements))));
    }

    // object
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        let content = &trimmed[1..trimmed.len()-1].trim();
        if content.is_empty() {
            return Ok(Value::Object(Rc::new(RefCell::new(HashMap::new()))));
        }

        let mut map = HashMap::new();
        let mut depth = 0;
        let mut current = String::new();
        let mut in_string = false;
        let mut escape_next = false;

        for ch in content.chars() {
            if escape_next {
                current.push(ch);
                escape_next = false;
                continue;
            }

            if ch == '\\' && in_string {
                current.push(ch);
                escape_next = true;
                continue;
            }

            if ch == '"' {
                in_string = !in_string;
                current.push(ch);
                continue;
            }

            if !in_string {
                if ch == '[' || ch == '{' {
                    depth += 1;
                } else if ch == ']' || ch == '}' {
                    depth -= 1;
                } else if ch == ',' && depth == 0 {
                    parse_key_value(&current, &mut map)?;
                    current.clear();
                    continue;
                }
            }

            current.push(ch);
        }

        if !current.trim().is_empty() {
            parse_key_value(&current, &mut map)?;
        }

        return Ok(Value::Object(Rc::new(RefCell::new(map))));
    }

    Err(format!("Invalid JSON: {}", trimmed))
}

fn parse_key_value(pair: &str, map: &mut HashMap<String, Value>) -> Result<(), String> {
    let parts: Vec<&str> = pair.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid key-value pair: {}", pair));
    }

    let key = parts[0].trim();
    if !key.starts_with('"') || !key.ends_with('"') {
        return Err(format!("Invalid key: {}", key));
    }
    let key = &key[1..key.len()-1];

    let value = parse_json(parts[1].trim())?;
    map.insert(key.to_string(), value);
    Ok(())
}
