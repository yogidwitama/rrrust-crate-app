use say_hello_jan::{say_hello, say_goodbye, say_goodbye_everyone, say_hello_to_everyone};

fn main() {
    hello::say_hello();
    bye::say_bye();
    let response= say_hello("Yogi");
    println!("{}", response);

    let responsesayhelloevery=say_hello_to_everyone();
    println!("{}", responsesayhelloevery);

    let responsebye=say_goodbye("Dwitama");
    println!("{}", responsebye);

    let responsebyeeveryone =say_goodbye_everyone();
    println!("{}", responsebyeeveryone)
}
#[test]
fn test_uuid() {
    let id =uuid::Uuid::new_v4().to_string();
    println!("{}", id);
}