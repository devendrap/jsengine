// Comprehensive Demo of JSEngine Features
// =========================================

console.log("=== JSEngine Comprehensive Demo ===\n");

// 1. Variables and Types
console.log("1. Variables and Types:");
let number = 42;
let string = "Hello, JSEngine!";
let boolean = true;
let nullValue = null;
let undefinedValue = undefined;

console.log("  number:", number, "- type:", typeof number);
console.log("  string:", string, "- type:", typeof string);
console.log("  boolean:", boolean, "- type:", typeof boolean);
console.log("  null:", nullValue, "- type:", typeof nullValue);
console.log("  undefined:", undefinedValue, "- type:", typeof undefinedValue);

// 2. Arithmetic Operations
console.log("\n2. Arithmetic Operations:");
let a = 10;
let b = 3;
console.log("  a =", a, ", b =", b);
console.log("  a + b =", a + b);
console.log("  a - b =", a - b);
console.log("  a * b =", a * b);
console.log("  a / b =", a / b);
console.log("  a % b =", a % b);
console.log("  2 ** 10 =", 2 ** 10);

// 3. String Operations
console.log("\n3. String Operations:");
let firstName = "John";
let lastName = "Doe";
let fullName = firstName + " " + lastName;
console.log("  Full name:", fullName);

// 4. Arrays
console.log("\n4. Arrays:");
let numbers = [1, 2, 3, 4, 5];
console.log("  numbers =", numbers);
console.log("  numbers[0] =", numbers[0]);
console.log("  numbers[4] =", numbers[4]);
console.log("  length =", numbers.length);

let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
console.log("  matrix[1][1] =", matrix[1][1]);

// 5. Objects
console.log("\n5. Objects:");
let person = {
    name: "Alice",
    age: 30,
    city: "San Francisco",
    isEmployed: true
};
console.log("  person.name =", person.name);
console.log("  person.age =", person.age);
console.log("  person['city'] =", person["city"]);

// 6. Functions
console.log("\n6. Functions:");

function greet(name) {
    return "Hello, " + name + "!";
}

function add(x, y) {
    return x + y;
}

function multiply(x, y) {
    return x * y;
}

console.log("  greet('World') =", greet("World"));
console.log("  add(15, 27) =", add(15, 27));
console.log("  multiply(6, 7) =", multiply(6, 7));

// 7. Recursion
console.log("\n7. Recursion:");

function fibonacci(n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

console.log("  fibonacci(0) =", fibonacci(0));
console.log("  fibonacci(1) =", fibonacci(1));
console.log("  fibonacci(8) =", fibonacci(8));

// 8. Arrow Functions
console.log("\n8. Arrow Functions:");
let square = (x) => x * x;
let sum = (a, b) => a + b;
console.log("  square(7) =", square(7));
console.log("  sum(100, 200) =", sum(100, 200));

// 9. Closures
console.log("\n9. Closures:");

function createMultiplier(factor) {
    return function(value) {
        return value * factor;
    };
}

let double = createMultiplier(2);
let triple = createMultiplier(3);
console.log("  double(5) =", double(5));
console.log("  triple(5) =", triple(5));

// 10. Higher-Order Functions
console.log("\n10. Higher-Order Functions:");

function applyOperation(x, y, operation) {
    return operation(x, y);
}

let result1 = applyOperation(10, 5, (a, b) => a + b);
let result2 = applyOperation(10, 5, (a, b) => a * b);
console.log("  applyOperation(10, 5, add) =", result1);
console.log("  applyOperation(10, 5, multiply) =", result2);

// 11. Conditionals
console.log("\n11. Conditionals:");

function checkAge(age) {
    if (age < 13) {
        return "child";
    } else if (age < 20) {
        return "teenager";
    } else if (age < 65) {
        return "adult";
    } else {
        return "senior";
    }
}

console.log("  checkAge(10) =", checkAge(10));
console.log("  checkAge(16) =", checkAge(16));
console.log("  checkAge(30) =", checkAge(30));
console.log("  checkAge(70) =", checkAge(70));

// 12. Ternary Operator
console.log("\n12. Ternary Operator:");
let age = 25;
let canVote = age >= 18 ? "Yes" : "No";
console.log("  age =", age, ", can vote?", canVote);

// 13. Loops
console.log("\n13. Loops:");

console.log("  For loop (0 to 4):");
for (let i = 0; i < 5; i = i + 1) {
    console.log("    i =", i);
}

console.log("  While loop (countdown from 3):");
let countdown = 3;
while (countdown > 0) {
    console.log("    countdown =", countdown);
    countdown = countdown - 1;
}

// 14. Logical Operators
console.log("\n14. Logical Operators:");
let x = true;
let y = false;
console.log("  x && y =", x && y);
console.log("  x || y =", x || y);
console.log("  !x =", !x);

// 15. Comparison Operators
console.log("\n15. Comparison Operators:");
console.log("  10 === 10 =", 10 === 10);
console.log("  10 !== 20 =", 10 !== 20);
console.log("  5 < 10 =", 5 < 10);
console.log("  10 >= 10 =", 10 >= 10);

// 16. Complex Example: Prime Number Checker
console.log("\n16. Prime Number Checker:");

function isPrime(num) {
    if (num <= 1) {
        return false;
    }
    if (num === 2) {
        return true;
    }
    let i = 2;
    while (i * i <= num) {
        if (num % i === 0) {
            return false;
        }
        i = i + 1;
    }
    return true;
}

console.log("  isPrime(7) =", isPrime(7));
console.log("  isPrime(12) =", isPrime(12));
console.log("  isPrime(17) =", isPrime(17));

// 17. Nested Data Structures
console.log("\n17. Nested Data Structures:");
let company = {
    name: "TechCorp",
    employees: [
        { name: "Alice", role: "Engineer" },
        { name: "Bob", role: "Designer" },
        { name: "Charlie", role: "Manager" }
    ]
};
console.log("  Company:", company.name);
console.log("  First employee:", company.employees[0].name, "-", company.employees[0].role);

// 18. Bitwise Operations
console.log("\n18. Bitwise Operations:");
console.log("  5 & 3 =", 5 & 3);
console.log("  5 | 3 =", 5 | 3);
console.log("  5 ^ 3 =", 5 ^ 3);
console.log("  8 << 1 =", 8 << 1);
console.log("  8 >> 1 =", 8 >> 1);

console.log("\n=== Demo Complete! ===");
