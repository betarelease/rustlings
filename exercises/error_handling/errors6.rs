// errors6.rs

// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here,
// we define a custom error type to make it possible for callers to decide
// what to do next when our function returns an error.

// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

use std::num::ParseIntError;

fn parse_pos_nonzero(s: &str) -> Result<i64, ParseIntError> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    match s.parse() {
        Ok(n) => Ok(n),
        Err(err) => Err(err),
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParseIntError))
        );
    }

    #[test]
    fn test_positive() {
        let x = parse_pos_nonzero("42");
        assert_eq!(42, x.ok().unwrap());
    }
}
