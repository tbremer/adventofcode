// const pipe = (...fns) => a => fns.reduce((val, fn) => fn(val), a);
const splitAt = (split: string | RegExp, str: string) => str.split(split);
const mapToInt = (arr: string[]) => arr.map((i) => Number(i));
const testInput = `1721
979
366
299
675
1456`;
const [fileName] = Deno.args;
const testLines = mapToInt(splitAt("\n", testInput)).filter((i) => i !== 0);
const lines = mapToInt(splitAt("\n", await Deno.readTextFile(fileName))).filter(
  (i) => i !== 0,
);

console.log(testLines);

function pt1(list: number[]) {
  for (let idx = 0; idx < list.length; idx++) {
    const el0 = list[idx];
    for (let idx2 = idx + 1; idx2 < list.length; idx2++) {
      const el1 = list[idx2];
      if (el0 + el1 === 2020) return console.log(el0 * el1);
    }
  }

  throw new Error("Part 1 oops!");
}

function pt2(list: number[]) {
  for (let idx0 = 0; idx0 < list.length; idx0++) {
    const el0 = list[idx0];

    inner:
    for (let idx1 = 0; idx1 < list.length; idx1++) {
      const el1 = list[idx1];

      if (el0 + el1 >= 2020) continue inner;

      inner2:
      for (let idx2 = 0; idx2 < list.length; idx2++) {
        const el2 = list[idx2];
        const maths = el0 + el1 + el2;

        if (maths !== 2020) continue inner2;

        return console.log(el0 * el1 * el2);
      }
    }
  }
  throw new Error("Part 2 oops!");
}

function main() {
  pt1(testLines);
  pt1(lines);

  pt2(testLines);
  pt2(lines);
}

main();
