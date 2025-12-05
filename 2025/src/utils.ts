import { deepEqual } from "node:assert";
import { readFile as nodeReadFile } from "node:fs/promises";
import { resolve } from "node:path";

export function has<K extends PropertyKey>(
  o: object,
  str: K,
): o is Record<K, unknown> {
  return Reflect.has(o, str);
}

export function assert(actual: unknown, expected: unknown) {
  try {
    deepEqual(actual, expected);
    return true;
  } catch (e) {
    if (
      typeof e === "object" &&
      e !== null &&
      has(e, "actual") &&
      has(e, "expected")
    ) {
      const { actual, expected } = e;
      console.warn("⚠️", { actual, expected });
    } else {
      console.warn("⚠️", e);
    }
    return false;
  }
}

export async function readInputFile(directory: string) {
  const filePath = resolve(directory, "input.txt");
  return nodeReadFile(filePath, "utf-8");
}
