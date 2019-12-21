const Assert = require('assert');
const readFile = require('../lib/read-file');
const { logAndReturn } = require('../lib/logger');

function manhattan(pointA, pointB) {
  return Math.abs(pointA.x - pointB.x) + Math.abs(pointA.y - pointB.y);
}

const boundManhattan = manhattan.bind(null, { x: 0, y: 0 });

{
  [
    [{ x: 2, y: 5 }, { x: 4, y: 8 }, 5],
    [{ x: 1, y: 6 }, { x: -1, y: 5 }, 3],
    [{ x: 3, y: 5 }, { x: -1, y: 5 }, 4],
    [{ x: 2, y: 3 }, { x: -1, y: 5 }, 5],
  ].forEach(([plane1, plane2, answer]) =>
    Assert.deepEqual(
      manhattan(plane1, plane2),
      answer,
      'Calcualtes Manhattan Distanc'
    )
  );
}

const UP = Symbol.for('Direction$$Up');
const RIGHT = Symbol.for('Direction$$Right');
const DOWN = Symbol.for('Direction$$Down');
const LEFT = Symbol.for('Direction$$Left');

function parseWireSegment(segment) {
  const parsed = {
    direction: null,
    length: null,
  };

  switch (segment.slice(0, 1).toUpperCase()) {
    case 'U': {
      parsed.direction = UP;
      break;
    }
    case 'R': {
      parsed.direction = RIGHT;
      break;
    }
    case 'D': {
      parsed.direction = DOWN;
      break;
    }
    case 'L': {
      parsed.direction = LEFT;
      break;
    }
    default:
      throw new Error('Unknown direction: ', segment.slice(0, 1));
  }

  parsed.length = parseInt(segment.slice(1), 10);

  return parsed;
}

function Wire(coordinateArray) {
  if (!(this instanceof Wire)) {
    return new Wire(coordinateArray);
  }
  // [x, y]
  const position = [0, 0];
  const cells = {};
  const parsedPath = coordinateArray.map(parseWireSegment);
  let totalSteps = 0;

  this.path = parsedPath;
  this.cells = cells;

  this.layWire = function layWire() {
    for (const segment of this.path) {
      let positionIdx = null;
      let additive = null;

      switch (segment.direction) {
        case UP: {
          positionIdx = 1;
          additive = 1;
          position;
          break;
        }
        case RIGHT: {
          positionIdx = 0;
          additive = 1;
          break;
        }
        case DOWN: {
          positionIdx = 1;
          additive = -1;
          break;
        }
        case LEFT: {
          positionIdx = 0;
          additive = -1;
          break;
        }
      }

      for (let i = 0; i < segment.length; i++) {
        position[positionIdx] += additive;
        const coord = position.join(',');
        totalSteps++;

        if (!(coord in cells)) cells[coord] = totalSteps;
      }
    }

    return this;
  };

  this.findIntersections = function findIntersections(wireB) {
    const intersections = [];
    for (const coord in cells) {
      if (coord in wireB.cells) intersections.push(coord);
    }

    return intersections;
  };
}

function findShortestIntersection(intersections) {
  return intersections.reduce((all, curr) => {
    if (!all) return curr;

    const [aX, aY] = all.split(',').map(i => parseInt(i, 10));
    const [bX, bY] = curr.split(',').map(i => parseInt(i, 10));

    const aDis = boundManhattan({ x: aX, y: aY });
    const bDis = boundManhattan({ x: bX, y: bY });

    if (aDis < bDis) return all;

    return curr;
  }, undefined);
}

// [wire, wire, distance]
{
  [
    ['R8,U5,L5,D3', 'U7,R6,D4,L4', 6],
    [
      'R75,D30,R83,U83,L12,D49,R71,U7,L72',
      'U62,R66,U55,R34,D71,R55,D58,R83',
      159,
    ],
    [
      'R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
      'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7',
      135,
    ],
  ].forEach(([wireA, wireB, expected]) => {
    const a = Wire(wireA.split(','));
    const b = Wire(wireB.split(','));

    a.layWire();
    b.layWire();

    const intersections = a.findIntersections(b);
    const shortestDistance = findShortestIntersection(intersections);
    const assertion = shortestDistance
      .split(',')
      .map(i => parseInt(i, 10))
      .reduce((all, curr) => {
        return all + curr;
      }, 0);

    Assert.deepEqual(expected, assertion);
  });
}

readFile(__dirname + '/input.txt')
  .then(([wireA, wireB]) => {
    const a = Wire(wireA.split(','));
    const b = Wire(wireB.split(','));

    a.layWire();
    b.layWire();

    const intersections = a.findIntersections(b);
    const shortestDistance = findShortestIntersection(intersections);

    return shortestDistance
      .split(',')
      .map(i => parseInt(i, 10))
      .map(i => (i < 0 ? i * -1 : i))
      .reduce((all, curr) => {
        return all + curr;
      }, 0);
  })
  .then(logAndReturn('part 1: '));

{
  [
    ['R8,U5,L5,D3', 'U7,R6,D4,L4', 30],
    [
      'R75,D30,R83,U83,L12,D49,R71,U7,L72',
      'U62,R66,U55,R34,D71,R55,D58,R83',
      610,
    ],
    [
      'R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
      'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7',
      410,
    ],
  ].forEach(([wireA, wireB, expected]) => {
    const a = Wire(wireA.split(','));
    const b = Wire(wireB.split(','));

    a.layWire();
    b.layWire();

    const intersections = a.findIntersections(b);
    const intersectionStepCount = intersections.map(
      coord => a.cells[coord] + b.cells[coord]
    );

    const assertion = Math.min(...intersectionStepCount);

    Assert.deepEqual(expected, assertion);
  });
}

readFile(__dirname + '/input.txt')
  .then(([wireA, wireB]) => {
    const a = Wire(wireA.split(','));
    const b = Wire(wireB.split(','));

    a.layWire();
    b.layWire();

    const intersections = a.findIntersections(b);
    const intersectionStepCount = intersections.map(
      coord => a.cells[coord] + b.cells[coord]
    );

    return Math.min(...intersectionStepCount);
  })
  .then(logAndReturn('part 2: '));
