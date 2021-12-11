const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./11.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

let flashes = 0;
const grid = rows.map((r) => [...r].map((char) => parseInt(char, 10)));

for (let step = 0; step < 500; step++) {
  grid.forEach((row, i) => {
    row.forEach((val, j) => {
      grid[i][j] = val + 1;
    });
  });

  while (anyFlashesLeft()) {
    grid.forEach((row, i) => {
      row.forEach((val, j) => {
        if (val > 9) {
          flash(i, j);
        }
      });
    });
  }

  let numFlashes = 0;
  grid.forEach((row, i) => {
    row.forEach((val, j) => {
      if (val === -Infinity) {
        grid[i][j] = 0;
        numFlashes++;
      }
    });
  });

  if (numFlashes === 100) {
    console.log("all of them flashed on step", step + 1);
    break;
  }
}

console.log(flashes);

console.timeEnd("logic");

function flash(i, j) {
  grid[i][j] = -Infinity;
  flashes++;

  neighbors(i, j).forEach(([i2, j2]) => {
    if (inside(i2, j2)) {
      grid[i2][j2] = grid[i2][j2] + 1;
    }
  });
}

function inside(i, j) {
  if (i < 0 || j < 0) {
    return false;
  }

  if (i > grid.length - 1 || j > grid.length - 1) {
    return false;
  }

  return true;
}

function neighbors(i, j) {
  return [
    [i - 1, j],
    [i + 1, j],
    [i, j - 1],
    [i, j + 1],
    [i - 1, j - 1],
    [i + 1, j + 1],
    [i + 1, j - 1],
    [i - 1, j + 1],
  ];
}

function printGrid() {
  console.log(
    grid
      .map((row) =>
        row
          .map((item) =>
            item === -Infinity ? "[x]" : (item === 10 ? "" : " ") + `${item} `
          )
          .join("")
      )
      .join("\n")
  );
  console.log("------");
}

function anyFlashesLeft() {
  let result = false;
  grid.forEach((row, i) => {
    row.forEach((val, j) => {
      if (val > 9) {
        result = true;
      }
    });
  });
  return result;
}
