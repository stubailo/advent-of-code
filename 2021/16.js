const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./16.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const binStr = [...rows[0]]
  .map((c) => {
    if (c === "0") return "0000";
    if (c === "1") return "0001";
    if (c === "2") return "0010";
    if (c === "3") return "0011";
    if (c === "4") return "0100";
    if (c === "5") return "0101";
    if (c === "6") return "0110";
    if (c === "7") return "0111";
    if (c === "8") return "1000";
    if (c === "9") return "1001";
    if (c === "A") return "1010";
    if (c === "B") return "1011";
    if (c === "C") return "1100";
    if (c === "D") return "1101";
    if (c === "E") return "1110";
    if (c === "F") return "1111";
  })
  .join("");

let counter = 0;
let sumOfVersionNumbers = 0;

function readVal() {
  const versionNum = parseInt(getBits(3), 2);
  const packetTypeId = parseInt(getBits(3), 2);

  sumOfVersionNumbers += versionNum;

  if (packetTypeId === 4) {
    let moreBits = "1";
    let numberBitsArr = [];

    while (moreBits === "1") {
      const [thereMoreBits, ...numberBits] = [...getBits(5)];
      numberBitsArr.push(numberBits.join(""));
      moreBits = thereMoreBits;
    }

    const val = parseInt(numberBitsArr.join(""), 2);

    console.log({ val });
    return val;
  }

  // it's an operator
  const lengthTypeId = getBits(1);
  const vals = [];

  if (lengthTypeId === "0") {
    const lengthOfSubpackets = parseInt(getBits(15), 2);

    let end = counter + lengthOfSubpackets;
    while (counter < end) {
      vals.push(readVal());
    }
  } else {
    const numberOfSubpackets = parseInt(getBits(11), 2);

    while (vals.length < numberOfSubpackets) {
      vals.push(readVal());
    }
  }

  let result = null;
  if (packetTypeId === 0) {
    let sum = 0;
    vals.forEach((v) => (sum += v));
    result = sum;
  } else if (packetTypeId === 1) {
    let product = 1;
    vals.forEach((v) => (product *= v));
    result = product;
  } else if (packetTypeId === 2) {
    result = Math.min(...vals);
  } else if (packetTypeId === 3) {
    result = Math.max(...vals);
  } else if (packetTypeId === 5) {
    result = +(vals[0] > vals[1]);
  } else if (packetTypeId === 6) {
    result = +(vals[0] < vals[1]);
  } else if (packetTypeId === 7) {
    result = +(vals[0] === vals[1]);
  }

  console.log({ packetTypeId, vals, result });
  return result;
}

function getBits(n) {
  let bits = [];
  while (bits.length < n) {
    bits.push(binStr[counter]);
    counter++;
  }
  return bits.join("");
}

const val = readVal();

console.log(sumOfVersionNumbers, val);

console.timeEnd("logic");
