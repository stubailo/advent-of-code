const fs = require("fs");
const { exit } = require("process");

// const inputFile = fs.readFileSync("./17.input.txt", { encoding: "utf-8" });
// const rows = inputFile.split(/\r?\n/);
// rows.pop();

console.time("logic");

// target area: x=241..275, y=-75..-49

function simulate(vx, vy) {
  // run the simulation. if in target area, return height. if past target area, return -Infinity
  let x = 0;
  let y = 0;
  let maxHeight = -Infinity;
  while (x < 275 && y > -75) {
    x += vx;
    y += vy;
    if (vx !== 0) {
      vx -= Math.sign(vx);
    }
    vy --;

    if (y > maxHeight) {
      maxHeight = y;
    }

    if (x >= 241 && x <= 275 && y >= -75 && y <= -49) {
      return maxHeight;
    }
  }

  return -Infinity;
}

let maxMaxHeight = 0;
let count = 0;
for (let vx = 0; vx <=275; vx++) {
  for (let vy = -75; vy <= 1000; vy++) {
    const result = simulate(vx, vy);
    if (result !== -Infinity) {
      count++;
    }
    if (result > maxMaxHeight) {
      maxMaxHeight = result;
    }
  }
}

console.log(maxMaxHeight, count);

console.timeEnd("logic");
