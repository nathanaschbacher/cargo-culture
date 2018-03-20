extern crate cargo_culture;

use cargo_culture::*;

use std::path::PathBuf;

#[test]
fn cargo_culture_project_should_pass_its_own_scrutiny() {
    // TODO - in order to prevent cargo deadlock, consider
    // copying this project into a temp dir and checking against that
    // ALT: Use an env-var to limit recursion

    match ::std::env::var("CARGO_CULTURE_TEST_RECURSION_BUSTER") {
        Ok(_) => {
            println!("Don't recurse infinitely.")
        },
        Err(_) => {
            println!("About to dogfood self with a check_culture");
            let mut v = Vec::new();
            let outcome = check_culture(&Opt {
                manifest_path: PathBuf::from("./Cargo.toml"),
                verbose: false,
            }, &mut v);

            assert_eq!(RuleOutcome::Success, outcome);
        },
    }
}