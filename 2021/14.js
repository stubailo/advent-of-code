const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./14.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

let str = rows[0];
const rules = {};
rows
  .slice(2)
  .map((row) => row.split(" -> "))
  .forEach(([from, to]) => {
    rules[from] = to;
  });

for (let i = 0; i < 10; i++) {
  let newStr = [];

  for (let c = 0; c < str.length - 1; c++) {
    const pair = str[c] + str[c + 1];
    newStr.push(str[c], rules[pair]);
  }

  newStr.push(str[str.length - 1]);

  str = newStr.join("");
}

const counts = {};
[...str].forEach((c) => {
  if (counts[c]) {
    counts[c] += 1;
  } else {
    counts[c] = 1;
  }
});

const vals = Object.values(counts);
vals.sort((a, b) => b - a);

console.log(vals[0] - vals[vals.length - 1]);

console.timeEnd("logic");
