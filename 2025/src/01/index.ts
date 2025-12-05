import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { assert } from "../utils";

// L rotates towards lower numbers
// R rotates towards higher numbers
// Dial starts at 50
type Rotation = `L${number}` | `R${number}`;

// Password is the number of times the dial is pointing at `0` after any rotation
const testInput: string = `
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
`;

function p1(input: string = testInput) {
  const rotations = input.split(/\n/g).filter((s) => isRotation(s));
  const parsedRotations = rotations.map((r) => {
    const [dir, ...num] = r;
    return (dir === "L" ? -1 : 1) * Number.parseInt(num.join(""), 10);
  });

  return (
    parsedRotations
      .reduce<[number, number]>(
        ([dial, count], rotation) => {
          let nextPos = dial + rotation;
          const offset = nextPos >= 100 ? -100 : 100;
          while (nextPos < 0 || nextPos >= 100) {
            nextPos += offset;
          }

          return [nextPos, count + (nextPos === 0 ? 1 : 0)];
        },
        [50, 0],
      )
      .at(1) ?? 0
  );
}

function p2(input: string = testInput) {
  const rotations = input.split(/\n/g).filter((s) => isRotation(s));
  const parsedRotations = rotations.map((r) => {
    const [dir, ...num] = r;

    return (dir === "L" ? -1 : 1) * Number.parseInt(num.join(""), 10);
  });

  let dial = 50;

  return parsedRotations.reduce((count, rotation) => {
    const maths = rotation < 0 ? 1 : -1;
    while (rotation !== 0) {
      rotation += maths;
      dial += maths;

      if (dial === 100 || dial === -100) {
        dial = 0;
      }

      if (dial === 0) count += 1;
    }

    return count;
  }, 0);
}

function isRotation(str: unknown): str is Rotation {
  if (typeof str !== "string") return false;

  return /^(?:L|R)\d{1,}$/.test(str);
}

if (assert(p1(), 3)) {
  console.log("✅ Part 1 tests pass\n");
} else {
  throw "ope";
}

if (assert(p2(), 6)) {
  console.log("✅ Part 2 tests pass\n");
} else {
  throw "ope";
}

const filePath = resolve(import.meta.dirname, "input.txt");
const input = await readFile(filePath, "utf-8");

console.log("Answer:", p1(input));
console.log("Answer:", p2(input));
