use super::correct_args;


#[test]
fn test_correct_arguments() {
    let args_to_correct = vec![
        "pause",
        ". /vault/blah",
        ". /vault/blah2",
        "echo $TEST_1",
        "echo $TEST_2"
    ].into_iter().map(String::from).collect();

    let expected = "pause; . /vault/blah; . /vault/blah2; echo $TEST_1; echo $TEST_2;";

    let actual = correct_args(args_to_correct).expect("failed to correct args");

    assert_eq!(expected.to_string(), actual);
}