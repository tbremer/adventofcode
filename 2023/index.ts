#!/usr/bin/env bun

const [cliWhich] = Bun.argv.slice(2);
let which: string | number | undefined = cliWhich;

if (!which) {
  const today = new Date();
  const day = today.getDate();

  which = day;
}

if (typeof which === 'string') which = parseInt(which, 10);

const whichString = which < 10 ? `0${which}` : which;
const whichPath = `./day-${whichString}`;

const puzzleInput = Bun.file(whichPath + '/input');

if (!(await puzzleInput.exists())) {
  console.log(
    `Cannot find Input file for day ${which} in "${process.cwd()}".\nExiting… 👋`,
  );
  process.exit(1);
}

const { pt1, pt2 } = await import(whichPath);
const inputText = await puzzleInput.text();

console.log(`Executing day ${which}...`);
console.log('part 1:', pt1(inputText));
console.log('part 2:', pt2(inputText));
