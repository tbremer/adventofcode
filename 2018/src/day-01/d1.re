let input: array(int) =
  Array.map(int_of_string, Js.String.split("\n", Input1.input_data));
let add = (a, b) => a + b;
let output: int = Array.fold_left(add, 0, input);

Js.log("Puzzle 1:" ++ string_of_int(output));

let rec until_duplicated =
        (idx: int, value: int, known_values: FastList.t(int)) => {
  let v = input[idx];
  let n_v = value + v;
  let n_idx =
    if (idx + 1 === Array.length(input)) {
      0;
    } else {
      idx + 1;
    };

  if (FastList.has(known_values, n_v)) {
    n_v;
  } else {
    until_duplicated(n_idx, n_v, FastList.add(known_values, n_v));
  };
};

Js.log(
  "Puzzle 2:" ++ string_of_int(until_duplicated(0, 0, FastList.make_empty())),
);
