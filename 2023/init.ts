#!/usr/bin/env bun

const [cliWhich] = Bun.argv.slice(2);
let which: string | number | undefined = cliWhich;

const date = new Date();
const day = which ? Number(which) : date.getDate() + 1;

const dayString = day < 10 ? `0${day}` : day;
const dayPath = `./day-${dayString}`;

Bun.spawnSync(['mkdir', dayPath]);

await Promise.all([
  // empty input file
  Bun.write(dayPath + '/input', ''),

  // define index scaffold
  Bun.write(
    dayPath + '/index.ts',
    `import { asLines } from '../utils/string';

export function pt1(input: string | string[]): null {
  return null;
}

export function pt2(input: string): null {
  return null;
}
`,
  ),

  // define test scaffold
  Bun.write(
    dayPath + '/index.test.ts',
    `import { describe, expect, test } from 'bun:test';
import { pt1, pt2 } from './index';

describe('day-01', () => {
  test('part 1', () => {
    const input = \`\`;
    const expected = null;

    expect(pt1(input)).toBe(expected);
  });

  test('part 2', () => {
    const input = \`\`;
    const expected = null;

    expect(pt2(input)).toBe(expected);
  });
});
`,
  ),
]);
