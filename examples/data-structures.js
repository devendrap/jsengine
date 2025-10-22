// Arrays
console.log("Arrays:");
let arr = [1, 2, 3, 4, 5];
console.log("  arr =", arr);
console.log("  arr[0] =", arr[0]);
console.log("  arr[2] =", arr[2]);
console.log("  arr.length =", arr.length);

// Nested arrays
let matrix = [[1, 2], [3, 4], [5, 6]];
console.log("  matrix =", matrix);
console.log("  matrix[1][0] =", matrix[1][0]);

// Objects
console.log("Objects:");
let person = {
    name: "Alice",
    age: 30,
    city: "New York"
};
console.log("  person =", person);
console.log("  person.name =", person.name);
console.log("  person.age =", person.age);

// Object with computed property access
let key = "city";
console.log("  person[key] =", person[key]);

// Nested objects
let company = {
    name: "TechCorp",
    employee: {
        name: "Bob",
        role: "Engineer"
    }
};
console.log("  company.employee.name =", company.employee.name);
