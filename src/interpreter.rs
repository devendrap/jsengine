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
        let console_log: NativeFn = |args| {
            let output: Vec<String> = args.iter().map(|v| v.to_string()).collect();
            println!("{}", output.join(" "));
            Ok(Value::Undefined)
        };

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
        let parse_int: NativeFn = |args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::NAN));
            }
            let s = args[0].to_string();
            match s.trim().parse::<f64>() {
                Ok(n) => Ok(Value::Number(n.trunc())),
                Err(_) => Ok(Value::Number(f64::NAN)),
            }
        };

        let parse_float: NativeFn = |args| {
            if args.is_empty() {
                return Ok(Value::Number(f64::NAN));
            }
            match args[0].to_number() {
                Ok(n) => Ok(Value::Number(n)),
                Err(_) => Ok(Value::Number(f64::NAN)),
            }
        };

        let is_nan: NativeFn = |args| {
            if args.is_empty() {
                return Ok(Value::Boolean(true));
            }
            match args[0].to_number() {
                Ok(n) => Ok(Value::Boolean(n.is_nan())),
                Err(_) => Ok(Value::Boolean(true)),
            }
        };

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

            Expr::Call { callee, args } => {
                // Evaluate the callee
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
                        Ok(map.borrow().get(&prop_name).cloned().unwrap_or(Value::Undefined))
                    }
                    Value::Array(arr) => {
                        if prop_name == "length" {
                            Ok(Value::Number(arr.borrow().len() as f64))
                        } else if let Ok(idx) = prop_name.parse::<usize>() {
                            Ok(arr.borrow().get(idx).cloned().unwrap_or(Value::Undefined))
                        } else {
                            Ok(Value::Undefined)
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
                        if let Some(Value::String(constructor_name)) = obj.borrow().get("__constructor__") {
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
