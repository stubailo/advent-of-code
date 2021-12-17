const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./13.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

let dots = {};
const folds = {x: [], y: []};

rows.forEach(row => {
  if (row.startsWith('fold')) {
    const [letter, num] = row.split(' ')[2].split('=');
    folds[letter].push(num);
  } else if (row === '') {
    return;
  } else {
    dots[row] = true;
  }
});

folds.x.forEach(xVal => {
  const newDots = {};
  Object.keys(dots).forEach(dot => {
    let [x, y] = dot.split(',').map(n => parseInt(n,10));
    if (x > xVal) {
      x = 2 * xVal - x;
    }
    newDots[[x,y].join(',')] = true;
  });

  dots = newDots;
})

folds.y.forEach(yVal => {
  const newDots = {};
  Object.keys(dots).forEach(dot => {
    let [x, y] = dot.split(',').map(n => parseInt(n,10));
    if (y > yVal) {
      y = 2 * yVal - y;
    }
    newDots[[x,y].join(',')] = true;
  });
  dots = newDots;
})

console.log({dots, folds});
console.log(Object.keys(dots).length)

console.timeEnd("logic");