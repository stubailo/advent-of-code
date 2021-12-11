const fs = require('fs');

const rows = readNumbers('./01.input.txt');

let numIncreased = 0;
let prev = null;
let window = [];

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



function readNumbers(filename) {
  const inputFile = fs.readFileSync(filename, { encoding: 'utf-8'});
  const rows = inputFile.split(/\r?\n/);
  const numbers = rows.map(row => parseInt(row, 10)).filter(n => !isNaN(n));
  return numbers;
}

function sum(arr) {
  let total = 0;
  arr.forEach((item) => {
    total += item;
  });
  return total;
}
