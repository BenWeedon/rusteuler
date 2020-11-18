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
    let syntax = format!("{}({});", input, numbers_string);
    TokenStream::from_str(&syntax).unwrap()
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
