const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./08.input.txt", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const parsed = rows.map(r => {
  const [input, output] = r.split(' | ');
  return {
    input: input.split(' '),
    output: output.split(' '),
  };
});

let num = 0;
parsed.forEach(({output}) => {
  output.forEach(o => {
    if (o.length === 2 || o.length === 3 || o.length === 4 || o.length === 7) {
      num++;
    }
  })
})

console.log(num);

console.timeEnd("logic");
