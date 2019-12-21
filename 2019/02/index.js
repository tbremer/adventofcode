const Assert = require('assert');
const readFile = require('../lib/read-file');
const { logAndReturn } = require('../lib/logger');
const Intcode = require('../lib/intcode');

// Test Data
{
  [
    [
      '1,9,10,3,2,3,11,0,99,30,40,50',
      [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
    ],
    ['1,0,0,0,99', [2, 0, 0, 0, 99]],
    ['2,3,0,3,99', [2, 3, 0, 6, 99]],
    ['2,4,4,5,99,0', [2, 4, 4, 5, 99, 9801]],
    ['1,1,1,4,99,5,6,0,99', [30, 1, 1, 4, 2, 5, 6, 0, 99]],
  ].forEach(([test_data, expected]) => {
    const assertion = Intcode(test_data.split(',').map(b => parseInt(b, 10)))
      .run()
      .dump();

    Assert.deepEqual(expected, assertion);
  });
}

readFile(`${__dirname}/input.txt`, ',')
  .then(input => input.map(i => parseInt(i, 10)))
  .then(data_array => {
    data_array[1] = 12;
    data_array[2] = 2;
    return data_array;
  })
  .then(program =>
    Intcode(program)
      .run()
      .dump()
  )
  .then(logAndReturn('part a:'));

// find noun and verb that add up to 19690720
readFile(`${__dirname}/input.txt`, ',')
  .then(input => input.map(i => parseInt(i, 10)))
  .then(program => {
    let a_idx = 0;
    const total = 99;

    for (; a_idx <= total; a_idx++) {
      let temp_idx = 0;
      for (; temp_idx <= total; temp_idx++) {
        const temp_program = [...program];
        temp_program[1] = a_idx;
        temp_program[2] = temp_idx;

        if (
          Intcode(temp_program)
            .run()
            .get(0) === 19690720
        ) {
          return { noun: a_idx, verb: temp_idx };
        }
      }
    }
  })
  .then(({ noun, verb }) => 100 * noun + verb)
  .then(logAndReturn('part b:'));
