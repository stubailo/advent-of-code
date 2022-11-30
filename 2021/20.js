const { reverse } = require("dns");
const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./20.input.txt", { encoding: "utf-8" });

const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const imageEnhancement =
  "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"; // rows[0];

let initialImage = {};
let imageRows = rows.filter((v, i) => i > 2);

imageRows = `...............
...............
...............
...............
...............
.....#..#......
.....#.........
.....##..#.....
.......#.......
.......###.....
...............
...............
...............
...............
...............
`.split("\n");

imageRows.forEach((row, i) => {
  [...row].forEach((v, j) => {
    if (v === "#") {
      initialImage[[i, j].join(",")] = true;
    }
  });
});

function enhance(image) {
  const pixels = Object.keys(image).map((k) =>
    k.split(",").map((v) => parseInt(v, 10))
  );

  const pixelAndNeighborSet = {};

  // add surrounding pixels to be considered
  [...pixels].forEach((p) => {
    [-1, 0, 1].forEach((i) => {
      [-1, 0, 1].forEach((j) => {
        pixelAndNeighborSet[[i + p[0], j + p[1]].join(",")] = true;
      });
    });
  });

  const pixelsAndNeighbors = Object.keys(pixelAndNeighborSet).map((k) =>
    k.split(",").map((v) => parseInt(v, 10))
  );

  const newImage = {};
  pixelsAndNeighbors.forEach((p) => {
    const neighborhood = [];
    [-1, 0, 1].forEach((i) => {
      [-1, 0, 1].forEach((j) => {
        neighborhood.push(image[[i + p[0], j + p[1]].join(",")] ? "1" : "0");
      });
    });

    const index = parseInt(neighborhood.join(""), 2);

    if (imageEnhancement[index] === "#") {
      newImage[p.join(",")] = true;
    }
  });

  return newImage;
}

logImage(initialImage);

const resultImage = enhance(enhance(initialImage));

logImage(resultImage);

console.log(Object.keys(resultImage).length);

function logImage(image) {
  for (let i = 0; i < 20; i++) {
    const row = [];
    for (let j = 0; j < 20; j++) {
      row.push(image[[i, j].join(",")] ? "#" : ".");
    }
    console.log(row.join(""));
  }
}

console.timeEnd("logic");
