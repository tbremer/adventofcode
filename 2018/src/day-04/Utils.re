type action =
  | StartShift
  | FallAsleep
  | WakeUp;

type proves =
  | ProofOfWork(float, action)
  | ProofOfExistence(float, action, int);

let to_opt = Js.Nullable.toOption;

exception Parse_error(string);

let main_parser = Js.Re.fromString("\\[([\\d-].+?)\\](.+$)");
let guard_parser = Js.Re.fromString("Guard #(\\d.+?) (.+$)");

let parse_timestamp: option(string) => float =
  str =>
    switch (str) {
    | None => 0.0
    | Some(timestamp) =>
      let timestamp = String.trim(timestamp);

      Js.Date.getTime(Js.Date.fromString(timestamp));
    };

let parse_action: option(string) => action =
  str =>
    switch (str) {
    | None => raise(Parse_error("Ah no!"))
    | Some(pa) =>
      if (Js.Re.test(pa, guard_parser)) {
        switch (Js.Re.exec(pa, guard_parser)) {
        | None => raise(Parse_error("Can't parse guard!"))
        | Some(_guard_res) => StartShift
        };
      } else {
        switch (String.trim(pa)) {
        | "wakes up" => WakeUp
        | "falls asleep" => FallAsleep
        | _ => raise(Parse_error("Could not get action"))
        };
      }
    };

let parse_guard_id: option(string) => option(int) =
  str =>
    switch (str) {
    | None => raise(Parse_error("Ah no!"))
    | Some(pa) =>
      if (Js.Re.test(pa, guard_parser)) {
        switch (Js.Re.exec(pa, guard_parser)) {
        | None => raise(Parse_error("Can't parse guard!"))
        | Some(guard_res) =>
          let guard_results = Js.Re.captures(guard_res);
          switch (to_opt(guard_results[1])) {
          | None => raise(Parse_error("No results from guard"))
          | Some(id) => Some(int_of_string(id))
          };
        };
      } else {
        None;
      }
    };
