use log::info;
use minigrep::{init, search};

#[test]
fn fn_one_result_test01() {
    init();
    
    let query = "duct";
    let contents = "\
Result: 
safe, fast, productive.
Pick three.";

    let result = search(query, contents);
    info!("{:?}",result);

    // assert_eq!(
    //     vec!["safe, fast, productive."],
    //     search(query, contents)
    // );
}

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents)
    );
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}