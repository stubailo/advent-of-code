console.time('program');

const fs = require('fs');

const inputFile = fs.readFileSync('./02.input.txt', { encoding: 'utf-8'});
const rows = inputFile.split(/\r?\n/);
rows.pop();

let x = 0;
let depth = 0;
let aim = 0;

rows.forEach(row => {
  const [direction, distanceString] = row.split(' ');
  const distance = parseInt(distanceString, 10);
  if (direction === 'forward') {
    x += distance;
    depth += distance * aim;
  } else if (direction === 'down') {
    aim += distance;
  } else if (direction === 'up') {
    aim -= distance;
  }
});


console.log(x * depth);

console.timeEnd('program');
