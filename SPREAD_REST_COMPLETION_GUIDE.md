# Spread/Rest Operators - Completion Guide

## Current Status: ~70% Complete ✅

The parser is **fully complete** and working. Only the interpreter needs updates to handle the new AST types.

## What's Done ✅

1. **AST** (`src/ast.rs`)
   - Added `ArrayElement` enum (Expression | Spread)
   - Added `ObjectProperty` enum (Property | Spread)
   - Added `FunctionParam` enum (Normal | Rest)

2. **Lexer** (`src/lexer.rs`)
   - Added `Ellipsis` token for `...`
   - Tokenization working correctly

3. **Parser** (`src/parser.rs`)
   - Array spread: `[1, 2, ...arr, 3]` ✅
   - Object spread: `{a: 1, ...obj, b: 2}` ✅
   - Rest parameters: `function(...args)` ✅
   - All function declarations/expressions updated

## What's Remaining: 5 Interpreter Fixes

Run `cargo build` to see the exact errors. There are 5 type mismatches in `src/interpreter.rs`:

### Fix 1: Array Evaluation (~line 1808)

**Current code:**
```rust
Expr::Array(elements) => {
    let values: Result<Vec<Value>, String> =
        elements.iter().map(|e| self.eval_expression(e)).collect();
    Ok(Value::Array(Rc::new(RefCell::new(values?))))
}
```

**Fixed code:**
```rust
Expr::Array(elements) => {
    let mut values = Vec::new();
    for element in elements {
        match element {
            ArrayElement::Expression(expr) => {
                values.push(self.eval_expression(expr)?);
            }
            ArrayElement::Spread(expr) => {
                // Evaluate the spread expression
                let spread_val = self.eval_expression(expr)?;
                match spread_val {
                    Value::Array(arr) => {
                        // Spread the array elements
                        values.extend(arr.borrow().clone());
                    }
                    _ => return Err("Spread operator requires an array".to_string()),
                }
            }
        }
    }
    Ok(Value::Array(Rc::new(RefCell::new(values))))
}
```

### Fix 2: Object Evaluation (~line 1814)

**Current code:**
```rust
Expr::Object(properties) => {
    let mut map = HashMap::new();
    for (key, value_expr) in properties {
        let value = self.eval_expression(value_expr)?;
        map.insert(key.clone(), value);
    }
    Ok(Value::Object(Rc::new(RefCell::new(map))))
}
```

**Fixed code:**
```rust
Expr::Object(properties) => {
    let mut map = HashMap::new();
    for property in properties {
        match property {
            ObjectProperty::Property { key, value } => {
                let val = self.eval_expression(value)?;
                map.insert(key.clone(), val);
            }
            ObjectProperty::Spread(expr) => {
                // Evaluate the spread expression
                let spread_val = self.eval_expression(expr)?;
                match spread_val {
                    Value::Object(obj) => {
                        // Spread the object properties
                        for (k, v) in obj.borrow().iter() {
                            map.insert(k.clone(), v.clone());
                        }
                    }
                    _ => return Err("Object spread requires an object".to_string()),
                }
            }
        }
    }
    Ok(Value::Object(Rc::new(RefCell::new(map))))
}
```

### Fix 3: Function Expression (~line 1823)

**Current code:**
```rust
Expr::Function { params, body } => {
    Ok(Value::Function(Function {
        params: params.clone(),
        body: body.clone(),
        closure: self.current_scope(),
    }))
}
```

**Fixed code:**
```rust
Expr::Function { params, body } => {
    Ok(Value::Function(Function {
        params: params.clone(),
        body: body.clone(),
        closure: self.current_scope(),
    }))
}
```
*Note: This stays the same - the Function struct already uses Vec<FunctionParam>*

### Fix 4: Arrow Function (~line 1835)

**Current code:**
```rust
Expr::ArrowFunction { params, body } => {
    Ok(Value::Function(Function {
        params: params.clone(),
        body: vec![Stmt::Return(Some(*body.clone()))],
        closure: self.current_scope(),
    }))
}
```

**Fixed code:**
```rust
Expr::ArrowFunction { params, body } => {
    Ok(Value::Function(Function {
        params: params.clone(),
        body: vec![Stmt::Return(Some(*body.clone()))],
        closure: self.current_scope(),
    }))
}
```
*Note: This also stays the same*

### Fix 5: Function Call with Rest Parameters (~line 683)

Find the `call_function` method in the interpreter. You need to update how function parameters are bound:

