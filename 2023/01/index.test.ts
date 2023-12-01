import { describe, expect, test } from 'bun:test';
import { pt1, pt2 } from './index';

describe('day-01', () => {
  test('part 1', () => {
    const input = `1abc2
      pqr3stu8vwx
      a1b2c3d4e5f
      treb7uchet`;
    const expected = 142;

    expect(pt1(input)).toBe(expected);
  });

  describe('part 2', () => {
    test('part 2', () => {
      const input = `two1nine
      eightwothree
      abcone2threexyz
      xtwone3four
      4nineeightseven2
      zoneight234
      7pqrstsixteen`;
      const expected = 281;

      expect(pt2(input)).toBe(expected);
    });

    test('multiple of same', () => {
      const input = `two1two
      two1two
      two1two`;
      const expected = 66;

      expect(pt2(input)).toBe(expected);
      expect(pt2(`oneoneoneoneoneoneoneoneone`)).toBe(11);
    });
  });
});
