import { asLines } from '../utils/string';

export function pt1(input: string | string[]): {
  sum: number;
  gearRatioSum: number;
} {
  const lines = asLines(input);
  const matrix = lines.map((l) => l.split(''));
  let sum = 0;
  const gears: Map<`${number},${number}`, number[]> = new Map();
  const thing: {
    value: `${number}` | '';
    hasSymbol: boolean;
    symbol: null | ReturnType<typeof checkForSymbol>;
  } = {
    value: '',
    hasSymbol: false,
    symbol: null,
  };

  for (const [lineNum, line] of lines.entries()) {
    thing.value = '';
    thing.hasSymbol = false;

    for (const [charIdx, char] of [...line].entries()) {
      if (/\D/.test(char)) {
        if (thing.hasSymbol && thing.value) {
          sum += Number(thing.value);

          if (
            thing.symbol &&
            thing.symbol.value === '*' &&
            thing.symbol.location
          ) {
            const gearsKey: `${number},${number}` = `${thing.symbol.location.row},${thing.symbol.location.col}`;
            const gearsValue = gears.get(gearsKey) ?? [];

            gears.set(gearsKey, [...gearsValue, Number(thing.value)]);
          }
        }

        thing.hasSymbol = false;
        thing.value = '';
        thing.symbol = null;
        continue;
      }

      thing.value += char;

      if (!thing.hasSymbol) {
        const symbolData = checkForSymbol(lineNum, charIdx, matrix);
        thing.hasSymbol = symbolData.has;
        thing.symbol = symbolData;
      }
    }

    // don't like this duplication, buuuut here we are.
    if (thing.hasSymbol && thing.value) {
      sum += Number(thing.value);

      if (thing.symbol && thing.symbol.value === '*' && thing.symbol.location) {
        const gearsKey: `${number},${number}` = `${thing.symbol.location.row},${thing.symbol.location.col}`;
        const gearsValue = gears.get(gearsKey) ?? [];

        gears.set(gearsKey, [...gearsValue, Number(thing.value)]);
      }
    }

    thing.hasSymbol = false;
    thing.value = '';
    thing.symbol = null;
  }

  let gearRatioSum = 0;

  for (const [, values] of gears.entries()) {
    if (values.length === 2) {
      gearRatioSum += values[0] * values[1];
    }
  }

  return { sum, gearRatioSum };
}

export function pt2(input: string): number {
  return pt1(input).gearRatioSum;
}

type Symbols = '*' | '/' | '-' | '@' | '$' | '=' | '%' | '+' | '#' | '&';
type Dir = -1 | 0 | 1;
type XDir = Dir;
type YDir = Dir;
const directions: { [key: string]: [XDir, YDir] } = {
  t: [0, -1],
  tr: [1, -1],
  r: [1, 0],
  br: [1, 1],
  b: [0, 1],
  bl: [-1, 1],
  l: [-1, 0],
  tl: [-1, -1],
};

function checkForSymbol(
  matrixRow: number,
  matrixColumn: number,
  matrix: string[][],
): {
  has: boolean;
  value: null | string;
  location: null | { row: number; col: number };
} {
  const data: ReturnType<typeof checkForSymbol> = {
    has: false,
    value: null,
    location: null,
  };
  for (const [xCoord, yCoord] of Object.values(directions)) {
    const dirRow = matrix[matrixRow + yCoord];

    if (!dirRow) continue;

    const dirCol = dirRow[matrixColumn + xCoord];

    if (!dirCol) continue;

    if (/[^\d\.]/.test(dirCol)) {
      data.has = true;
      data.value = `${dirCol}`;
      data.location = {
        row: matrixRow + yCoord,
        col: matrixColumn + xCoord,
      };
      break;
    }
  }

  return data;
}
