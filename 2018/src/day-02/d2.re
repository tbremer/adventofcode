let split_new_lines = Js.String.split("\n");

let is_int: (int, int) => bool = (desired_num, num) => desired_num === num;
let is_two = is_int(2);
let is_three = is_int(3);
let if_two_add = (acc, test) => is_two(test) ? acc + 1 : acc;
let if_three_add = (acc, test) => is_three(test) ? acc + 1 : acc;

let string_to_letter_map: string => Js.Dict.t(int) =
  (input_string: string) => {
    let hash = Js.Dict.empty();
    let get = Js.Dict.get(hash);
    let set = Js.Dict.set(hash);
    let make = String.make(1);

    for (idx in 0 to String.length(input_string) - 1) {
      let ch_str = make(input_string.[idx]);
      switch (get(ch_str)) {
      | None => set(ch_str, 1)
      | Some(ch) => set(ch_str, ch + 1)
      };
    };

    hash;
  };
type key_int_tuple = (Js.Dict.key, int);
type tuple_array = array(key_int_tuple);
type tuple_array_array = array(tuple_array);
let a =
  Array.map(
    Js.Dict.entries,
    Array.map(string_to_letter_map, split_new_lines(Input2.input)),
  );

let num_of_twos =
  Array.fold_left(
    (acc, cur) => {
      let twos =
        Array.fold_left((acc, (_, cur)) => if_two_add(acc, cur), 0, cur);
      twos > 0 ? acc + 1 : acc;
    },
    0,
    a,
  );

let num_of_threes =
  Array.fold_left(
    (acc, cur) => {
      let threes =
        Array.fold_left((acc, (_, cur)) => if_three_add(acc, cur), 0, cur);
      threes > 0 ? acc + 1 : acc;
    },
    0,
    a,
  );
Js.log("Test 1: " ++ string_of_int(num_of_twos * num_of_threes));

let input = split_new_lines(Input2.input);

let eqq: ('a, 'a) => bool = (a, b) => a === b;

let close_enough = (a: string, b: string): bool => {
  let matches = ref(0);
  let usable_len = String.length(a) - 1;
  for (c in 0 to usable_len) {
    let a = a.[c];
    let b = b.[c];

    if (!eqq(a, b)) {
      matches := matches^ + 1;
    };
  };

  matches^ === 1;
};

let rec find_matching: (array(string), int) => (string, string) =
  (haystack, idx) => {
    let get = Array.get(haystack);
    let needle = get(idx);

    switch (
      List.find(i => close_enough(needle, i), Array.to_list(haystack))
    ) {
    | item => (needle, item)
    | exception Not_found => find_matching(haystack, idx + 1)
    };
  };

let drop_mismatch: ((string, string)) => string =
  ((a, b)) => {
    let str = ref("");
    let len = String.length(a) - 1;

    for (c in 0 to len) {
      let aa = a.[c];
      let bb = b.[c];

      if (aa === bb) {
        str := str^ ++ String.make(1, aa);
      };
    };

    str^;
  };

let match = drop_mismatch(find_matching(input, 0));
Js.log("Test 2: " ++ match);
