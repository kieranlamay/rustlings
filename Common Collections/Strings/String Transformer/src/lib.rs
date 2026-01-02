pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod transformer {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = vec![];

        for (s, c) in &input {
            match c {
                Command::Uppercase => output.push(s.to_uppercase()),
                Command::Trim => output.push(s.trim().to_string()),
                Command::Append(n) => {
                    // let mut bars : String = String::new();
                    let mut bar_string : String = s.clone();
                    for _ in 0..*n {
                        bar_string = format!("{}{}", bar_string, "bar");
                    }
                    output.push(bar_string)
                }
            }
        }
        output
    }
}