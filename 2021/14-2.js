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

const cache = {};
function countsFor(pair, rounds) {
  if (rounds === 0) {
    return {};
  }

  if (cache[`${pair},${rounds}`]) {
    return cache[`${pair},${rounds}`];
  }

  const mid = rules[pair];

  const result = mergeFreq([
    { [mid]: 1 },
    countsFor(pair[0] + mid, rounds - 1),
    countsFor(mid + pair[1], rounds - 1),
  ]);
  cache[`${pair},${rounds}`] = result;
  return result;
}

function freq(str) {
  const counts = {};
  [...str].forEach((c) => {
    if (counts[c]) {
      counts[c] += 1;
    } else {
      counts[c] = 1;
    }
  });
  return counts;
}

function mergeFreq(arrOfFreq) {
  const merged = {};
  arrOfFreq.forEach((freq) => {
    Object.entries(freq).forEach(([key, val]) => {
      if (merged[key]) {
        merged[key] += val;
      } else {
        merged[key] = val;
      }
    });
  });
  return merged;
}

const pairCounts = [];
for (let c = 0; c < str.length - 1; c++) {
  const pair = str[c] + str[c + 1];
  pairCounts.push(countsFor(pair, 40));
}
const counts = mergeFreq([...pairCounts, freq(str)]);

const vals = Object.values(counts);
vals.sort((a, b) => b - a);

console.log(vals[0] - vals[vals.length - 1]);

console.timeEnd("logic");
