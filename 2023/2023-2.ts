import fs from "fs";
import _ from "lodash";

const inputFile = fs.readFileSync("./2023-2-input.txt", { encoding: "utf-8" });
const rows = inputFile
  .split(/\r?\n/)
  // filter empty rows
  .filter((row) => row.trim())
  // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
  .map((row) => {
    const [a, b] = row.split(": ");
    const gameNumber = parseInt(a.split(" ")[1]);
    const grabs = b.split("; ").map((grab) => {
      const counts: Record<string, number> = {};

      grab.split(", ").forEach((color) => {
        const [count, colorName] = color.split(" ");
        counts[colorName] = parseInt(count);
      });

      return counts;
    });

    // Get max number of each color
    const max: Record<string, number> = {};
    grabs.forEach((grab) => {
      Object.keys(grab).forEach((color) => {
        max[color] = Math.max(max[color] ?? 0, grab[color]);
      });
    });

    return {
      gameNumber,
      grabs,
      max,
    };
  });

const desiredCounts = {
  red: 12,
  green: 13,
  blue: 14,
};

// For each game, multiply the values of max
let sum = 0;
rows.forEach(({ max }) => {
  const product = Object.keys(max).reduce((acc, color) => {
    return acc * max[color];
  }, 1);
  sum += product;
});

console.log(sum);
