const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./08.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const parsed = rows.map((r) => {
  const [input, output] = r.split(" | ");
  return {
    input: input.split(" "),
    output: output.split(" "),
  };
});

// correct locations
const numbers = [
  "abcefg",
  "cf",
  "acdeg",
  "acdfg",
  "bcdf",
  "abdfg",
  "abdefg",
  "acf",
  "abcdefg",
  "abcdfg",
];

const freq = getFrequencies(numbers);
const reverse = {};
numbers.forEach((str, i) => reverse[str] = i);

let sum = 0;
parsed.forEach(({ input, output }) => {
  input.sort((l, r) => l.length - r.length);

  const rowFreq = getFrequencies(input);

  const one = [...input[0]];
  const seven = [...input[1]];
  const four = [...input[2]];

  const bd = [...difference(new Set(four), new Set(one)).values()];
  const eg = [
    ...difference(
      difference(new Set([..."abcdefg"]), new Set(seven)),
      new Set(four)
    ),
  ];

  const reverseMapping = {
    a: difference(new Set(seven), new Set(one)).values().next().value,
    c: pickBasedOnFrequency(rowFreq, "c", one),
    f: pickBasedOnFrequency(rowFreq, "f", one),
    b: pickBasedOnFrequency(rowFreq, "b", bd),
    d: pickBasedOnFrequency(rowFreq, "d", bd),
    e: pickBasedOnFrequency(rowFreq, "e", eg),
    g: pickBasedOnFrequency(rowFreq, "g", eg),
  };

  const mapping = Object.fromEntries(Object.entries(reverseMapping).map(([k, v]) => [v, k]));

  const outDigits = output.map((str) => {
    const chars = [...str].map(c => mapping[c]);
    chars.sort();
    return reverse[chars.join('')];
  })

  const result = parseInt(outDigits.join(''), 10);
  sum += result;
});

console.log(sum);

console.timeEnd("logic");

function difference(setA, setB) {
  let _difference = new Set(setA);
  for (let elem of setB) {
    _difference.delete(elem);
  }
  return _difference;
}

function pickBasedOnFrequency(rowFreq, letter, candidates) {
  for (c of candidates) {
    if (rowFreq[c] === freq[letter]) {
      return c;
    }
  }

  throw new Error("didn't find candidate");
}

function getFrequencies(listOfStrings) {
  const freq = {};
  [...numbers[8]].forEach((letter) => (freq[letter] = 0));
  listOfStrings.forEach((num) =>
    [...num].forEach((letter) => (freq[letter] += 1))
  );
  return freq;
}
