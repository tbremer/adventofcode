/*
 /* let test_input = "[1518-06-12 00:00] Guard #3359 begins shift"; */
 /* let test_input = "[1518-10-08 00:19] wakes up"; */
 /* let test_input = "this is a far"; */
 /* let test_input = "[1518-06-07 00:26] falls asleep"; */

 /* let to_proof: string => option(Utils.proves) =
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
      }; */

 let get_time_from_proof: option(Utils.proves) => float =
   proof =>
     switch (proof) {
     | None => 0.0
     | Some(ProofOfWork(timestamp, _possible_action)) => timestamp
     | Some(ProofOfExistence(timestamp, _possible_action, _id)) => timestamp
     };

 /*let get_id_from_proof: option(Utils.proves) => option(int) =
   proof =>
     switch (proof) {
     | None => None
     | Some(ProofOfWork(_timestamp, _possible_action)) => None
     | Some(ProofOfExistence(_timestamp, _possible_action, id)) => Some(id)
     }; */

 let to_proof_dict = (acc, cur) => {
   Js.log(cur);
   acc;
 };

 let guard_tower =
   Array.fold_left(
     to_proof_dict,
     Js.Dict.empty(),
     Js.String.split("\n", Day4Data.input),
   );

 /* Array.sort(
      (a, b) => {
        let ta = get_time_from_proof(a);
        let tb = get_time_from_proof(b);
        ta < tb ? (-1) : ta === tb ? 0 : 1;
      },
      guard_tower,
    ); */

 let all_ids: FastList.t(int) =
   Array.fold_left(
     (set, item) =>
       switch (get_id_from_proof(item)) {
       | None => set
       | Some(id) => FastList.has(set, id) ? set : FastList.add(set, id)
       },
     FastList.make_empty(),
     guard_tower,
   );

 let unique_ids =
   Array.fold_left((acc, _i) => acc + 1, 0, FastList.to_array(all_ids));

 /* type action_tbl = Hashtbl.t(Utils.action, float);
    type tracker = {
      id: int,
      actions_logged: array(action_tbl),
    };
    type tracker_tbl = Hashtbl.t(int, tracker);

    let make_action: (Utils.action, float) => action_tbl =
      (a, t) => {
        let tbl = Hashtbl.create(1);

        Hashtbl.add(tbl, a, t);

        tbl;
      };

    let last_guard: ref(int) = ref(0);

    let guard_table: tracker_tbl =
      Array.fold_left(
        (tbl, cur_input: option(Utils.proves)) =>
          switch (cur_input) {
          | None => tbl
          | Some(ProofOfWork(timestamp, action)) =>
            let last_record = Hashtbl.find(tbl, last_guard^);
            Hashtbl.replace(
              tbl,
              last_guard^,
              {
                ...last_record,
                actions_logged:
                  Array.append(
                    last_record.actions_logged,
                    [|make_action(action, timestamp)|],
                  ),
              },
            );
            tbl;
          | Some(ProofOfExistence(timestamp, action, id)) =>
            Hashtbl.add(
              tbl,
              id,
              {id, actions_logged: [|make_action(action, timestamp)|]},
            );

            /* mutate last guard */
            last_guard := id;

            tbl;
          },
        Hashtbl.create(unique_ids),
        guard_tower,
      );

    /*
     {
       [guard_id]: array(sleeping_minutes)
     }
      */

    let sleeping: Js.Dict.t(array(float)) = Js.Dict.empty();

    let set_action = Js.Dict.set(sleeping);
    let get_action = Js.Dict.get(sleeping);

    let add_time = (id, time) => {
      let id = string_of_int(id);
      switch (get_action(id)) {
      | None => set_action(id, [|time|])
      | Some(times) => set_action(id, Array.append(times, [|time|]))
      };
    };

    Hashtbl.iter(
      (id, value) =>
        Array.iter(
          act_tbl =>
            Hashtbl.iter(
              (act, time) =>
                switch (act) {
                | Utils.StartShift => ()
                | FallAsleep => add_time(id, time)
                | WakeUp => add_time(id, time)
                },
              act_tbl,
            ),
          value.actions_logged,
        ),
      guard_table,
    );

    let diff_to_min: (float, float) => int =
      (a, b) => int_of_float((a -. b) /. 1000.0 /. 60.0);

    let most_mins =
      Array.fold_left(
        (most_sleeps, (guard_id, sleeps)) =>
          if (Array.length(sleeps) > most_sleeps) {
            int_of_string(guard_id);
          } else {
            most_sleeps;
          },
        -1,
        Js.Dict.entries(sleeping),
      );

    let recordings =
      switch (get_action(string_of_int(most_mins))) {
      | None => [|0.0, 0.0|]
      | Some(time) => time
      };

    Array.mapi(
      (idx, s) =>
        if (idx mod 2 !== 0) {
          (-1);
        } else {
          let start_nap = Js.Date.getMinutes(Js.Date.fromFloat(s));
          let end_nap =
            Js.Date.getMinutes(Js.Date.fromFloat(recordings[idx + 1]));

          Js.log(end_nap -. start_nap);

          /* Js.log(start_nap); */
          /* Js.log(end_nap); */
          Js.log("===");

          2;
        },
      recordings,
    ); */ */
