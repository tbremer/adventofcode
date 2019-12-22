const readline = require('readline');

/*
INSTRUCTION KEY:

ABCDE
01002

DE - two-digit opcode,      02 == opcode 2
 C - mode of 1st parameter,  0 == position mode
 B - mode of 2nd parameter,  1 == immediate mode
 A - mode of 3rd parameter,  0 == position mode,

 MODES
 1 = immediate mode - use value of param
 0 = position mode - lookup value at index of param
*/

const IMMEDIATE = Symbol.for('Mode$$Immediate');
const POSITION = Symbol.for('Mode$$Position');

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
  // let pointer = program[position];

  function next() {
    position += 1;
  }

  function advance(int) {
    position += int;
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

  function getParamMode(mode) {
    if (mode === '1' || mode === 1) return IMMEDIATE;
    if (mode === '0' || mode === 0) return POSITION;

    throw new TypeError(`unknown mode: \`${mode}\``);
  }

  this.run = function Intcode$Run() {
    const instructions = program[position].toString().padStart(5, '0');
    const opcode = Number(instructions.slice(3));

    debug && debug('\ninstructions:', instructions);
    debug && debug('opcode:', opcode);

    switch (opcode) {
      // addition
      case 1: {
        debug && debug('opcode 1, adding');

        const [paramMode0, paramMode1, paramMode2] = instructions;
        const [readFromA, readFromB, storeAt] = peek(3);

        const firstValue = paramMode2 === '0' ? program[readFromA] : readFromA;
        const secondValue = paramMode1 === '0' ? program[readFromB] : readFromB;

        debug && debug('firstValue:', firstValue);
        debug && debug('secondValue:', secondValue);

        if (paramMode0 !== '0')
          throw new ReferenceError('Cannot preform write with immediate value');

        program[storeAt] = firstValue + secondValue;

        // move forward 4 spots
        advance(4);

        // continue running program
        return this.run();
      }

      // multiplication
      case 2: {
        debug && debug('opcode 2, multiplying');

        const [paramMode0, paramMode1, paramMode2] = instructions;
        const [readFromA, readFromB, storeAt] = peek(3);

        const firstValue = paramMode2 === '0' ? program[readFromA] : readFromA;
        const secondValue = paramMode1 === '0' ? program[readFromB] : readFromB;

        if (paramMode0 !== '0')
          throw new ReferenceError('Cannot preform write with immediate value');

        program[storeAt] = firstValue * secondValue;

        // move forward 4 spots
        advance(4);

        // continue running program
        return this.run();
      }

      // place input at param
      case 3: {
        debug && debug('opcode 3, place value at param');

        const [storeAt] = peek(1);

        const rl = readline.createInterface({
          input: process.stdin,
          output: process.stdout,
        });

        rl.question('Input Value: ', answer => {
          rl.close();
          const value = answer;

          debug && debug('storeAt:', storeAt);
          debug && debug('value:', value);

          program[storeAt] = Number(value);

          advance(2);

          return this.run();
        });
        break;
      }

      // output a value
      case 4: {
        debug && debug('opcode 4, output value at param');
        debug && debug('instructions:', instructions);

        const [, , paramMode] = instructions;
        const [readFrom] = peek(1);

        debug && debug('paramMode:', paramMode, typeof paramMode);
        debug && debug('peek(1):', peek(1));
        debug && debug('program[readFrom]:', program[readFrom]);

        const outputValue = paramMode === '1' ? readFrom : program[readFrom];

        if (outputValue !== 0) {
          const [, nextInstruction] = peek(2);

          // if (nextInstruction !== 99)
          //   throw new Error(
          //     `Read error occured at position \`${position}\` with previous instructions of: ${program.slice(
          //       0,
          //       position
          //     )}.`
          //   );
        }

        console.log('Output:', outputValue);

        advance(2);

        return this.run();
      }

      // jump if true
      case 5: {
        debug && debug('opcode 5, jump if true');
        const [, paramModeForParam2, paramModeForParam1] = instructions;
        const [p1, p2] = peek(2);

        const isNonZero =
          getParamMode(paramModeForParam1) === IMMEDIATE ? p1 : program[p1];
        const value =
          getParamMode(paramModeForParam2) === IMMEDIATE ? p2 : program[p2];

        if (isNonZero !== 0) position = value;
        else advance(3);

        return this.run();
      }

      // jump if false
      case 6: {
        debug && debug('opcode 5, jump if true');
        const [, paramModeForParam2, paramModeForParam1] = instructions;
        const [p1, p2] = peek(2);

        const isZero =
          getParamMode(paramModeForParam1) === IMMEDIATE ? p1 : program[p1];
        const value =
          getParamMode(paramModeForParam2) === IMMEDIATE ? p2 : program[p2];

        if (isZero === 0) position = value;
        else advance(3);

        return this.run();
      }

      // less than
      case 7: {
        debug && debug('opcode 7, less than');
        const [, paramModeForParam2, paramModeForParam1] = instructions;
        const [p1, p2, storeAt] = peek(3);

        const valA = paramModeForParam1 === '1' ? p1 : program[p1];
        const valB = paramModeForParam2 === '1' ? p2 : program[p2];

        program[storeAt] = valA < valB ? 1 : 0;

        advance(4);

        return this.run();
      }

      // equals
      case 8: {
        debug && debug('opcode 8, equals');
        const [, paramModeForParam2, paramModeForParam1] = instructions;
        const [p1, p2, storeAt] = peek(3);

        const valA = paramModeForParam1 === '1' ? p1 : program[p1];
        const valB = paramModeForParam2 === '1' ? p2 : program[p2];

        program[storeAt] = valA === valB ? 1 : 0;

        advance(4);

        return this.run();
      }

      // halt
      case 99: {
        debug && debug('opcode 99, halting');
        console.log('HALT');
        return this;
      }

      default: {
        throw Error(
          `Unknown opcode: \`${opcode}\` at position: \`${position}\` in program: ${program.join(
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
