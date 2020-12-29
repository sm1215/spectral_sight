use spectral_sight::*;

#[test]
fn test_read_contents() {
    let testing_path = String::from("./tests/path_test");
    let result = read_contents(testing_path);
    println!("result {:#?}", result);
    // assert_eq!("sample", output);
}
