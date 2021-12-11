const fs = require('fs');

const inputFile = fs.readFileSync('./01.input.txt', { encoding: 'utf-8'});
const rows = inputFile.split(/\r?\n/).map(row => parseInt(row, 10)).filter(n => !isNaN(n));

let numIncreased = 0;
let prev = 0;

rows.forEach((num, i) => {
  if (i === 0) {
    prev = num;
    return;
  }

  if (num > prev) {
    numIncreased++;
  }

  prev = num;
});

console.log(numIncreased);
