// Test all quick-win features: instanceof, bitwise compound, logical assignment, for...in

console.log("=== instanceof operator ===");
function Animal(name) {
    this.name = name;
}

function Dog(name, breed) {
    this.name = name;
    this.breed = breed;
}

let animal = new Animal("Generic");
let dog = new Dog("Buddy", "Golden Retriever");

console.log("animal instanceof Animal:", animal instanceof Animal);
console.log("dog instanceof Dog:", dog instanceof Dog);

console.log("\n=== 'in' operator ===");
let person = { name: "Alice", age: 30, city: "NYC" };
console.log("'name' in person:", "name" in person);
console.log("'age' in person:", "age" in person);
console.log("'country' in person:", "country" in person);

let arr = [10, 20, 30];
console.log("0 in arr:", 0 in arr);
console.log("2 in arr:", 2 in arr);
console.log("5 in arr:", 5 in arr);

console.log("\n=== Bitwise Compound Assignments ===");
let flags = 5; // Binary: 101
console.log("flags =", flags);

flags &= 3; // AND with 011 = 001
console.log("flags &= 3:", flags);

flags |= 6; // OR with 110 = 111
console.log("flags |= 6:", flags);

flags ^= 2; // XOR with 010 = 101
console.log("flags ^= 2:", flags);

let val = 8;
val <<= 2; // Shift left 2 positions
console.log("8 <<= 2:", val);

val >>= 1; // Shift right 1 position
console.log("32 >>= 1:", val);

console.log("\n=== Logical Assignment Operators ===");

// &&= (assign if truthy)
let x = 5;
x &&= 10;
console.log("x = 5; x &&= 10:", x); // 10

let y = 0;
y &&= 10;
console.log("y = 0; y &&= 10:", y); // 0

// ||= (assign if falsy)
let a = null;
a ||= 42;
console.log("a = null; a ||= 42:", a); // 42

let b = 100;
b ||= 42;
console.log("b = 100; b ||= 42:", b); // 100

console.log("\n=== for...in loop (Objects) ===");
let car = { make: "Toyota", model: "Camry", year: 2020 };
console.log("Car properties:");
for (key in car) {
    console.log("  " + key + ":", car[key]);
}

console.log("\n=== for...in loop (Arrays) ===");
let fruits = ["apple", "banana", "cherry"];
console.log("Fruits:");
for (index in fruits) {
    console.log("  [" + index + "]:", fruits[index]);
}

console.log("\n=== Combined Example ===");
let score = 100;
let bonus = 0;

// Bitwise operations
score &= 127; // Cap at 127
console.log("Capped score:", score);

// Logical assignment
bonus ||= 10; // Set default bonus
console.log("Bonus:", bonus);

// Count properties
let stats = { wins: 5, losses: 3, draws: 2 };
let count = 0;
for (prop in stats) {
    count++;
}
console.log("Total stat categories:", count);

console.log("\n=== All quick-win features work! ===");
