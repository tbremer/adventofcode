// Heap Algorithm for generating permutations
// https://en.wikipedia.org/wiki/Heap%27s_algorithm

function generate(array) {
  const collection = [];

  function permutationGenerator(k, a) {
    if (k === 1) {
      collection.push([...a]);
      return;
    }

    for (let i = 0; i < k; i++) {
      permutationGenerator(k - 1, a);
      if (k % 2 === 0) {
        swap(a, i, k - 1);
      } else {
        swap(a, 0, k - 1);
      }
    }
  }

  permutationGenerator(array.length, array);

  return collection;
}

function swap(arr, idx1, idx2) {
  const tmp = arr[idx1];

  arr[idx1] = arr[idx2];
  arr[idx2] = tmp;

  return arr;
}

// const start = [1, 2, 3];
const collector = generate([1, 2, 3]);

console.log(collector);

module.exports = generate;
