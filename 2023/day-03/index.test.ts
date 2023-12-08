import { beforeAll, describe, expect, test } from 'bun:test';
import { pt1, pt2 } from './index';

const input = `467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..`;

describe('day-03', () => {
  beforeAll(() => {
    console.clear();
  });

  test('part 1', () => {
    const assert = pt1(input);
    const expected = 4361;

    expect(assert.sum).toBe(expected);
  });

  test('part 2', () => {
    const assert = pt1(input);
    const expected = 467835;

    expect(assert.gearRatioSum).toBe(expected);
  });

  test('solution', async () => {
    const inputFile = Bun.file('day-03/input');
    const input = await inputFile.text();

    expect(pt1(input)).toEqual({
      sum: 519444,
      gearRatioSum: 74528807,
    });
  });
});
