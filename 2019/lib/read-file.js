const {
  promises: { readFile },
} = require('fs');

module.exports = async function(path, split = true) {
  return readFile(path, 'utf-8').then(s => (split ? s.split(/\r?\n/) : s));
};
