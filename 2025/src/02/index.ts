import { assert, readInputFile } from "../utils";

type Range = [number, number];
const input = await readInputFile(import.meta.dirname);

// comma separated values that represent a range of [FIRST]-[LAST] ids
const testInput = `11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124`;

function p1(input?: string) {
  const ranges = parse(input ?? testInput);
  let sum = 0;

  for (const [lower, upper] of ranges) {
    for (let i = lower; i <= upper; i++) {
      const rangeArray = `${i}`.split("");
      const head = rangeArray.splice(0, rangeArray.length / 2).join("");
      const tail = rangeArray.join("");

      if (head === tail) {
        sum += i;
      }
    }
  }

  return sum;
}

function p2(input?: string) {
  const ranges = parse(input ?? testInput);
  let sum = 0;
  for (const [lower, upper] of ranges) {
    for (let i = lower; i <= upper; i++) {
      const asString = i.toString();
      const sliced = (asString + asString).slice(1, -1);

      if (sliced.includes(asString)) sum += i;
    }
  }

  return sum;
}

if (assert(p1(testInput), 1227775554)) {
  console.log("✅ Part 1 tests pass");
} else {
  throw "ope";
}

console.log("✨ Part 1 answer:", p1(input), "\n");

if (assert(p2(), 4174379265)) {
  console.log("✅ Part 2 tests pass");
} else {
  throw "ope";
}
console.log("Part 2 answer:", p2(input), "\n");

function parse(input: string): Range[] {
  return input
    .replace(/\n/g, "")
    .split(",")
    .map((r) => {
      const sp = r.split("-");

      return sp.map((i) => parseInt(i, 10)) as [number, number];
    });
}
