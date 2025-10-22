// Test compound assignment and increment/decrement operators

console.log("=== Compound Assignment Operators ===");
let x = 10;
console.log("x =", x);

x += 5;
console.log("x += 5:", x); // 15

x -= 3;
console.log("x -= 3:", x); // 12

x *= 2;
console.log("x *= 2:", x); // 24

x /= 4;
console.log("x /= 4:", x); // 6

x %= 4;
console.log("x %= 4:", x); // 2

console.log("\n=== Prefix Increment/Decrement ===");
let a = 5;
console.log("a =", a);

let b = ++a;
console.log("b = ++a: a =", a, ", b =", b); // a = 6, b = 6

let c = --a;
console.log("c = --a: a =", a, ", c =", c); // a = 5, c = 5

console.log("\n=== Postfix Increment/Decrement ===");
let d = 10;
console.log("d =", d);

let e = d++;
console.log("e = d++: d =", d, ", e =", e); // d = 11, e = 10

let f = d--;
console.log("f = d--: d =", d, ", f =", f); // d = 10, f = 11

console.log("\n=== In Loop ===");
for (let i = 0; i < 5; i++) {
    console.log("  i =", i);
}

console.log("\n=== Countdown ===");
let count = 5;
while (count > 0) {
    console.log("  count:", count);
    count--;
}
console.log("  Blast off!");

console.log("\n=== Compound in Expressions ===");
let sum = 0;
sum += 1;
sum += 2;
sum += 3;
sum += 4;
sum += 5;
console.log("sum = 1+2+3+4+5 =", sum);

console.log("\n=== Complex Example ===");
let score = 100;
console.log("Initial score:", score);
score -= 10; // Penalty
console.log("After penalty:", score);
score *= 1.5; // Bonus multiplier
console.log("After bonus:", score);
let bonus = score++;
console.log("Bonus saved:", bonus, ", Score now:", score);

console.log("\n=== All tests completed! ===");
