const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./09.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const nums = rows.map(row => row.split('').map(char => parseInt(char, 10)));

let score = 0;
for (let i=0; i<rows.length; i++) {
  for (let j=0; j<rows[0].length; j++) {
    const item = get(i, j);
    if (item < get(i - 1, j) && item < get(i, j - 1) && item < get(i + 1, j) && item < get(i, j + 1)) {
      score += item + 1;
    }
  }
}

console.log(score);

console.timeEnd("logic");

function get(i, j) {
  if (i < 0 || j < 0) {
    return Infinity;
  }

  if (i > nums.length - 1 || j > nums.length - 1) {
    return Infinity;
  }

  return nums[i][j];
}
