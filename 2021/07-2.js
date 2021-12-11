const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./07.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

let max = 0;

const locations = rows[0].split(",").map((f) => {
  const l = parseInt(f, 10);
  if (l > max) {
    max = l;
  }
  return l;
});

let smallestFuel = Infinity;
for (let i = 0; i < max; i++) {
  let sum = 0;

  locations.forEach((l) => {
    const d = Math.abs(i - l);
    f = ((1 + d) * d) / 2;
    sum += f;
  });

  if (sum < smallestFuel) {
    smallestFuel = sum;
  }
}

console.log(smallestFuel);

console.timeEnd("logic");
