module.exports = function Intcode(
  prog,
  debug // = (...args) => console.log(...args)
) {
  if (!Array.isArray(prog))
    throw new TypeError('Program must be an Array of numbers');
  if (!prog.every(i => typeof i === 'number'))
    throw new TypeError('Program must only container numbers');

  if (!(this instanceof Intcode)) return new Intcode(prog, debug);

  const program = prog;
  let position = 0;
  let pointer = program[position];

  function next() {
    position += 1;
    pointer = program[position];
  }

  function advance(int) {
    position += int;
    pointer = program[position];
  }

  function peek(int) {
    const peeked = [];

    for (let i = 0; i < int; i++) {
      // push a =n item from the program that is:
      // current position + offset + count
      peeked.push(program[position + 1 + i]);
    }

    return peeked;
  }

  this.run = function Intcode$Run() {
    debug && debug('run:', pointer);
    switch (pointer) {
      // addition
      case 1: {
        debug && debug('opcode 1, adding');

        const [readFromA, readFromB, storeAt] = peek(3);
        program[storeAt] = program[readFromA] + program[readFromB];

        // move forward 4 spots
        advance(4);

        // continue running program
        return this.run();
      }

      // multiplication
      case 2: {
        debug && debug('opcode 2, multiplying');

        const [readFromA, readFromB, storeAt] = peek(3);
        program[storeAt] = program[readFromA] * program[readFromB];

        // move forward 4 spots
        advance(4);

        // continue running program
        return this.run();
      }

      // halt
      case 99: {
        debug && debug('opcode 99, halting');
        return this;
      }

      default: {
        throw Error(
          `Unknown opcode: \`${pointer}\` at position: \`${position}\` in program: ${program.join(
            ','
          )}`
        );
      }
    }
    return this;
  };

  this.dump = function Intcode$Dump() {
    return program;
  };

  this.get = function Intcode$Get(idx) {
    return program[idx];
  };
};
