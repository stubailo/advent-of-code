const fs = require('fs');

const inputFile = fs.readFileSync('./01.input.txt', { encoding: 'utf-8'});
const rows = inputFile.split(/\r?\n/).map(row => parseInt(row, 10)).filter(n => !isNaN(n));

let numIncreased = 0;
let prev = null;
let window = [];

function sum(arr) {
  let total = 0;
  arr.forEach((item) => {
    total += item;
  });
  return total;
}

rows.forEach((num, i) => {
  if (window.length === 3) {
    window.shift();
  }

  if (window.length < 3) {
    window.push(num);
  }

  if (window.length === 3) {
    let num = sum(window);

    if (prev !== null && num > prev) {
      numIncreased++;
    }

    prev = num;
  }
});

console.log(numIncreased);
