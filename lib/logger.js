module.exports.logAndReturn = function logAndReturn(message) {
  return function(input) {
    console.log(message, input);

    return input;
  };
};
