use {
    proc_macro::TokenStream,
    std::{error, fs, str::FromStr},
};

#[proc_macro]
pub fn declare_problem_mods(input: TokenStream) -> TokenStream {
    check_input_empty(input);

    let numbers = get_problem_numbers("src").unwrap();
    let output = numbers
        .into_iter()
        .map(|n| format!("mod problem_{};", n))
        .collect::<Vec<String>>()
        .join("\n");
    TokenStream::from_str(&output).unwrap()
}

#[proc_macro]
pub fn match_problems(input: TokenStream) -> TokenStream {
    check_input_empty(input);

    const MATCH_STR: &str = r#"
        match problem_number {
            <>
            _ => Err(format!("{} is not a valid problem number.", problem_number)),
        }
    "#;

    let numbers = get_problem_numbers("src").unwrap();
    let match_cases = numbers
        .into_iter()
        .map(|n| format!("{n} => problem_{n}::run(),", n = n))
        .collect::<Vec<String>>()
        .join("\n");

    let output = MATCH_STR.replace("<>", &match_cases);
    TokenStream::from_str(&output).unwrap()
}

#[proc_macro_attribute]
pub fn answer(attrs: TokenStream, input: TokenStream) -> TokenStream {
    const TEST_STR: &str = r#"
        #[cfg(test)]
        mod tests {
            use super::run;
            #[test]
            fn it_works() {
                assert_eq!(run().unwrap(), <>);
            }
        }
    "#;

    let answer = format!("{}", attrs);
    let test_output = TEST_STR.replace("<>", &answer);

    let full_output = format!("{}", input) + &test_output;
    TokenStream::from_str(&full_output).unwrap()
}

fn check_input_empty(input: TokenStream) {
    if !input.is_empty() {
        panic!("This macro does not take any input");
    }
}

fn get_problem_numbers(dir: &str) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let mut numbers = vec![];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_str().unwrap();
        if name.starts_with("problem_") {
            let number_string = &name[8..name.len() - 3];
            let number = number_string.parse::<usize>()?;
            numbers.push(number);
        }
    }

    numbers.sort_unstable();
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_problem_numbers_test() {
        let numbers = get_problem_numbers("../src").unwrap();
        assert_ne!(numbers.len(), 0);

        let mut i = 1;
        for n in numbers {
            assert_eq!(n, i);
            i += 1;
        }
    }
}
