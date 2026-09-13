// JavaScript: Type Systems demo
// JavaScript is DYNAMICALLY typed (like Python) but WEAKLY typed:
// it silently coerces mismatched types instead of raising an error.

let value = 5;
console.log("value =", value, "| type:", typeof value);

value = "five";
console.log("value =", value, "| type:", typeof value);

value = [1, 2, 3];
console.log("value =", value, "| type:", typeof value);

// Weak typing: "5" + 5 does NOT throw. The + operator coerces 5 to "5"
// and performs string concatenation instead of numeric addition.
const result1 = "5" + 5;
console.log('"5" + 5 =', result1, "| type:", typeof result1);

// More surprising coercions that Python/C++ would reject outright:
console.log('"5" - 1 =', "5" - 1);        // "-" forces numeric coercion -> 4
console.log("[] + [] =", JSON.stringify([] + [])); // "" (empty string)
console.log("[] + {} =", [] + {});        // "[object Object]"
console.log("1 == '1' =", 1 == "1");      // true (loose equality coerces)
console.log("1 === '1' =", 1 === "1");    // false (strict equality does not)
