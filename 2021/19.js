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
const allRotations = [
  [
    [1, 0, 0],
    [0, 1, 0],
  ],
  [
    [1, 0, 0],
    [0, -1, 0],
  ],
  [
    [1, 0, 0],
    [0, 0, 1],
  ],
  [
    [1, 0, 0],
    [0, 0, -1],
  ],
  [
    [0, 1, 0],
    [1, 0, 0],
  ],
  [
    [0, 1, 0],
    [-1, 0, 0],
  ],
  [
    [0, 1, 0],
    [0, 0, 1],
  ],
  [
    [0, 1, 0],
    [0, 0, -1],
  ],
  [
    [0, 0, 1],
    [1, 0, 0],
  ],
  [
    [0, 0, 1],
    [-1, 0, 0],
  ],
  [
    [0, 0, 1],
    [0, 1, 0],
  ],
  [
    [0, 0, 1],
    [0, -1, 0],
  ],
].map((m) => {
  const a = m[0];
  const b = m[1];
  return [
    a,
    b,
    [
      a[1] * b[2] - a[2] * b[1],
      a[2] * b[0] - a[0] * b[2],
      a[0] * b[1] - a[1] * b[0],
    ],
  ];
});

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

// to detect overlap -- test 8 different rotations, one of each n as a starting point, each pair 25*25

const groupMap = {};
const groups = [];

scanners.forEach((s1, i) => {
  if (i < scanners.length - 1) {
    for (let j = i + 1; j < scanners.length; j++) {
      const s2 = scanners[j];

      if (i !== j) {
        const result = detectOverlap(s1, s2);
        if (result) {
          if (groupMap[i]) {
            groups[groupMap[i]].push(result);
          } else if (groupMap[j]) {
            groups[groupMap[j]].push(result);
          } else {
            groupMap[i] = groups.length;
            groupMap[j] = groups.length;

            groups.push([result]);
          }
        }
      }
    }
  }
});

fs.writeFileSync("19.intermediate.json", JSON.stringify(groups), {
  encoding: "utf-8",
});

console.timeEnd("logic");
