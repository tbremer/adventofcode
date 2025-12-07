import { readInputFile, runTestInput } from "../utils";

const input = await readInputFile(import.meta.dirname);

const testInput = `987654321111111
811111111111119
234234234234278
818181911112111`;

function pt1(input?: string) {
  const rows = parse(input ?? testInput);
  let sum = 0;

  for (const battery of rows) {
    const max = Math.max(...battery.slice(0, -1));
    const maxIdx = battery.indexOf(max);

    if (maxIdx < 0) {
      throw new Error(
        `Could not find index for \`${maxIdx}\` in battery \`${battery.join("")}\``,
      );
    }
    const remaining = battery.slice(maxIdx + 1);
    const nextMax = Math.max(...remaining);

    sum += Number.parseInt(max.toString() + nextMax.toString(), 10);
  }

  return sum;
}

function pt2(input?: string) {
  const rows = parse(input ?? testInput);
  let sum = 0;

  for (const battery of rows) {
    while (battery.length > 12) {
      for (let idx = 0; idx < battery.length; idx++) {
        // If we're at the end of the battery, the final index is the lowest value
        if (idx === battery.length - 1) battery.splice(idx, 1);

        // remove value at current index if it's lower than next index
        if ((battery[idx] ?? 0) < (battery[idx + 1] ?? 0)) {
          battery.splice(idx, 1);
          break;
        }
      }
    }

    sum += Number.parseInt(battery.join(""), 10);
  }

  return sum;
}

runTestInput(() => pt1(testInput), 357);
console.log("✨ Part 1 answer:", pt1(input));

runTestInput(() => pt2(testInput), 3121910778619);
console.log("✨ Part 2 answer:", pt2(input));

function parse(input: string) {
  return input
    .split(/\r?\n/)
    .map((b) => b.split("").map((i) => Number.parseInt(i, 10)));
}
