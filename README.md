# JSEngine - JavaScript Engine in Rust

A cross-platform JavaScript engine implementation written in Rust, serving as a proof-of-concept replacement for V8. This engine features a complete lexer, parser, and tree-walking interpreter with support for core JavaScript ES6+ features.

## Features

### Language Support

**Core ES5+ Features:**
- Variables: `var`, `let`, `const`
- Primitive types: numbers, strings, booleans, null, undefined
- Objects and arrays (literal syntax)
- Property assignment: `obj.prop = value`, `obj[key] = value`
- Functions (declarations and expressions)
- Control flow: `if/else`, `while`, `for`, `for...in`
- `break` and `continue` statements
- Exception handling: `throw`, `try/catch/finally`
- Object-oriented: `this` keyword, `new` operator
- Comments (single-line `//` and multi-line `/* */`)

**Modern ES6+ Features:**
- Arrow functions: `(x, y) => x + y`
- Template strings (basic concatenation)
- Closures with lexical scoping
- First-class functions

**Operators:**
- Arithmetic: `+`, `-`, `*`, `/`, `%`, `**`
- Comparison: `<`, `>`, `<=`, `>=`, `==`, `===`, `!=`, `!==`
- Logical: `&&`, `||`, `!`
- Bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`, `>>>`
- Assignment: `=`, `+=`, `-=`, `*=`, `/=`, `%=`
- Bitwise Assignment: `&=`, `|=`, `^=`, `<<=`, `>>=`, `>>>=`
- Logical Assignment: `&&=`, `||=` (with short-circuit evaluation)
- Increment/Decrement: `++`, `--` (prefix and postfix)
- Type Checking: `typeof`, `instanceof`, `in`
- Ternary: `condition ? true_val : false_val`

**Built-in Functions:**
- `console.log()` - Print to stdout
- `parseInt()` - Parse string to integer
- `parseFloat()` - Parse string to float
- `isNaN()` - Check if value is NaN

## Architecture

The engine consists of five main components:

1. **Lexer** (`src/lexer.rs`) - Tokenizes JavaScript source code
2. **Parser** (`src/parser.rs`) - Builds an Abstract Syntax Tree (AST) from tokens
3. **AST** (`src/ast.rs`) - Defines expression and statement node types
4. **Value System** (`src/value.rs`) - Runtime type system and operations
5. **Interpreter** (`src/interpreter.rs`) - Tree-walking interpreter with scope management

## Installation

### Prerequisites

- Rust 1.90+ (latest stable)
- Cargo package manager

### Build

```bash
cargo build --release
```

The compiled binary will be at `target/release/jsengine`.

## Usage

### REPL Mode

Run the engine without arguments to start an interactive REPL:

```bash
./target/release/jsengine
```

```javascript
JSEngine v0.1.0 - JavaScript Engine in Rust
Type JavaScript code and press Enter. Type '.exit' to quit.

> let x = 42
> console.log("The answer is", x)
The answer is 42
> .exit
Goodbye!
```

### Execute Files

Run JavaScript files:

```bash
./target/release/jsengine examples/hello.js
```

## Examples

The `examples/` directory contains sample programs:

- **hello.js** - Basic variables and console output
- **functions.js** - Function declarations, recursion, arrow functions, closures
- **loops.js** - While, for, break, continue, Fibonacci sequence
- **data-structures.js** - Arrays, objects, nested structures, property access
- **operators.js** - All supported operators and type checking
- **new-features.js** - Property assignment, throw/try/catch, this, new operator
- **operators-new.js** - Compound assignments, increment/decrement operators
- **quick-wins.js** - instanceof, in, bitwise/logical assignments, for...in loop
- **comprehensive-demo.js** - Complete feature showcase

### Quick Example

```javascript
// Object-oriented programming with this and new
function Person(name, age) {
    this.name = name;
    this.age = age;
    this.greet = function() {
        console.log("Hi, I'm " + this.name);
    };
}

let person = new Person("Alice", 30);
person.greet(); // Hi, I'm Alice
console.log(person instanceof Person); // true

// Modern operators
let score = 100;
score += 50;  // Compound assignment
score++;      // Increment

// Exception handling
try {
    if (score > 100) {
        throw "Score too high!";
    }
} catch (error) {
    console.log("Error:", error);
}

