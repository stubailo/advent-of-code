console.time("program");

console.time("setup");


const fs = require("fs");
const { exit } = require("process");



const inputFile = fs.readFileSync("./04.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

const bingoNumbers = rows[0].split(",").map((x) => parseInt(x, 10));

// now we need to ingest the bingo boards

// keep a pointer
let currentLine = 2;
const boards = [];

while (currentLine < rows.length) {
  const board = [];
  for (let i = 0; i < 5; i++) {
    board.push(
      ...rows[currentLine]
        .split(/\s+/g)
        .map((x) => parseInt(x, 10))
        .filter((x) => !isNaN(x))
    );
    currentLine++;
  }
  boards.push(board);

  // skip one empty line
  currentLine++;
}

console.timeEnd("setup");


// risky -- we'll just replace the called number with an x

const MARKED = -1;

let numWins = 0;
const wins = {};

bingoNumbers.forEach((bingoNum) => {
  boards.forEach((board, boardIndex) => {
    if (!wins[boardIndex]) {
      // add mark
      board.forEach((item, i) => {
        if (item === bingoNum) {
          board[i] = MARKED;
        }
      });

      // detect if win
      let win = false;
      [0, 1, 2, 3, 4].forEach((rowNum) => {
        if (
          [0, 1, 2, 3, 4].every(
            (colNum) => board[rowNum * 5 + colNum] === MARKED
          )
        ) {
          win = true;
        }
      });

      [0, 1, 2, 3, 4].forEach((colNum) => {
        if (
          [0, 1, 2, 3, 4].every(
            (rowNum) => board[rowNum * 5 + colNum] === MARKED
          )
        ) {
          win = true;
        }
      });

      if (win) {
        wins[boardIndex] = true;
        numWins++;

        if (numWins === boards.length) {
          let sum = 0;
          board.forEach((item) => {
            if (item !== MARKED) {
              sum += item;
            }
          });

          console.log(sum * bingoNum);
          console.timeEnd("program");

          exit(0);
        }
      }
    }
  });
});
