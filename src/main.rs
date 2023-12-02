use std::env;

use day::Day;
use easybench::bench;

use crate::day::InputType;

pub mod day;
pub mod solutions;

fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    if let [_, day] = &args[..] {
        let day: Day = day
            .to_owned()
            .try_into()
            .expect(format!("missing day {}", day).as_str());
        tracing::info!("Reading inputs for day {:?}", day);

        let solution = solutions::get_solution(&day);
        let inputs = day.get_inputs().expect("failed to read inputs for day");
        for i in inputs {
            tracing::info!("Running input: {:?}", i.path);
            let actual = solution.run(&i.contents).expect("failed to run solution");
            if i.input_type == InputType::Test {
                let expected = i.get_expected_output().expect("could not read expected output");
                if expected != actual {
                    panic!("test {:?} got {}, expected {}", &i, actual, expected)
                } else {
                    tracing::info!("Test {:?} passed! Got {}, expected {}", &i.path, actual, expected);
                }
            } else {
                tracing::info!("Input {:?} got:\n{}", &i.path, actual);

                tracing::info!("Running benchmarks:");
                println!("{}", bench(|| solution.run(&i.contents)));
            }
        }
    }
}
