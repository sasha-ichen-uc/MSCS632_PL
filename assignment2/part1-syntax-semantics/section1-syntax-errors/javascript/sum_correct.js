// JavaScript: Calculate the sum of an array
function calculateSum(arr) {
    let total = 0;
    for (const num of arr) {
        total += num;
    }
    return total;
}

const numbers = [1, 2, 3, 4, 5];
const result = calculateSum(numbers);
console.log("Sum in JavaScript:", result);
