type t('a);

[@bs.new] external make: array('a) => t('a) = "Set";
[@bs.new] external make_empty: unit => t('a) = "Set";
[@bs.new] external from_string: string => t(string) = "Set";
[@bs.new] external from_int: int => t(int) = "Set";
[@bs.send] external add: (t('a), 'a) => t('a) = "";
[@bs.send] external has: (t('a), 'a) => bool = "";
[@bs.val] external to_array: t('a) => array('a) = "Array.from";
[@bs.send] external values: t('a) => t('a) = "";
