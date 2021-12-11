const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./06.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);

let fish = rows[0].split(",").map((f) => parseInt(f, 10));

console.time("logic");

let numFishEachAge = [0, 0, 0, 0, 0, 0, 0, 0, 0];

fish.forEach((age) => {
  numFishEachAge[age] += 1;
});

for (let i = 0; i < 256; i++) {
  const newNumFishEachAge = [0, 0, 0, 0, 0, 0, 0, 0, 0];

  numFishEachAge.forEach((num, age) => {
    if (age === 0) {
      newNumFishEachAge[8] += num;
      newNumFishEachAge[6] += num;
    } else {
      newNumFishEachAge[age - 1] += num;
    }
  });

  numFishEachAge = newNumFishEachAge;
}

let s = 0;
numFishEachAge.forEach((n) => {
  s += n;
});

console.log(s);

console.timeEnd("logic");
