const { reverse } = require("dns");
const fs = require("fs");
const { exit } = require("process");

const inputFile = fs.readFileSync("./19.input.txt", { encoding: "utf-8" });

const rows = inputFile.split(/\r?\n/);
rows.pop();

console.time("logic");

const scanners = [];

let currentScanner = -1;
rows.forEach((r) => {
  if (r.startsWith("---")) {
    currentScanner++;
    scanners.push([]);
  } else if (r === "") {
    // do nothing
  } else {
    point = r.split(",").map((i) => parseInt(i, 10));
    scanners[currentScanner].push(point);
  }
});

// x, y, z
const allRotations = [];

for (let xIndex = 0; xIndex < 6; xIndex++) {
  for (let yIndex = 0; yIndex < 4; yIndex++) {
    const xRow = [0, 0, 0];
    xRow[xIndex % 3] = Math.floor(xIndex / 3) === 1 ? -1 : 1;

    const availableYPositions = [0, 1, 2].filter((p) => p !== xIndex % 3);

    const yRow = [0, 0, 0];
    yRow[availableYPositions[yIndex % 2]] =
      Math.floor(yIndex / 2) === 1 ? -1 : 1;

    const zRow = [
      xRow[1] * yRow[2] - xRow[2] * yRow[1],
      xRow[2] * yRow[0] - xRow[0] * yRow[2],
      xRow[0] * yRow[1] - xRow[1] * yRow[0],
    ];

    allRotations.push([xRow, yRow, zRow]);
  }
}

function detectOverlap(scanner1, scanner2) {
  let result = null;
  // rotate scanner2
  const found = allRotations.some((rotation) => {
    const scanner2Rotation = scanner2.map((vec) => {
      return mul(rotation, vec);
    });

    return scanner1.some((startingPoint1) => {
      const scanner1Normalized = scanner1.map((p) => {
        return p.map((val, i) => val - startingPoint1[i]);
      });
      return scanner2Rotation.some((startingPoint2) => {
        scanner2RotationNormalized = scanner2Rotation.map((p) => {
          return p.map((val, i) => val - startingPoint2[i]);
        });

        let overlapCount = 0;

        // ok now we compare
        const scanner1Set = {};
        scanner1Normalized.forEach((p) => {
          scanner1Set[p.join(",")] = true;
        });

        scanner2RotationNormalized.forEach((p) => {
          if (scanner1Set[p.join(",")]) {
            overlapCount++;
          }
        });

        if (overlapCount >= 12) {
          result = {
            overlapCount,
            rotation,
            startingPoint1,
            startingPoint2,
          };
          return true;
        }
      });
    });
  });

  if (found) {
    return result;
  } else {
    return null;
  }
}

function mul(mat, vec) {
  return [dot(mat[0], vec), dot(mat[1], vec), dot(mat[2], vec)];
}

function dot(l, r) {
  return l[0] * r[0] + l[1] * r[1] + l[2] * r[2];
}

function sub(l, r) {
  return [l[0] - r[0], l[1] - r[1], l[2] - r[2]];
}

function add(l, r) {
  return [l[0] + r[0], l[1] + r[1], l[2] + r[2]];
}

// to detect overlap -- test 8 different rotations, one of each n as a starting point, each pair 25*25

const located = { 0: true };
let next = [scanners[0]];

const setOfPoints = {};

scanners[0].forEach((p) => (setOfPoints[p.join(",")] = true));

while (next.length) {
  const originScanner = next.pop();
  for (let i = 0; i < scanners.length; i++) {
    if (!located[i]) {
      const s2 = scanners[i];
      const result = detectOverlap(originScanner, s2);

      if (result) {
        console.log("detected overlap!", i);

        const relativeLocation = sub(
          result.startingPoint1,
          result.startingPoint2
        );

        // convert points
        const remappedPoints = s2
          .map((p) => mul(result.rotation, p))
          .map((p) => add(p, relativeLocation));

        remappedPoints.forEach((p) => (setOfPoints[p.join(",")] = true));

        located[i] = true;
        next.push(remappedPoints);
      }
    }
  }
}

console.log(Object.keys(setOfPoints).length);

console.timeEnd("logic");
