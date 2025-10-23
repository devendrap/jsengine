// Built-in Methods and Objects Demo
// Demonstrates Array methods, String methods, Math object, and compound member assignments

console.log("=== Array Methods ===");

// Basic array manipulation
let arr = [1, 2, 3];
console.log("Initial array:", arr);

arr.push(4);
console.log("After push(4):", arr);

let popped = arr.pop();
console.log("Popped value:", popped);
console.log("After pop():", arr);

arr.unshift(0);
console.log("After unshift(0):", arr);

let shifted = arr.shift();
console.log("Shifted value:", shifted);
console.log("After shift():", arr);

// Array methods with compound assignments
let data = { count: 0 };
data.count += 5;
console.log("Compound assignment on object:", data.count);

arr[0] += 10;
console.log("Compound assignment on array:", arr);

// Array iteration methods
console.log("\n=== Array Iteration Methods ===");

let numbers = [1, 2, 3, 4, 5];
console.log("Original:", numbers);

// map
let doubled = numbers.map(function(x) { return x * 2; });
console.log("Doubled (map):", doubled);

// filter
let evens = numbers.filter(function(x) { return x % 2 === 0; });
console.log("Evens (filter):", evens);

// reduce
let sum = numbers.reduce(function(acc, x) { return acc + x; }, 0);
console.log("Sum (reduce):", sum);

// Arrow functions work too
let tripled = numbers.map((x) => x * 3);
console.log("Tripled (arrow):", tripled);

// Array utility methods
console.log("\n=== Array Utility Methods ===");

let items = [10, 20, 30, 40, 50];
console.log("Items:", items);

let sliced = items.slice(1, 3);
console.log("Sliced [1:3]:", sliced);

console.log("indexOf(30):", items.indexOf(30));
console.log("indexOf(99):", items.indexOf(99));

let spliced = items.splice(2, 1, 100, 200);
console.log("Spliced (removed):", spliced);
console.log("After splice:", items);

console.log("\n=== String Methods ===");

let text = "Hello World";
console.log("Original:", text);
console.log("Length:", text.length);
console.log("toUpperCase:", text.toUpperCase());
console.log("toLowerCase:", text.toLowerCase());

let parts = text.split(" ");
console.log("Split by space:", parts);

let chars = "abc".split("");
console.log("Split to chars:", chars);

console.log("substring(0, 5):", text.substring(0, 5));
console.log("slice(6):", text.slice(6));
console.log("slice(-5):", text.slice(-5));
console.log("indexOf('World'):", text.indexOf("World"));
console.log("indexOf('xyz'):", text.indexOf("xyz"));

console.log("\n=== Math Object ===");

console.log("Math.PI:", Math.PI);
console.log("Math.E:", Math.E);

console.log("Math.floor(4.7):", Math.floor(4.7));
console.log("Math.ceil(4.2):", Math.ceil(4.2));
console.log("Math.round(4.5):", Math.round(4.5));
console.log("Math.abs(-42):", Math.abs(-42));
console.log("Math.sqrt(16):", Math.sqrt(16));
console.log("Math.pow(2, 8):", Math.pow(2, 8));
console.log("Math.min(5, 2, 8, 1):", Math.min(5, 2, 8, 1));
console.log("Math.max(5, 2, 8, 1):", Math.max(5, 2, 8, 1));

let random = Math.random();
console.log("Math.random():", random);
console.log("Random 1-10:", Math.floor(random * 10) + 1);

console.log("\n=== Compound Member Assignments ===");

let obj = { x: 10, y: 20 };
console.log("Initial object:", obj);

obj.x += 5;
console.log("After obj.x += 5:", obj);

obj.y *= 2;
console.log("After obj.y *= 2:", obj);

obj["x"] -= 3;
console.log("After obj['x'] -= 3:", obj);

let matrix = [[1, 2], [3, 4]];
console.log("Matrix:", matrix);
matrix[0][1] += 10;
console.log("After matrix[0][1] += 10:", matrix);

console.log("\n=== Logical Member Assignments ===");

let config = { enabled: true, count: 0 };
config.enabled &&= true;
console.log("After enabled &&= true:", config);

config.count ||= 10;
console.log("After count ||= 10:", config);

console.log("\n=== Combined Example ===");

// Calculate average of filtered numbers
let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let evenValues = values.filter((n) => n % 2 === 0);
let total = evenValues.reduce((sum, n) => sum + n, 0);
let average = total / evenValues.length;

console.log("Values:", values);
console.log("Even values:", evenValues);
console.log("Average of evens:", average);

// String processing
let message = "JavaScript is awesome";
let words = message.split(" ");
let upperWords = words.map((w) => w.toUpperCase());
let result = upperWords.reduce((acc, w) => acc + " " + w, "");

console.log("Original:", message);
console.log("Transformed:", result);

console.log("\nAll built-in methods working!");
