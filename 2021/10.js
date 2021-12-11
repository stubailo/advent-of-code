const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./10.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const lArr = ["[", "(", "<", "{"];
const left = new Set(lArr);
const rArr = ["]", ")", ">", "}"];
const right = new Set(rArr);
const mapRightToLeft = {};
rArr.forEach((r, i) => {
  mapRightToLeft[r] = lArr[i];
});

const scores = {
  ")": 3,
  "]": 57,
  "}": 1197,
  ">": 25137,
};

const completionScores = {
  "(": 1,
  "[": 2,
  "{": 3,
  "<": 4,
};

let score = 0;
const completionScoreList = [];
rows.forEach((row) => {
  const stack = [];
  let corrupt = false;
  [...row].forEach((char) => {
    if (left.has(char)) {
      stack.push(char);
    } else if (right.has(char)) {
      if (mapRightToLeft[char] !== stack.pop()) {
        // found syntax error
        score += scores[char];
        corrupt = true;
      }
    }
  });

  if (corrupt) {
    return;
  }

  let completionScore = 0;
  stack.reverse();

  stack.forEach((char) => {
    completionScore = completionScore * 5 + completionScores[char];
  });
  completionScoreList.push(completionScore);
});

completionScoreList.sort((a, b) => a - b);

const middleScore = completionScoreList[(completionScoreList.length - 1) / 2];

console.log("corruption score", score);
console.log("completion score", middleScore);

console.timeEnd("logic");
