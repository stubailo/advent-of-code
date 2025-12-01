// paste the results into https://strudel.cc/ to run!

setcpm(40);

const results = await solve();
const shortResults = results.slice(0, 50);

const getNote = (pos) => note("c2".add(ceil(pos / 2))).label("" + pos);
const getBassNote = (pos) =>
  note("c2".add(ceil((pos / 2) % 12))).label("" + pos);
const getPadNote = (pos) =>
  note("c3".add(ceil((pos / 2) % 12))).label("" + pos);

const resultNotes = cat(...results.map(getNote)).fast(4);
const bassNotes = cat(...results.map(getBassNote)).fast(4);
const padNotes = cat(...results.map(getPadNote)).fast(4);

piano: resultNotes.sound("piano").delay(0.2).pianoroll({ labels: 1 });
bass: bassNotes.sound("gm_synth_bass_2");
drums: sound("bd hh [bd,sd] hh").velocity(0.5).room(0.4).delay(0.1);
pad: padNotes.sound("gm_pad_bowed").mask("0 1").room(0.5);

async function solve() {
  const positions = [];

  const input = await fetch(
    "https://stubailo.github.io/advent-of-code/2025/01.input.txt"
  ).then((r) => r.text());
  const lines = input.split("\n");

  let position = 50;
  positions.push(position);

  let zeroCount = 0;
  for (let line of lines) {
    const dir = line[0] === "R" ? 1 : -1;
    const num = parseInt(line.slice(1));
    let diff = dir * num;

    while (diff < -100) {
      diff += 100;
      zeroCount++;
    }

    while (diff > 100) {
      diff -= 100;
      zeroCount++;
    }

    let newPos = position + diff;
    if (newPos < 0) {
      newPos += 100;
      if (position !== 0) {
        zeroCount++;
      }
    } else if (newPos >= 100) {
      newPos -= 100;
      zeroCount++;
    } else if (newPos === 0) {
      zeroCount++;
    }

    position = newPos;
    positions.push(position);
  }

  logger("zeroCount " + zeroCount);

  return positions;
}

silence;
