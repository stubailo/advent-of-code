import fs from "fs";
import _ from "lodash";

const inputFile = fs.readFileSync("./2023-1-input.txt", { encoding: "utf-8" });
const rows = inputFile
  .split(/\r?\n/)
  // filter empty rows
  .filter((row) => row.trim())
  // replace the numbers "one", "two", etc with their digit equivalents
  .map((row) => {
    console.log(row);
    const digits = [];
    for (let i = 0; i < row.length; i++) {
      [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
      ].forEach((num, j) => {
        if (row.substring(i, i + num.length) === num) {
          digits.push(j);
        }
      });

      if (/\d/.test(row[i])) {
        digits.push(parseInt(row[i]));
      }
    }

    return digits;
  });

// get first and last character of each row
const nums = rows.map((row) => parseInt(`${row[0]}${row[row.length - 1]}`));

// sum all numbers
const sum = _.sum(nums);

console.log(sum);
