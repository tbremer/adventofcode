const Assert = require('assert');
const input = '271973-785961';

const [min, max] = input.split('-').map(i => parseInt(i, 10));

const possibleValues = [];

function isValidPassword(password) {
  const asString = `${password}`;

  // 6 characters long
  if (asString.length < 6) return false;

  // test for concurrent numbers
  if (/(\d)\1/.test(asString) === false) return false;

  return Array.from(asString).every((item, idx, arr) =>
    idx === 0 ? true : item >= arr[idx - 1]
  );
}

{
  [122345, 111123, 111111].forEach(pwd =>
    Assert.deepEqual(isValidPassword(pwd), true, `${pwd} failed`)
  );
}

for (let count = min; count <= max; count++) {
  if (isValidPassword(count)) possibleValues.push(count);
}

console.log('part 1:', possibleValues.length);

const furtherRefined = possibleValues.reduce((all, cur) => {
  const asString = cur.toString();
  const matchesInOrder = [];

  for (const i of asString) {
    if (matchesInOrder.length === 0) {
      matchesInOrder.push(i);
    } else {
      const idx = matchesInOrder.length - 1;

      if (matchesInOrder[idx].slice(-1) === i)
        matchesInOrder[idx] = matchesInOrder[idx] + i;
      else matchesInOrder.push(i);
    }
  }

  if (matchesInOrder.some(i => i.length === 2)) all.push(cur);
  return all;
}, []);

console.log('part 2:', furtherRefined.length);
