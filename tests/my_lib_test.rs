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