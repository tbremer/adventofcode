const {
  promises: { readFile },
} = require('fs');

module.exports = async function(path, split = /\r?\n/) {
  return readFile(path, 'utf-8').then(s => s.split(split));
};
