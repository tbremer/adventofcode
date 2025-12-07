import { readInputFile, runTestInput } from "../utils";

const input = await readInputFile(import.meta.dirname);

const testInput = `987654321111111
811111111111119
234234234234278
818181911112111`;

function solve(input?: string, length: 2 | 12 = 2) {
  const rows = parse(input ?? testInput);
  let sum = 0;

  for (const battery of rows) {
    while (battery.length > length) {
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

runTestInput(() => solve(testInput, 2), 357);
console.log("✨ Part 1 answer:", solve(input, 2));

runTestInput(() => solve(testInput, 12), 3121910778619);
console.log("✨ Part 1 answer:", solve(input, 12));

function parse(input: string) {
  return input
    .split(/\r?\n/)
    .map((b) => b.split("").map((i) => Number.parseInt(i, 10)));
}