**Current code (approximately):**
```rust
fn call_function(&mut self, func: &Function, args: &[Value], this: Value) -> Result<Value, String> {
    self.push_scope();

    // Bind parameters
    for (i, param) in func.params.iter().enumerate() {
        let value = args.get(i).cloned().unwrap_or(Value::Undefined);
        self.set_variable(param, value);
    }

    // ... rest of function
}
```

**Fixed code:**
```rust
fn call_function(&mut self, func: &Function, args: &[Value], this: Value) -> Result<Value, String> {
    self.push_scope();

    // Bind 'this' if provided
    self.set_variable("this", this);

    // Bind parameters
    let mut arg_index = 0;
    for param in &func.params {
        match param {
            FunctionParam::Normal(name) => {
                let value = args.get(arg_index).cloned().unwrap_or(Value::Undefined);
                self.set_variable(name, value);
                arg_index += 1;
            }
            FunctionParam::Rest(name) => {
                // Collect remaining arguments into an array
                let rest_args: Vec<Value> = args[arg_index..].to_vec();
                self.set_variable(name, Value::Array(Rc::new(RefCell::new(rest_args))));
                break; // Rest parameter must be last
            }
        }
    }

    // Execute function body
    let result = self.execute_block(&func.body);
    self.pop_scope();

    match result {
        Ok(flow) => match flow {
            ControlFlow::Return(val) => Ok(val),
            _ => Ok(Value::Undefined),
        },
        Err(e) => Err(e),
    }
}
```

## Testing After Fixes

Create a test file `examples/spread-rest-test.js`:

```javascript
// Array spread
let arr1 = [1, 2, 3];
let arr2 = [4, 5, 6];
let combined = [...arr1, ...arr2];
console.log("Array spread:", combined); // [1, 2, 3, 4, 5, 6]

let mixed = [0, ...arr1, 3.5, ...arr2, 7];
console.log("Mixed spread:", mixed); // [0, 1, 2, 3, 3.5, 4, 5, 6, 7]

// Object spread
let obj1 = { a: 1, b: 2 };
let obj2 = { c: 3, d: 4 };
let merged = { ...obj1, ...obj2 };
console.log("Object spread:", merged); // { a: 1, b: 2, c: 3, d: 4 }

let overwrite = { x: 1, ...{ x: 2, y: 3 } };
console.log("Overwrite:", overwrite); // { x: 2, y: 3 }

// Rest parameters
function sum(...numbers) {
    let total = 0;
    for (let i = 0; i < numbers.length; i++) {
        total += numbers[i];
    }
    return total;
}

console.log("sum(1, 2, 3):", sum(1, 2, 3)); // 6
console.log("sum(1, 2, 3, 4, 5):", sum(1, 2, 3, 4, 5)); // 15

function greet(greeting, ...names) {
    for (let i = 0; i < names.length; i++) {
        console.log(greeting + ", " + names[i]);
    }
}

greet("Hello", "Alice", "Bob", "Charlie");
// Hello, Alice
// Hello, Bob
// Hello, Charlie

// Combined example
function logAll(...args) {
    console.log("Arguments:", args);
}

let data = [1, 2, 3];
logAll(...data); // Should work once spread in calls is added
```

## Build and Test

```bash
# Fix the 5 errors
cargo build --release

# Test
./target/release/jsengine examples/spread-rest-test.js
```

## Additional Feature: Spread in Function Calls (Optional)

The current implementation supports spread in arrays/objects and rest parameters.

To also support spread in function calls (e.g., `func(...args)`), you need to update the `Call` expression evaluation in the interpreter to handle spread arguments.

This is optional and can be added later if desired.

## Completion Checklist

- [ ] Fix 1: Array spread evaluation
- [ ] Fix 2: Object spread evaluation
- [ ] Fix 3: Function expression (verify no change needed)
- [ ] Fix 4: Arrow function (verify no change needed)
- [ ] Fix 5: Rest parameter binding in call_function
- [ ] Test with examples/spread-rest-test.js
- [ ] Update README with new features
- [ ] Create PR and merge

## Expected Results

After all fixes:
- ✅ Arrays can be spread: `[...arr1, ...arr2]`
- ✅ Objects can be spread: `{...obj1, ...obj2}`
- ✅ Functions can use rest parameters: `function(...args)`
- ✅ Rest parameters collect remaining arguments into an array

Total effort: ~30-45 minutes to complete all fixes and testing.

Good luck! 🚀
