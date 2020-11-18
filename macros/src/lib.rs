use {
    proc_macro::TokenStream,
    std::{error, fs, str::FromStr},
};

#[proc_macro]
pub fn get_problem_numbers(input: TokenStream) -> TokenStream {
    let numbers = get_problem_numbers_fn("src").unwrap();
    let numbers_string = numbers
        .into_iter()
        .map(|n| format!("{}", n))
        .collect::<Vec<String>>()
        .join(", ");
    let input_pattern = format!("{}", input);
    let input_matched = input_pattern.replace("1", &numbers_string);
    TokenStream::from_str(&input_matched).unwrap()
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

fn get_problem_numbers_fn(dir: &str) -> Result<Vec<usize>, Box<dyn error::Error>> {
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
        let numbers = get_problem_numbers_fn("../src").unwrap();
        assert_ne!(numbers.len(), 0);

        let mut i = 1;
        for n in numbers {
            assert_eq!(n, i);
            i += 1;
        }
    }
}
