import { asLines } from '../utils/string';

export function pt1(input: string | string[]): number {
  const lines = asLines(input);

  return linesAsNumbers(lines, /(\d).*$/, /^.*(\d)/).reduce(
    (acc, curr) => acc + curr,
    0,
  );
}

export function pt2(input: string): number {
  const regStart = /(\d|one|two|three|four|five|six|seven|eight|nine).*$/i;
  const regEnd = /^.*(\d|one|two|three|four|five|six|seven|eight|nine)/i;
  const lines = asLines(input);

  return linesAsNumbers(lines, regStart, regEnd).reduce(
    (acc, curr) => acc + curr,
    0,
  );
}

function linesAsNumbers(
  lines: string[],
  regStart: RegExp,
  regEnd: RegExp,
): number[] {
  return lines.map((l) => {
    const start = l.match(regStart);
    const end = l.match(regEnd);

    if (!start || !end) {
      throw new Error(`Could not find start or end of line \`${l}\``);
    }

    return getNfromStr(start[1]) * 10 + getNfromStr(end[1]);
  });
}

function getNfromStr(str: string): number {
  switch (str) {
    case 'one':
    case '1':
      return 1;
    case 'two':
    case '2':
      return 2;
    case 'three':
    case '3':
      return 3;
    case 'four':
    case '4':
      return 4;
    case 'five':
    case '5':
      return 5;
    case 'six':
    case '6':
      return 6;
    case 'seven':
    case '7':
      return 7;
    case 'eight':
    case '8':
      return 8;
    case 'nine':
    case '9':
      return 9;
    default:
      throw new Error(`Unknown value \`${str}\``);
  }
}
