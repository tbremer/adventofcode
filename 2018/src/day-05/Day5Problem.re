/* let re = Js.Re.fromStringWithFlags("([A-Z][a-z]|[a-z][A-Z])", ~flags="g"); */
let str = "aA|bB|cC|dD|eE|fF|gG|hH|iI|jJ|kK|lL|mM|nN|oO|pP|qQ|rR|sS|tT|uU|vV|wW|xX|yY|zZ";
let str_backwards =
  str
  |> Js.String.split("")
  |> Js.Array.reverseInPlace
  |> Js.Array.joinWith("");
let polymer_regex =
  Js.Re.fromStringWithFlags(
    "(" ++ str ++ "|" ++ str_backwards ++ ")",
    ~flags="g",
  );
let alphabet = [|
  "a",
  "b",
  "c",
  "d",
  "e",
  "f",
  "g",
  "h",
  "i",
  "j",
  "k",
  "l",
  "m",
  "n",
  "o",
  "p",
  "q",
  "r",
  "s",
  "t",
  "u",
  "v",
  "w",
  "x",
  "y",
  "z",
|];
let test_polymer = Js.Re.test(_, polymer_regex);
let case_insensitive_re = Js.Re.fromStringWithFlags(_, ~flags="gi");
let replace_letter = Js.String.replaceByRe(_, "");
let replace_chain = Js.String.replaceByRe(polymer_regex, "");

let rec remove_pairs: (string, int) => string =
  (input, count) =>
    if (input |> test_polymer) {
      remove_pairs(replace_chain(input), count + 1);
    } else {
      Js.log("Count: " ++ string_of_int(count));
      input;
    };

let find_shortest_polymer_chain = trimmed_chain =>
  Array.fold_left(
    (chain, cur) => {
      let cur_chain =
        replace_letter(case_insensitive_re(cur), trimmed_chain);
      let reacted_chain = remove_pairs(cur_chain, 0);
      let cur_len = String.length(reacted_chain);
      let chain_len = String.length(chain);
      cur_len < chain_len ? reacted_chain : chain;
    },
    trimmed_chain,
    alphabet,
  );

Js.log("Test data (1):");
Js.log(remove_pairs(Day5Data.test_input, 0) === "dabCBAcaDA");

Js.log("Puzzle 1:");
Js.log(String.length(remove_pairs(Day5Data.input, 0)));

Js.log("Test data (2):");
Js.log(find_shortest_polymer_chain(Day5Data.test_input) === "daDA");

Js.log("Puzzle 2:");
Js.log(find_shortest_polymer_chain(Day5Data.input) |> String.length);
