fn main() {


    let mut hello1 = hello_with_exclamation();

    println!("{} is `{}`", "hello1", hello1);

    hello1.push_str("!");

    println!("{} is `{}`", "hello1", hello1);
}

fn hello_with_exclamation() -> String {
    let hello = String::from("Hello!");
    hello
}
