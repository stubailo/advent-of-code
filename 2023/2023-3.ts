import fs from "fs";
import _ from "lodash";

const inputFile = fs.readFileSync("./2023-3-input.txt", { encoding: "utf-8" });
const rows = inputFile
  .split(/\r?\n/)
  // filter empty rows
  .filter((row) => row.trim());

const gearLocations = new Map<string, boolean>();

rows.forEach((row, rowIndex) => {
  row.split("").forEach((char, charIndex) => {
    // if char is not . and not a digit
    if (char !== "." && !/\d/.test(char)) {
      // mark all spaces within 1

      gearLocations.set([rowIndex, charIndex].join(","), true);
    }
  });
});

const gearLocationToProduct = new Map<string, number>();
const gearLocationToCount = new Map<string, number>();

const numbers: number[] = [];
let sum = 0;
rows.forEach((row, rowIndex) => {
  let currNumber: string[] = [];
  let gearLocation: string | null = null;

  function save() {
    if (currNumber.length > 0 && gearLocation) {
      const num = parseInt(currNumber.join(""));

      gearLocationToCount.set(
        gearLocation,
        (gearLocationToCount.get(gearLocation) || 0) + 1
      );

      if (gearLocationToProduct.has(gearLocation)) {
        gearLocationToProduct.set(
          gearLocation,
          gearLocationToProduct.get(gearLocation)! * num
        );
      } else {
        gearLocationToProduct.set(gearLocation, num);
      }
    }

    currNumber = [];
  }

  row.split("").forEach((char, charIndex) => {
    if (/\d/.test(char)) {
      for (let i = -1; i <= 1; i++) {
        for (let j = -1; j <= 1; j++) {
          if (gearLocations.get([rowIndex + i, charIndex + j].join(","))) {
            gearLocation = [rowIndex + i, charIndex + j].join(",");
          }
        }
      }

      currNumber.push(char);
    } else {
      save();
    }
  });

  save();
});

// sum of all values of gearLocationToProduct, where gearLocationToCount === 2
let answer = 0;
gearLocationToProduct.forEach((product, gearLocation) => {
  if (gearLocationToCount.get(gearLocation) === 2) {
    console.log(gearLocation, product);
    answer += product;
  }
});

console.log(answer);
