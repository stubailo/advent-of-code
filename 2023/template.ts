import fs from "fs";

const inputFile = fs.readFileSync("{{INPUTFILE}}", { encoding: "utf-8" });
const rows = inputFile.split(/\r?\n/).map((row) => row);

console.log("hello world", rows.length);
