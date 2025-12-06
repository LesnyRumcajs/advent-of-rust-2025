use paste::paste;
macro_rules! solution_day {
    ($day:ident,$part1:expr,$part2:expr) => {
        paste! {
            #[test]
            fn [<solution_ $day>]() {
            use assert_cmd::prelude::*;
            use assert_cmd::cargo::cargo_bin_cmd;

            cargo_bin_cmd!(stringify!($day))
                .pipe_stdin(format!("inputs/{}/input.txt", stringify!($day)))
                .unwrap()
                .unwrap()
                .assert()
                .success()
                .stdout(concat!($part1, "\n", $part2, "\n"));
            }
        }
    };
}

solution_day!(day1, "1172", "6932");
solution_day!(day2, "38437576669", "49046150754");
solution_day!(day3, "17343", "172664333119298");
solution_day!(day4, "1419", "8739");
solution_day!(day5, "888", "344378119285354");
solution_day!(day6, "4309240495780", "0"); // TODO part 2
