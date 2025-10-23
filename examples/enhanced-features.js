// Enhanced Features Demo
// Demonstrates Object methods, additional Array/String methods, and JSON support

console.log("=== Object Methods ===");

let person = { name: "Alice", age: 30, city: "NYC" };
console.log("Object:", person);

// Object.keys
let keys = Object.keys(person);
console.log("Object.keys:", keys);

// Object.values
let values = Object.values(person);
console.log("Object.values:", values);

// Object.entries
let entries = Object.entries(person);
console.log("Object.entries:", entries);

// Object.assign
let target = { a: 1 };
let source1 = { b: 2 };
let source2 = { c: 3 };
let merged = Object.assign(target, source1, source2);
console.log("Object.assign:", merged);
console.log("Target modified:", target);

// Object.create
let newObj = Object.create(null);
console.log("Object.create:", newObj);

// hasOwnProperty
console.log("person.hasOwnProperty('name'):", person.hasOwnProperty("name"));
console.log("person.hasOwnProperty('salary'):", person.hasOwnProperty("salary"));

console.log("\n=== Array Methods - join, reverse, concat, includes ===");

let fruits = ["apple", "banana", "cherry"];
console.log("Fruits:", fruits);

// join
let joined = fruits.join(", ");
console.log("Joined:", joined);

let numbers = [1, 2, 3, 4, 5];
console.log("Numbers:", numbers);

// reverse
let reversed = numbers.reverse();
console.log("Reversed:", reversed);
console.log("Original modified:", numbers);

// concat
let moreNumbers = [6, 7, 8];
let combined = numbers.concat(moreNumbers, 9, 10);
console.log("Concat:", combined);

// includes
console.log("Includes 3:", numbers.includes(3));
console.log("Includes 99:", numbers.includes(99));

console.log("\n=== Array Methods - find, findIndex, some, every, forEach ===");

let items = [
    { id: 1, name: "Laptop", price: 1000 },
    { id: 2, name: "Mouse", price: 25 },
    { id: 3, name: "Keyboard", price: 75 }
];

// find
let found = items.find(function(item) { return item.price > 50; });
console.log("Find (price > 50):", found);

// findIndex
let foundIdx = items.findIndex(function(item) { return item.name === "Mouse"; });
console.log("FindIndex (Mouse):", foundIdx);

// some
let hasCheap = items.some(function(item) { return item.price < 30; });
console.log("Some (price < 30):", hasCheap);

// every
let allExpensive = items.every(function(item) { return item.price > 20; });
console.log("Every (price > 20):", allExpensive);

// forEach
console.log("ForEach:");
items.forEach(function(item) {
    console.log("  -", item.name, ":", item.price);
});

console.log("\n=== String Methods - charAt, charCodeAt, trim ===");

let text = "  Hello World  ";
console.log("Original:", text);

// trim
console.log("Trimmed:", text.trim());

let str = "JavaScript";

// charAt
console.log("charAt(0):", str.charAt(0));
console.log("charAt(4):", str.charAt(4));

// charCodeAt
console.log("charCodeAt(0):", str.charCodeAt(0));
console.log("charCodeAt(4):", str.charCodeAt(4));

console.log("\n=== String Methods - replace, startsWith, endsWith, includes ===");

let message = "Hello World";

// replace
console.log("Replace 'World' with 'JavaScript':", message.replace("World", "JavaScript"));

// startsWith
console.log("Starts with 'Hello':", message.startsWith("Hello"));
console.log("Starts with 'World':", message.startsWith("World"));

// endsWith
console.log("Ends with 'World':", message.endsWith("World"));
console.log("Ends with 'Hello':", message.endsWith("Hello"));

// includes
console.log("Includes 'World':", message.includes("World"));
console.log("Includes 'xyz':", message.includes("xyz"));

console.log("\n=== String Methods - repeat, padStart, padEnd ===");

// repeat
console.log("'Ha'.repeat(3):", "Ha".repeat(3));
console.log("'-'.repeat(10):", "-".repeat(10));

// padStart
console.log("'5'.padStart(3, '0'):", "5".padStart(3, "0"));
console.log("'Hello'.padStart(10, '*'):", "Hello".padStart(10, "*"));

// padEnd
console.log("'5'.padEnd(3, '0'):", "5".padEnd(3, "0"));
console.log("'Hello'.padEnd(10, '*'):", "Hello".padEnd(10, "*"));

console.log("\n=== JSON Support ===");

// JSON.stringify
let data = {
    name: "Bob",
    age: 25,
    hobbies: ["reading", "coding"],
    address: {
        city: "SF",
        zip: "94102"
    }
};

console.log("Original object:", data);
let jsonString = JSON.stringify(data);
console.log("JSON.stringify:", jsonString);

// JSON.parse
let parsed = JSON.parse(jsonString);
console.log("JSON.parse:", parsed);

// Test with array
let arr = [1, 2, { x: 10, y: 20 }, "test"];
let arrJson = JSON.stringify(arr);
console.log("Array JSON:", arrJson);
let arrParsed = JSON.parse(arrJson);
console.log("Parsed array:", arrParsed);

// Test with primitives
console.log("JSON.stringify(42):", JSON.stringify(42));
console.log("JSON.stringify(true):", JSON.stringify(true));
console.log("JSON.stringify('hello'):", JSON.stringify("hello"));
console.log("JSON.stringify(null):", JSON.stringify(null));

console.log("\n=== Combined Example ===");

// Process data with new methods
let products = [
    { name: "Laptop", category: "Electronics", price: 999.99 },
    { name: "Shirt", category: "Clothing", price: 29.99 },
    { name: "Book", category: "Books", price: 14.99 },
    { name: "Phone", category: "Electronics", price: 699.99 }
];

console.log("All products:");
products.forEach(function(p) {
    console.log("  " + p.name.padEnd(10) + " - $" + p.price);
});

// Filter electronics
let electronics = products.filter(function(p) { return p.category === "Electronics"; });
console.log("\nElectronics:");
console.log(JSON.stringify(electronics));

// Check if any product is under $20
let hasCheapItem = products.some(function(p) { return p.price < 20; });
console.log("\nHas item under $20:", hasCheapItem);

// Get all product names
let names = products.map(function(p) { return p.name; }).join(", ");
console.log("Product names:", names);

console.log("\nAll enhanced features working!");
