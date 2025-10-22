// While loop
console.log("While loop:");
let i = 0;
while (i < 5) {
    console.log("  i =", i);
    i = i + 1;
}

// For loop
console.log("For loop:");
for (let j = 0; j < 5; j = j + 1) {
    console.log("  j =", j);
}

// Fibonacci sequence
console.log("Fibonacci sequence:");
let a = 0;
let b = 1;
for (let k = 0; k < 10; k = k + 1) {
    console.log("  fib(" + k + ") =", a);
    let temp = a + b;
    a = b;
    b = temp;
}

// Break and continue
console.log("Break example:");
for (let n = 0; n < 10; n = n + 1) {
    if (n === 5) {
        break;
    }
    console.log("  n =", n);
}

console.log("Continue example:");
for (let m = 0; m < 5; m = m + 1) {
    if (m === 2) {
        continue;
    }
    console.log("  m =", m);
}
