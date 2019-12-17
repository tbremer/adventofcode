const Assert = require('assert');
const readFile = require('../lib/read-file');
const { logAndReturn } = require('../lib/logger');

function massToFuelUnit(mass) {
  return Math.floor(mass / 3) - 2;
}

function sumFuelUnits(all, curr) {
  return all + curr;
}

function getFuelNeeded(mass) {
  return mass.map(massToFuelUnit).reduce(sumFuelUnits);
}

function mapRecursive(i) {
  return recursiveMassGetter(i);
}

function recursiveMassGetter(mass, totalMass = 0) {
  const addTo = massToFuelUnit(mass);

  if (addTo > 0) return recursiveMassGetter(addTo, totalMass + addTo);
  else return totalMass;
}

Assert.deepEqual([[12], [14], [1969], [100756]].map(getFuelNeeded), [
  2,
  2,
  654,
  33583,
]);

Assert.deepEqual(
  [14, 1969, 100756].map(i => recursiveMassGetter(i)),
  [2, 966, 50346]
);

function add(a, b) {
  return a + b;
}

readFile(__dirname + '/input-a.txt')
  .then(i => i.map(line => parseInt(line, 10)))
  .then(getFuelNeeded)
  .then(logAndReturn('part a:'));

readFile(__dirname + '/input-a.txt')
  .then(i => i.map(line => parseInt(line, 10)))
  .then(modules => modules.map(mapRecursive))
  .then(mass => mass.reduce(sumFuelUnits, 0))
  .then(logAndReturn('part b:'));
