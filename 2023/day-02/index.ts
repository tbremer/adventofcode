import { asLines } from '../utils/string';

export function pt1(input: string | string[]): number {
  const max = {
    red: 12,
    green: 13,
    blue: 14,
  };
  const lines = asLines(input);
  const parsed = lines.map(parseLine);

  return parsed.reduce((acc, cur) => {
    if (
      cur.rounds.every(
        (round) =>
          round.red <= max.red &&
          round.green <= max.green &&
          round.blue <= max.blue,
      )
    ) {
      return acc + cur.id;
    }

    return acc;
  }, 0);
}

export function pt2(input: string): number {
  const lines = asLines(input);
  const parsed = lines.map(parseLine);

  const maxPerGame = parsed.map((g) => {
    const max = g.rounds.reduce(
      (acc, cur) => {
        acc.red = Math.max(acc.red, cur.red);
        acc.green = Math.max(acc.green, cur.green);
        acc.blue = Math.max(acc.blue, cur.blue);

        return acc;
      },
      {
        red: 0,
        green: 0,
        blue: 0,
      },
    );

    return { id: g.id, max };
  });

  return maxPerGame.reduce((acc, cur) => {
    return acc + cur.max.red * cur.max.green * cur.max.blue;
  }, 0);
}

const gameRegex = /game (\d+):/i;

function parseLine(line: string): {
  id: number;
  rounds: {
    red: number;
    green: number;
    blue: number;
  }[];
} {
  const gameMatcher = line.match(gameRegex);
  const gameId = gameMatcher ? gameMatcher[1] : -1;
  const rounds = line
    .replace(gameRegex, '')
    .split(';')
    .map((round) => {
      const colors = round.split(',').map((c) => c.trim());
      return colors.reduce(
        (acc, cur) => {
          const [count, color] = cur.split(' ');
          const valid = ensureColorName(color);

          acc[valid] = Number(count);

          return acc;
        },
        {
          red: 0,
          green: 0,
          blue: 0,
        },
      );
    });

  return {
    id: Number(gameId),
    rounds,
  };
}

type Color = 'red' | 'green' | 'blue';

function ensureColorName(color: string): Color {
  if (color === 'red' || color === 'green' || color === 'blue') {
    return color;
  }

  throw new Error(`Invalid color: ${color}`);
}
