# Test262 Integration for JSEngine

This document describes the Test262 conformance testing integration for JSEngine.

## What is Test262?

**Test262** is the official ECMAScript Conformance Test Suite maintained by TC39 (the JavaScript standards committee). It contains over 50,000 test files that verify JavaScript engine compliance with the ECMAScript specification.

- Repository: https://github.com/tc39/test262
- Covers: ECMA-262 (JavaScript) and ECMA-402 (Internationalization)
- Used by: All major JavaScript engines (V8, SpiderMonkey, JavaScriptCore, ChakraCore)

## Integration Status

### Completed Components

✅ **Test262 Repository Cloned** - 54,067 files including harness and tests
✅ **Test Parser** - YAML frontmatter parser for test metadata
✅ **Simplified Harness** - Custom harness compatible with JSEngine's feature set
✅ **Test Runner** - Automated test execution and reporting
✅ **Feature Filtering** - Skip unsupported features automatically
✅ **Reporting System** - Pass/fail/skip statistics with detailed output

### Architecture

```
src/
├── test262.rs           # Test262 integration module
├── bin/
│   └── test262-runner.rs # Standalone test runner binary
└── ...

test262/                 # Test262 repository (gitignored)
├── test/                # 50,000+ test files
├── harness/             # Official harness files
└── ...
```

## Current Limitations

JSEngine is a proof-of-concept implementation that doesn't yet support all JavaScript features. The Test262 integration currently works around these limitations:

### Missing Language Features

The following features prevent many Test262 tests from running:

1. **`this` keyword** - Object context binding not implemented
2. **`new` operator** - Constructor functions not supported
3. **`throw` statement** - Exception throwing not implemented
4. **Prototype chains** - `Object.prototype`, `Function.prototype` not available
5. **Property assignment** - `obj.prop = value` syntax not parsed
6. **Built-in constructors** - `Boolean()`, `Number()`, `String()`, `Error()`, etc.
7. **Error handling** - `try/catch/finally` not implemented
8. **Classes** - ES6 `class` syntax not supported
9. **Modules** - `import/export` not implemented
10. **Advanced features** - async/await, generators, symbols, proxies, etc.

### Workarounds Implemented

**Custom Harness**: Created a simplified Test262 harness that doesn't require unsupported features:
- Uses plain functions instead of constructors
- Uses object literals instead of `this` binding
- Forces errors via undefined property access instead of `throw`
- Compatible with JSEngine's current parser and interpreter

**Aggressive Filtering**: Tests requiring unsupported features are automatically skipped based on:
- Feature flags in YAML metadata
- Known unsupported patterns (classes, async, RegExp, etc.)
- Special test modes (module, raw, async)

## Usage

### Running Test262 Tests

```bash
# Build the test runner
cargo build --release

# Run tests (defaults: 100 tests from language/expressions)
./target/release/test262-runner

# Run specific test pattern
./target/release/test262-runner test262 "language/expressions/addition" 50

# Run all tests in a category (no limit)
./target/release/test262-runner test262 "language/statements/if"
```

### Command-Line Arguments

```
test262-runner [test262_path] [pattern] [max_tests]

test262_path  : Path to Test262 repository (default: "test262")
pattern       : Test file pattern (default: "language/expressions")
max_tests     : Maximum tests to run (default: 100, omit for unlimited)
```

### Example Output

```
JSEngine Test262 Conformance Runner
====================================

Test262 path: test262
Test pattern: language/expressions/addition
Max tests:    50

Found 48 tests matching pattern 'language/expressions/addition'

SFSFFFFFFFFSFSFFFFSFFFSFFFFSFFSFFFFSFFFFSSFSFFFS

============================================================
Test262 Conformance Test Results
============================================================
Total:   48
Passed:  0 (0.0%)
Failed:  35
Skipped: 13
============================================================
```

## Test Categories

Test262 organizes tests into several categories:

