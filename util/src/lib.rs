use std::{fmt::Debug, str::{FromStr, Split}};

pub trait ExpectParse<T> {
    fn e_parse(&self) -> T;
}

impl<T> ExpectParse<T> for str where T: FromStr, T::Err: Debug {
    fn e_parse(&self) -> T {
        self.parse::<T>().expect("Cannot parse")
    }
}

pub trait ExpectNext<T> {
    fn e_next(&mut self) -> T;
}

impl<'a> ExpectNext<&'a str> for Split<'a, char> {
    fn e_next(&mut self) -> &'a str {
        self.next().expect("Expected value")
    }
}

pub trait ExpectParseNext<T> {
    fn e_parse_next(&mut self) -> T;
}

impl<'a, T> ExpectParseNext<T> for Split<'a, char> where T: FromStr, T::Err: Debug {
    fn e_parse_next(&mut self) -> T {
        self.e_next().e_parse()
    }
}