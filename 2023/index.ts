const [cliWhich] = Bun.argv.slice(2);
let which: string | number | undefined = cliWhich;

if (!which) {
  const today = new Date();
  const day = today.getDate();

  which = day;
}

if (typeof which === 'string') which = parseInt(which, 10);

const whichPath = `./${which < 10 ? `0${which}` : which}`;

const { pt1, pt2 } = await import(whichPath);

const pt1Input = Bun.file(whichPath + '/1.part');
let pt2Input = Bun.file(whichPath + '/2.part');

if (!(await pt2Input.exists())) pt2Input = pt1Input;

if (!(await pt1Input.exists())) {
  console.log(`Input file for day ${which} not found\n`);
  process.exit(1);
}

console.log(`Executing day ${which}...`);
console.log('part 1:', pt1(await pt1Input.text()));
console.log('part 2:', pt2(await pt2Input.text()));
