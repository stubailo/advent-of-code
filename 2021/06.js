const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./06.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);

let fish = rows[0].split(',').map(f => parseInt(f, 10));

console.time("logic");


console.log(fish);

for (let i=0; i<80; i++) {
  const newFish = [];

  fish = fish.map((f) => {
    if (f === 0) {
      newFish.push(8);
      return 6;
    }

    return f-1;
  }).concat(newFish);
}

console.log(fish.length);

console.timeEnd("logic");
