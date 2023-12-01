export const asLines = (input: string | string[]) =>
  Array.isArray(input) ? input : input.split('\n').map((line) => line.trim());

export const removeAlpha = (lines: string[]) =>
  lines.map((line) => line.replace(/[a-z]/gi, ''));
