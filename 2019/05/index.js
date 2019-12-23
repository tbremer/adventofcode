const Assert = require('assert');
const readFile = require('../lib/read-file');
const Intcode = require('../lib/intcode');
const { logAndReturn } = require('../lib/logger');

Assert.deepEqual(
  Intcode('1101,100,-1,4,0'.split(',').map(Number)) //, console.log)
    .run()
    .get(4),
  99
);

Assert.deepEqual(
  Intcode('1002,4,3,4,33'.split(',').map(Number))
    .run()
    .dump(),
  [1002, 4, 3, 4, 99]
);

Assert.deepEqual(
  Intcode([1101, 100, -1, 4, 0])
    .run()
    .dump(),
  [1101, 100, -1, 4, 99]
);

readFile(__dirname + '/input.txt', ',')
  .then(data => data.map(Number))
  .then(program => Intcode(program, [1]).run());

readFile(__dirname + '/input.txt', ',')
  .then(data => data.map(Number))
  .then(program => Intcode(program, [5]).run());
