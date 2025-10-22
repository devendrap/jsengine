// Test262 conformance test runner for JSEngine
use jsengine::test262::Test262Runner;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let test262_path = if args.len() > 1 {
        &args[1]
    } else {
        "test262"
    };

    let pattern = if args.len() > 2 {
        &args[2]
    } else {
        "language/expressions"
    };

    let max_tests = if args.len() > 3 {
        args[3].parse::<usize>().ok()
    } else {
        Some(100) // Default: run 100 tests
    };

    println!("JSEngine Test262 Conformance Runner");
    println!("====================================\n");
    println!("Test262 path: {}", test262_path);
    println!("Test pattern: {}", pattern);
    if let Some(max) = max_tests {
        println!("Max tests:    {}", max);
    } else {
        println!("Max tests:    unlimited");
    }
    println!();

    let runner = match Test262Runner::new(test262_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to initialize Test262 runner: {}", e);
            eprintln!("\nMake sure test262 is cloned in the current directory:");
            eprintln!("  git clone --depth 1 https://github.com/tc39/test262.git");
            std::process::exit(1);
        }
    };

    let results = runner.run_test_suite(pattern, max_tests);
    results.print_summary();

    // Exit with error code if tests failed
    if results.failed > 0 {
        std::process::exit(1);
    }
}
