use crate::args::ProcessedArgs;

pub fn process_line<'a, 'b>(args: &'b ProcessedArgs, s: &'a str) -> &'a str {
    args.delimiter.split(s).nth(args.field - 1).unwrap_or("")
}

#[cfg(test)]
mod tests {
    use crate::args::ProcessedArgs;
    use crate::split::process_line;
    use anyhow::Result;
    use regex::Regex;

    #[test]
    fn test_delimiters() -> Result<()> {
        let cases = vec![
            (r"\s+", "1 2 3 4", "2"),
            (r"\s+", "1    2    3    4   ", "2"),
            (r",\s+", "1, 2, 3, 4", "2"),
            (r",", "1,2,3,4", "2"),
            (r"\s+", "1234", ""),
        ];
        let mut buf: String;
        for (d, input, expected) in cases {
            let args = ProcessedArgs {
                delimiter: Regex::new(d)?,
                field: 2,
                col_name: None,
            };
            buf = String::from(input);
            let actual = process_line(&args, &mut buf).as_bytes();
            assert_eq!(actual, expected.as_bytes());
        }
        Ok(())
    }
}