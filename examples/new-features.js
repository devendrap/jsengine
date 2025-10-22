// Test new features: property assignment, throw/try/catch, this, and new

console.log("=== Testing Property Assignment ===");
let obj = { x: 10, y: 20 };
console.log("Before:", obj);
obj.x = 100;
obj.z = 30;
console.log("After:", obj);

let arr = [1, 2, 3];
console.log("Array before:", arr);
arr[1] = 99;
console.log("Array after:", arr);

console.log("\n=== Testing throw and try/catch ===");
function divide(a, b) {
    if (b === 0) {
        throw "Division by zero!";
    }
    return a / b;
}

try {
    console.log("10 / 2 =", divide(10, 2));
    console.log("10 / 0 =", divide(10, 0));
    console.log("This won't print");
} catch (error) {
    console.log("Caught error:", error);
}

console.log("Continuing after error...");

console.log("\n=== Testing 'this' keyword ===");
let person = {
    name: "Alice",
    age: 30,
    greet: function() {
        console.log("Hello, my name is", this.name);
        console.log("I am", this.age, "years old");
    }
};

person.greet();

console.log("\n=== Testing 'new' operator ===");
function Person(name, age) {
    this.name = name;
    this.age = age;
    this.introduce = function() {
        console.log("Hi, I'm", this.name, "and I'm", this.age);
    };
}

let bob = new Person("Bob", 25);
console.log("bob.name:", bob.name);
console.log("bob.age:", bob.age);
bob.introduce();

let carol = new Person("Carol", 28);
carol.introduce();

console.log("\n=== Testing try/finally ===");
function testFinally() {
    try {
        console.log("In try block");
        return "try return";
    } finally {
        console.log("Finally block always runs");
    }
}

let result = testFinally();
console.log("Result:", result);

console.log("\n=== All tests completed! ===");
