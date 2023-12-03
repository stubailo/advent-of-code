import fs from "fs";
import _ from "lodash";

const inputFile = fs.readFileSync("./2023-3-input.txt", { encoding: "utf-8" });
const rows = inputFile
  .split(/\r?\n/)
  // filter empty rows
  .filter((row) => row.trim());

const spacesNextToPartNumbers = new Map<string, boolean>();

rows.forEach((row, rowIndex) => {
  row.split("").forEach((char, charIndex) => {
    // if char is not . and not a digit
    if (char !== "." && !/\d/.test(char)) {
      // mark all spaces within 1
      for (let i = -1; i <= 1; i++) {
        for (let j = -1; j <= 1; j++) {
          spacesNextToPartNumbers.set(
            [rowIndex + i, charIndex + j].join(","),
            true
          );
        }
      }
    }
  });
});

const numbers: number[] = [];
let sum = 0;
rows.forEach((row, rowIndex) => {
  let currNumber: string[] = [];
  let currNumberIsAdjacent = false;
  row.split("").forEach((char, charIndex) => {
    if (/\d/.test(char)) {
      if (spacesNextToPartNumbers.get([rowIndex, charIndex].join(","))) {
        currNumberIsAdjacent = true;
      }

      currNumber.push(char);
    } else {
      if (currNumber.length > 0 && currNumberIsAdjacent) {
        const num = parseInt(currNumber.join(""));
        sum += num;
        console.log(rowIndex, num, sum);
      }

      currNumber = [];
      currNumberIsAdjacent = false;
    }
  });
});

// sum
console.log(sum);
