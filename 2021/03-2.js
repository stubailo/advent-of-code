const fs = require('fs');
const { exit } = require('process');

const inputFile = fs.readFileSync('./03.input.txt', { encoding: 'utf-8'});
const rows = inputFile.split(/\r?\n/);
rows.pop();

function filterToMostOrLeastCommon(rows, lookingForMost = true) {
  // convert rows into digits ('0', '1')
  let candidates = [...rows].map(row => [...row]);
  // which digit are we looking at?
  for (let i = 0; i < rows[0].length; i++) {
    // find the most common
    let sum = 0;
    candidates.forEach(row => {
      if (row[i] === '1') {
        sum++;
      }
    });

    // need to handle equally common case
    let mostCommon = sum > candidates.length / 2 ? '1' : '0';
    let lookingFor = mostCommon;
    if (lookingForMost === false) {
      // flip it
      lookingFor = mostCommon === '1' ? '0' : '1';
    }

    if (sum === candidates.length / 2) {
      lookingFor = lookingForMost ? '1' : '0';
    }

    console.log({candidates, lookingFor, i, sum});

    const newCandidates = candidates.filter(c => c[i] === lookingFor);
    if (newCandidates.length === 0) {
      return candidates[candidates.length - 1].join('');
    }
    candidates = newCandidates;
  }

  return candidates[0].join('');
}


const oxygen = filterToMostOrLeastCommon(rows);
const co2 = filterToMostOrLeastCommon(rows, false);

console.log(parseInt(oxygen, 2) * parseInt(co2, 2));
