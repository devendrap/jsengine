// Function declarations and calls
function add(a, b) {
    return a + b;
}

function factorial(n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

console.log("add(5, 3) =", add(5, 3));
console.log("factorial(5) =", factorial(5));

// Arrow functions
let multiply = (a, b) => a * b;
console.log("multiply(4, 7) =", multiply(4, 7));

// Closures
function makeCounter() {
    let count = 0;
    return function() {
        count = count + 1;
        return count;
    };
}

let counter = makeCounter();
console.log("counter() =", counter());
console.log("counter() =", counter());
console.log("counter() =", counter());
