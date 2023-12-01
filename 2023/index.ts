const [cliWhich] = Bun.argv.slice(2);
let which: string | number | undefined = cliWhich;

if (!which) {
  const today = new Date();
  const day = today.getDate();

  which = day;
}

if (typeof which === 'string') which = parseInt(which, 10);

const { run } = await import(`./${which < 10 ? `0${which}` : which}`);

console.log(`Ececute day ${which}...`);
run();
