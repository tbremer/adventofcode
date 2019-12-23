const Assert = require('assert');
const readFile = require('../lib/read-file');
const { logAndReturn } = require('../lib/logger');
const { intersection } = require('lodash');

function Planet(name) {
  if (!(this instanceof Planet)) return new Planet(name);

  this.id = name;
  this.orbits = null;
  this.count = function() {
    return this.orbits ? 1 + this.orbits.count() : 0;
  };
}

function Universe(planetList) {
  if (!(this instanceof Universe)) return new Universe(planetList);

  const planets = {};

  for (const map of planetList) {
    const [a, b] = map.split(')');
    if (!(a in planets)) planets[a] = Planet(a);
    if (!(b in planets)) planets[b] = Planet(b);

    planets[b].orbits = planets[a];
  }

  this.countConnections = function() {
    let count = 0;
    for (const key in planets) {
      count += planets[key].count();
    }

    return count;
  };

  this.findPath = function Universe$$FindPath(start, end) {
    let path = typeof start === 'string' ? [start] : [start.id];
    let [lookup] = path;
    // console.log('start:', start);
    // console.log('planets[start]:', planets[start]);
    // console.log('end:', end);
    function next() {
      if (planets[lookup].orbits) {
        path.push(planets[lookup].orbits.id);
        lookup = planets[lookup].orbits.id;

        return lookup === end ? path : next();
      }

      return path;
    }

    return next();
  };

  return this;
}

Assert.deepEqual(
  Universe([
    'COM)B',
    'B)C',
    'C)D',
    'D)E',
    'E)F',
    'B)G',
    'G)H',
    'D)I',
    'E)J',
    'J)K',
    'K)L',
  ]).countConnections(),
  42
);

readFile(__dirname + '/input.txt')
  .then(Universe)
  .then(u => [u, u.countConnections()])
  .then(logAndReturn('Part 1:'));

const testU = Universe([
  'COM)B',
  'B)C',
  'C)D',
  'D)E',
  'E)F',
  'B)G',
  'G)H',
  'D)I',
  'E)J',
  'J)K',
  'K)L',
  'K)YOU',
  'I)SAN',
]);

const ytc = testU.findPath('YOU', 'COM');
const stc = testU.findPath('SAN', 'COM');

let closestHub;
for (const hub of ytc) {
  const idx = stc.indexOf(hub);
  if (idx !== -1) {
    closestHub = hub;
    break;
  }
}

readFile(__dirname + '/input.txt')
  .then(Universe)
  .then(universe => {
    const ytc = universe.findPath('YOU', 'COM');
    const stc = universe.findPath('SAN', 'COM');

    let closestHub;
    for (const hub of ytc) {
      const idx = stc.indexOf(hub);
      if (idx !== -1) {
        closestHub = hub;
        break;
      }
    }

    return ytc.indexOf(closestHub) + stc.indexOf(closestHub) - 2;
  })
  .then(logAndReturn('part 2:'));
