// Generated by BUCKLESCRIPT VERSION 4.0.8, PLEASE EDIT WITH CARE
'use strict';

var $$Array = require("bs-platform/lib/js/array.js");
var Caml_array = require("bs-platform/lib/js/caml_array.js");
var Caml_format = require("bs-platform/lib/js/caml_format.js");
var Input1$Adventofcode2018 = require("./Input1.bs.js");

var input = $$Array.map(Caml_format.caml_int_of_string, Input1$Adventofcode2018.input_data.split("\n"));

function add(a, b) {
  return a + b | 0;
}

var output = $$Array.fold_left(add, 0, input);

console.log("Puzzle 1:" + String(output));

function until_duplicated(_idx, _value, _known_values) {
  while(true) {
    var known_values = _known_values;
    var value = _value;
    var idx = _idx;
    var v = Caml_array.caml_array_get(input, idx);
    var n_v = value + v | 0;
    var n_idx = (idx + 1 | 0) === input.length ? 0 : idx + 1 | 0;
    if (known_values.has(n_v)) {
      return n_v;
    } else {
      _known_values = known_values.add(n_v);
      _value = n_v;
      _idx = n_idx;
      continue ;
    }
  };
}

console.log("Puzzle 2:" + String(until_duplicated(0, 0, new Set())));

exports.input = input;
exports.add = add;
exports.output = output;
exports.until_duplicated = until_duplicated;
/* input Not a pure module */