// Iterate over object properties
let user = { name: "Bob", role: "admin", active: true };
for (key in user) {
    console.log(key + ":", user[key]);
}
```

## Technical Details

### Memory Management

The engine uses Rust's `Rc<RefCell<>>` for reference-counted heap-allocated values (objects, arrays, closures), allowing shared mutable state while maintaining Rust's safety guarantees.

### Scope Chain

Functions capture their lexical environment (closure), and the interpreter maintains a scope chain for variable resolution. Each function call creates a new scope that chains to:
1. Global scope
2. Closure scope (captured environment)
3. Local function scope

### Type System

JavaScript's dynamic typing is implemented through the `Value` enum with automatic type conversions:
- Numbers are stored as `f64`
- Strings as `String`
- Objects as `HashMap<String, Value>`
- Arrays as `Vec<Value>`

## Performance Characteristics

This is a **proof-of-concept** implementation prioritizing:
- ✅ Correctness and clarity
- ✅ Feature completeness for core JavaScript
- ✅ Cross-platform compatibility
- ❌ **NOT optimized for production use**

As a tree-walking interpreter, it's significantly slower than JIT-compiled engines like V8 or SpiderMonkey.

## Limitations

Current limitations (by design for POC):
- No async/await or Promises
- No standard library beyond basic built-ins (no Array/String/Object methods)
- Limited prototype chain support (basic constructor tracking only)
- No regular expressions
- No destructuring assignment
- No spread operator
- No modules/imports
- No eval() or dynamic code generation
- No WebAssembly support (planned for future)
- Compound/logical assignment on member expressions not yet supported (`obj.x += 5`)

## Future Enhancements

Potential areas for expansion:
- **Bytecode compiler** - Replace tree-walking with bytecode VM
- **JIT compilation** - Add basic JIT tier for hot code paths
- **Garbage collection** - Replace reference counting with proper GC
- **WebAssembly** - Add WASM runtime and compilation
- **Standard library** - Implement more built-in objects (Math, Array methods, etc.)
- **Source maps** - Better error reporting with line/column numbers
- **Debugger** - Step-through debugging capabilities

## Development

### Run Tests

```bash
cargo test
```

### Format Code

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

## Cross-Platform Support

Built and tested on:
- macOS (ARM64/Apple Silicon and x86_64)
- Linux (x86_64, ARM64)
- Windows (x86_64)

Rust's portability ensures the engine runs on any platform supported by the Rust compiler.

## License

This is a proof-of-concept educational project. Feel free to use and modify for learning purposes.

## Comparison to V8

| Feature | JSEngine (POC) | V8 |
|---------|---------------|-----|
| Architecture | Tree-walking interpreter | Multi-tier JIT compiler |
| Speed | ~100-1000x slower | Baseline |
| Memory | Reference counting | Generational GC |
| ES6+ Support | Core features | Full spec compliance |
| WebAssembly | No | Yes |
| Production Ready | No | Yes |
| Code Size | ~1000 LOC | ~2M+ LOC |
| Purpose | Educational | Production |

## Test262 Conformance Testing

JSEngine includes integration with **Test262**, the official ECMAScript conformance test suite:

```bash
# Build the test runner
cargo build --release

# Run Test262 conformance tests
./target/release/test262-runner test262 "language/expressions" 100
```

See [TEST262_INTEGRATION.md](TEST262_INTEGRATION.md) for detailed information about:
- Test262 setup and usage
- Current limitations and workarounds
- Conformance goals and roadmap
- Technical implementation details

**Current Status**: Infrastructure complete, ready for feature implementation. Most tests are skipped due to missing language features (`this`, `new`, `throw`, prototypes, etc.).

## Contributing

This is a learning project, but suggestions and improvements are welcome! Focus areas:
- Bug fixes for edge cases
- Additional JavaScript features (prioritize: prototype chain, Array/String methods)
- Performance improvements
- Better error messages with line numbers
- Test262 conformance improvements
- Unit tests for individual modules

## Acknowledgments

Inspired by:
- V8 JavaScript Engine (Google)
- Boa JavaScript Engine (Rust)
- Test262 - Official ECMAScript Conformance Test Suite (TC39)
- Crafting Interpreters (Robert Nystrom)
