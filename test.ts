const weights = [7, 3, 1, 9, 7, 3, 1, 7, 3];

function from_range(min: number, max: number) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

let number = Array(5).fill(0).map(() => from_range(0, 9));
console.log("number", number);

let both = [
    from_range(10, 13),
    from_range(10, 35),
    from_range(10, 35),
    ...number,
];

let zip = both.map((v, i) => [v, weights[i]]);
console.log("zip", JSON.stringify(zip));

let cost = zip.map(([v, w]) => v * w).reduce((a, b) => a + b);
console.log("cost", cost);

let missing_find = find_missing(cost);
let missing_math = math_missing(cost);

console.log("missing", missing_find);
console.log("missing_math", missing_math);

function find_missing(cost: number) {
    let missing = 0;
    while ((cost + (missing * 3)) % 10 != 0) {
        missing++;
    }
    return missing;
}

// function math_missing(cost: number) {
//     let remainder = cost % 10;
//     let missing = (10 - (cost + (3 - remainder))) / 3;
//     return missing;
// }

let after = [...zip, [missing_find, 3]];
console.log("zip", JSON.stringify(after));

let cost_after = after.map(([v, w]) => v * w).reduce((a, b) => a + b);
console.log("cost_after", cost_after);

function math_missing(cost: number) {
    return ((cost % 10) * 3) % 10;
}

console.log("cost", cost);
console.log("needed ", missing_find);
console.log("missing", missing_math);