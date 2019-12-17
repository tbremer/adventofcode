type elf_tuple = (int, array(int), array(int));

let split_by_space: string => array('string) = Js.String.split(" ");
let split_by_new_line: string => array('string) = Js.String.split("\n");
let split_at_comma: string => array('string) = Js.String.split(",");
let split_at_x: string => array('string) = Js.String.split("x");
let map_to_int: array('string) => array('int) = Array.map(int_of_string);

let remove_last_char: string => string =
  str => String.sub(str, 0, String.length(str) - 1);

let remove_first_char: string => string =
  str => String.sub(str, 1, String.length(str) - 1);

let remove_first_char_parse_number: string => int =
  value => int_of_string(remove_first_char(value));

let to_tuple: array('string) => elf_tuple =
  data => {
    let get = Array.get(data);
    let elf_id = remove_first_char_parse_number(get(0));
    let coords = map_to_int(split_at_comma(remove_last_char(get(2))));
    let dimensions = map_to_int(split_at_x(get(3)));

    (elf_id, coords, dimensions);
  };
let base_array =
  Array.map(
    to_tuple,
    Array.map(split_by_space, split_by_new_line(Day3Data.input)),
  );

let matrix = Array.make_matrix(1000, 1000, (0, [||]));
let intersecting_ids: ref(FastList.t(int)) = ref(FastList.make_empty());
let all_ids: ref(FastList.t(int)) = ref(FastList.make_empty());

let place_on_matrix: elf_tuple => unit =
  ((id, coords, dimensions)) => {
    let coords_get = Array.get(coords);
    let dimensions_get = Array.get(dimensions);
    let x = coords_get(0);
    let y = coords_get(1);
    let w = dimensions_get(0) - 1;
    let h = dimensions_get(1) - 1;

    all_ids := FastList.add(all_ids^, id);

    Array.iteri(
      (x_idx, row) =>
        if (x_idx >= x && x_idx <= x + w) {
          Array.iteri(
            (y_idx, _col) =>
              if (y_idx >= y && y_idx <= y + h) {
                let (count, all_ids) = matrix[x_idx][y_idx];
                let a = Array.append(all_ids, [|id|]);
                let c = count + 1;

                if (c > 1) {
                  Array.iter(
                    i =>
                      if (!FastList.has(intersecting_ids^, i)) {
                        let b = FastList.add(intersecting_ids^, i);
                        intersecting_ids := b;
                      },
                    a,
                  );
                  intersecting_ids := FastList.add(intersecting_ids^, id);
                };

                matrix[x_idx][y_idx] = (c, a);
              },
            row,
          );
        },
      matrix,
    );
  };

Array.iter(place_on_matrix, base_array);

let total =
  Array.fold_left(
    (acc, row) =>
      acc
      + Array.fold_left(
          (acc, (count, _item)) => count > 1 ? acc + 1 : acc,
          0,
          row,
        ),
    0,
    matrix,
  );

Js.log("Puzzle 1: " ++ string_of_int(total));

exception Type_error(string);
let non_intersecting =
  Array.fold_left(
    (all, cur) =>
      if (!FastList.has(intersecting_ids^, cur)) {
        if (all !== (-1)) {
          raise(
            Type_error(
              "All has an id already ("
              ++ string_of_int(all)
              ++ "), but we tried: "
              ++ string_of_int(cur),
            ),
          );
        } else {
          cur;
        };
      } else {
        all;
      },
    -1,
    FastList.to_array(all_ids^),
  );

Js.log("Puzzle 2: " ++ string_of_int(non_intersecting));
