const fs = require('fs');

const inputFile = fs.readFileSync('./02.input.txt', { encoding: 'utf-8'});
const rows = inputFile.split(/\r?\n/);
rows.pop();

let x = 0;
let depth = 0;

rows.forEach(row => {
  const [direction, distanceString] = row.split(' ');
  const distance = parseInt(distanceString, 10);
  if (direction === 'forward') {
    x += distance;
  } else if (direction === 'down') {
    depth += distance;
  } else if (direction === 'up') {
    depth -= distance;
  }

  console.log({direction, distance, x, depth});
});


console.log(x * depth);
