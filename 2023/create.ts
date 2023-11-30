import fs from "fs";
import fetch from "node-fetch";

const cookie =
  "session=53616c7465645f5fe4ca20f9d2b2a42c4650721d84e686ec7553cbea9cbdda6a775bb5a16bf7d6087dce3cd47b25ffa05f567a043f8245c18a35140fa4ed5418";

// Get year and puzzle number from arguments
const [year, day] = process.argv[2].split("-");

// Error if year or day is not provided
if (!year || !day) {
  console.log("Please provide year and day");
  process.exit(1);
}

async function run() {
  // Use fetch to get https://adventofcode.com/2021/day/4/input
  const url = `https://adventofcode.com/${year}/day/${day}/input`;
  const input = await fetch(url, {
    headers: {
      cookie,
    },
  }).then((r) => r.text());

  // Write input to file
  const filename = `./${year}-${day}-input.txt`;

  fs.writeFileSync(filename, input);

  // Load template.ts
  const template = fs.readFileSync("./template.ts", { encoding: "utf-8" });

  // Replace {{INPUTFILE}} with filename
  const code = template.replace("{{INPUTFILE}}", filename);

  // Write code to file
  fs.writeFileSync(`./${year}-${day}.ts`, code);
}

run();
