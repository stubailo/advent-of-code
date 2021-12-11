console.time("setup");

console.time("readFile");

const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./04.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.timeEnd("readFile");


const bingoNumbers = rows[0].split(",").map((x) => parseInt(x, 10));

// now we need to ingest the bingo boards

// keep a pointer
let currentLine = 2;
const boards = new Int8Array(rows.length * 5);

let currentIndex = 0;
while (currentLine < rows.length) {
  const splitRow = rows[currentLine].split(/\s+/g);
  currentLine++;

  if (splitRow.length < 5) {
    continue;
  }

  splitRow
    .map((x) => parseInt(x, 10))
    .filter((x) => !isNaN(x))
    .forEach((num) => {
      boards[currentIndex] = num;
      currentIndex++;
    });
}

const numBoards = currentIndex / 25;

console.timeEnd("setup");
console.time("program");


// risky -- we'll just replace the called number with a -1

const MARKED = -1;

let numWins = 0;
const wins = {};

bingoNumbers.forEach((bingoNum) => {
  boards.forEach((item, i) => {
    if (item === bingoNum) {
      boards[i] = MARKED;
    }
  });

  for (let boardIndex = 0; boardIndex < numBoards; boardIndex++) {
    if (!wins[boardIndex]) {
      const offset = boardIndex * 25;

      // detect if win
      let win = false;
      [0, 1, 2, 3, 4].forEach((rowNum) => {
        if (
          [0, 1, 2, 3, 4].every(
            (colNum) => boards[offset + rowNum * 5 + colNum] === MARKED
          )
        ) {
          win = true;
        }
      });

      [0, 1, 2, 3, 4].forEach((colNum) => {
        if (
          [0, 1, 2, 3, 4].every(
            (rowNum) => boards[offset + rowNum * 5 + colNum] === MARKED
          )
        ) {
          win = true;
        }
      });

      if (win) {
        wins[boardIndex] = true;
        numWins++;

        if (numWins === numBoards) {
          let sum = 0;

          for (let i = offset; i < offset + 25; i++) {
            if (boards[i] !== MARKED) {
              sum += boards[i];
            }
          }

          console.log(sum * bingoNum);
          console.timeEnd("program");

          exit(0);
        }
      }
    }
  }
});
