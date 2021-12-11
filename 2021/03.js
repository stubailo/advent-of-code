const fs = require('fs');

const inputFile = fs.readFileSync('./03.input.txt', { encoding: 'utf-8'});
const rows = inputFile.split(/\r?\n/);
rows.pop();

const sums = [...rows[0]].map(x => 0);

rows.forEach(row => {
  [...row].forEach((digit, i) => {
    if (digit === '1') {
      sums[i] += 1;
    }
  });
});

let gamma = [];
let epsilon = [];

sums.forEach(d => {
  if (d > rows.length / 2) {
    gamma.push('1');
    epsilon.push('0');
  } else {
    gamma.push('0');
    epsilon.push('1');
  }
})

const gammaNum = parseInt(gamma.join(''), 2);
const epsilonNum = parseInt(epsilon.join(''), 2);

console.log(gammaNum * epsilonNum);