### Language Features (`test/language/`)
- **expressions/** - Operators, literals, function calls
- **statements/** - if/else, loops, declarations
- **types/** - Type conversions, primitives
- **functions/** - Function declarations, calls, scope
- **literals/** - Array, object, string literals

### Built-in Objects (`test/built-ins/`)
- **Object/** - Object constructor and methods
- **Array/** - Array methods
- **String/** - String methods
- **Number/** - Number methods
- **Function/** - Function methods
- **Math/** - Math object
- And many more...

### Annexes (`test/annexB/`)
- Legacy features for compatibility

### Internationalization (`test/intl402/`)
- Locale-specific features

## Next Steps for Full Compliance

To achieve better Test262 conformance, JSEngine needs:

### Phase 1: Core Language Features
1. **Property assignment parsing** - Support `obj.prop = value`
2. **`this` keyword** - Context binding in functions
3. **`new` operator** - Constructor function support
4. **`throw` statement** - Exception throwing
5. **Error handling** - `try/catch/finally` blocks

### Phase 2: Object Model
6. **Prototype chains** - `__proto__`, `Object.prototype`
7. **Constructors** - Built-in object constructors
8. **Property descriptors** - `Object.defineProperty`
9. **Getters/setters** - Property accessors

### Phase 3: Advanced Features
10. **ES6 classes** - `class`, `extends`, `super`
11. **Arrow functions** - Proper lexical `this` binding
12. **Destructuring** - Array and object destructuring
13. **Spread/rest** - `...` operator
14. **Template literals** - Backtick strings with interpolation
15. **Modules** - `import`/`export` support

### Phase 4: Built-in Objects
16. **String methods** - `.slice()`, `.substring()`, `.split()`, etc.
17. **Array methods** - `.map()`, `.filter()`, `.reduce()`, etc.
18. **Number methods** - `.toFixed()`, `.toPrecision()`, etc.
19. **Math object** - Math.floor, Math.ceil, etc.
20. **Date object** - Date handling
21. **RegExp object** - Regular expressions
22. **Error objects** - Error, TypeError, ReferenceError, etc.

### Phase 5: Modern Features
23. **Promises** - Async promise handling
24. **async/await** - Asynchronous functions
25. **Generators** - `function*` and `yield`
26. **Iterators** - `Symbol.iterator`
27. **Map/Set** - Collection objects
28. **WeakMap/WeakSet** - Weak reference collections
29. **Proxy/Reflect** - Meta-programming
30. **Symbols** - Unique identifiers

## Conformance Goals

Current status and realistic goals:

| Category | Total Tests | Currently Passing | Short-term Goal | Long-term Goal |
|----------|------------|-------------------|-----------------|----------------|
| Core ES5 | ~15,000 | ~0 | 500 (3%) | 10,000 (67%) |
| ES6+ | ~25,000 | ~0 | 100 (0.4%) | 5,000 (20%) |
| Built-ins | ~10,000 | ~0 | 50 (0.5%) | 2,000 (20%) |
| **Total** | **~50,000** | **~0** | **650 (1.3%)** | **17,000 (34%)** |

*Note: These are estimates based on feature filtering*

## Technical Implementation

### Test File Format

Test262 tests use YAML frontmatter:

```javascript
// Copyright notice
/*---
description: Test description
info: Detailed information
features: [feature1, feature2]
flags: [onlyStrict, module, async]
negative:
  phase: parse|runtime
  type: SyntaxError|ReferenceError|etc.
---*/

// Test code
assert(true === true);
```

### Simplified Harness

Our custom harness (`src/test262.rs`) provides:

```javascript
// Test262Error - Error constructor
function Test262Error(message) { ... }

// assert - Basic assertion
function assert(mustBeTrue, message) { ... }

// assert.sameValue - Equality test
assert.sameValue(actual, expected, message)

// assert.notSameValue - Inequality test
assert.notSameValue(actual, unexpected, message)

// $DONOTEVALUATE - Marker for invalid code
function $DONOTEVALUATE() { ... }
```

### Test Execution Flow

1. **Discovery** - Find all .js files matching pattern
2. **Parse Metadata** - Extract YAML frontmatter
3. **Filter** - Skip tests with unsupported features
4. **Inject Harness** - Prepend simplified harness code
5. **Parse** - Build AST from combined code
6. **Execute** - Run through interpreter
7. **Report** - Collect pass/fail/skip results

### Result Classification

- **Passed** - Test completed without errors, or negative test failed as expected
- **Failed** - Test threw error (positive test) or passed (negative test)
- **Skipped** - Test requires unsupported features

## Contributing

To improve Test262 conformance:

1. **Fix Parser Issues** - Enable property assignment, this, new, throw
2. **Add Missing Features** - Implement prototypes, constructors, try/catch
3. **Expand Built-ins** - Add String/Array/Number/Math methods
4. **Run More Tests** - Test with different test patterns
5. **Report Issues** - Document failures and edge cases

## References

- [Test262 Repository](https://github.com/tc39/test262)
- [Test262 Contributing Guide](https://github.com/tc39/test262/blob/main/CONTRIBUTING.md)
- [ECMAScript Specification](https://tc39.es/ecma262/)
- [Test262 Report (Daily Results)](https://test262.report/)

## Conclusion

The Test262 integration provides a **professional framework for validating JSEngine's correctness** as it evolves. While current pass rates are low due to missing language features, the infrastructure is in place to:

- Track conformance progress over time
- Identify bugs and edge cases
- Guide feature prioritization
- Validate correctness against the official spec

As JSEngine gains more features (especially `this`, `new`, `throw`, and prototypes), Test262 pass rates will increase significantly, providing concrete metrics for measuring progress toward full ECMAScript compliance.
