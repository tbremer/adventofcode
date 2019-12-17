type log = Js.Dict.t(array(float));

let input_data = Js.String.split("\n", Day4Data.input);

Array.sort((a, b) => a < b ? (-1) : a > b ? 1 : 0, input_data);

let to_proof: string => option(Utils.proves) =
  str =>
    switch (Js.Re.exec(str, Utils.main_parser)) {
    | None => raise(Utils.Parse_error("Unable to parse input"))
    | Some(res) =>
      let result = Js.Re.captures(res);
      let date = Utils.to_opt(result[1]);
      let action_string = Utils.to_opt(result[2]);

      let timestamp = Utils.parse_timestamp(date);
      let possible_action = Utils.parse_action(action_string);

      let guard_id: option(int) =
        if (possible_action !== StartShift) {
          None;
        } else {
          Utils.parse_guard_id(action_string);
        };

      switch (guard_id) {
      | None => Some(ProofOfWork(timestamp, possible_action))
      | Some(id) => Some(ProofOfExistence(timestamp, possible_action, id))
      };
    };

exception Error(string);
let soi = string_of_int;
let last_guard = ref(-1);
let captains_log: (log, string) => log =
  (acc, cur) => {
    let proof = to_proof(cur);

    switch (proof) {
    | None => raise(Error("Could not parse proof"))
    | Some(ProofOfExistence(_parse_timestamp, _possible_action, id)) =>
      switch (Js.Dict.get(acc, soi(id))) {
      | None => Js.Dict.set(acc, soi(id), [||])
      | Some(_id) => ()
      };

      last_guard := id;
    | Some(ProofOfWork(timestamp, _possible_action)) =>
      let item =
        switch (Js.Dict.get(acc, soi(last_guard^))) {
        | None =>
          raise(Error("Could not get last guard: " ++ soi(last_guard^)))
        | Some(item) => item
        };
      let append = Array.append(item);
      Js.Dict.set(acc, soi(last_guard^), append([|timestamp|]));
    };

    acc;
  };

let guard_log = Array.fold_left(captains_log, Js.Dict.empty(), input_data);

let get_time_from_proof: option(Utils.proves) => int =
  proof =>
    switch (proof) {
    | None => 0
    | Some(ProofOfWork(timestamp, _possible_action)) =>
      int_of_float(timestamp)
    | Some(ProofOfExistence(timestamp, _possible_action, _id)) =>
      int_of_float(timestamp)
    };

let get_range: (int, int) => array(float) =
  (s, e) =>
    Array.mapi((idx, _n) => float_of_int(s + idx), Array.make(e - s, 0));

let get_minutes: float => float =
  ts => ts |> Js.Date.fromFloat |> Js.Date.getMinutes;

let nap_length: (float, float) => float =
  (a, b) => get_minutes(b) -. get_minutes(a);

Array.iter(
  item => {
    let (guard_id, timestamps) = item;
    let len = Array.length(timestamps) - 1;
    let nap_array = ref([||]);

    for (ts_idx in 0 to len) {
      if (ts_idx mod 2 === 0) {
        let start_nap = int_of_float(get_minutes(timestamps[ts_idx]));
        let end_nap = int_of_float(get_minutes(timestamps[ts_idx + 1]));
        let nap_minutes = get_range(start_nap, end_nap);
        nap_array := Array.append(nap_array^, nap_minutes);
      };
    };
    Js.Dict.set(guard_log, guard_id, nap_array^);
  },
  Js.Dict.entries(guard_log),
);

let most_minutes =
  Array.fold_left(
    ((sleepiest_guard, longest_nap), (guard_id, minutes)) =>
      Array.length(minutes) > longest_nap ?
        (guard_id, Array.length(minutes)) : (sleepiest_guard, longest_nap),
    ("", 0),
    Js.Dict.entries(guard_log),
  );

let (sleepiest_guard, _) = most_minutes;
let nap_times =
  switch (Js.Dict.get(guard_log, sleepiest_guard)) {
  | None => raise(Error("Could not fetch guard"))
  | Some(i) => i
  };

let get_minute_count =
  Array.fold_left((map, cur) => {
    switch (Js.Dict.get(map, string_of_float(cur))) {
    | None => Js.Dict.set(map, string_of_float(cur), 1)
    | Some(count) => Js.Dict.set(map, string_of_float(cur), count + 1)
    };
    map;
  });
let find_sleepiest_minute =
  Array.fold_left(
    ((acc_minute, acc_count), (cur_minute, cur_count)) =>
      cur_count > acc_count ?
        (cur_minute, cur_count) : (acc_minute, acc_count),
    ("", 0),
  );
let sleepiest_minute =
  find_sleepiest_minute(
    Js.Dict.entries(get_minute_count(Js.Dict.empty(), nap_times)),
  );

let (guard_id, _) = most_minutes;
let (minute_id, _) = sleepiest_minute;

Js.log("Puzzle 1:");
Js.log(int_of_string(guard_id) * int_of_float(float_of_string(minute_id)));
Js.log("");

let find_sleepies = a => {
  Array.sort(
    ((_mina, a_cnt), (_minb, b_cnt)) =>
      a_cnt === b_cnt ? 0 : a_cnt > b_cnt ? 1 : (-1),
    a,
  );

  a[Array.length(a) - 1];
};

let p2 =
  Array.fold_left(
    (acc, cur) => {
      let (_a_guard, _a_minute, a_count) = acc;
      let (c_guard, minutes) = cur;
      if (Array.length(minutes) === 0) {
        acc;
      } else {
        let min_count = get_minute_count(Js.Dict.empty(), minutes);
        let most_likely_asleep = find_sleepies(Js.Dict.entries(min_count));
        let (c_min, cur_count) = most_likely_asleep;
        if (cur_count > a_count) {
          (c_guard, c_min, cur_count);
        } else {
          acc;
        };
      };
    },
    ("", "", 0),
    Js.Dict.entries(guard_log),
  );

let (guard_id, cur_min, _) = p2;

Js.log("Puzzle 2:");
Js.log(guard_id->float_of_string *. cur_min->float_of_string);
