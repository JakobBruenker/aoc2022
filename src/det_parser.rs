use std::{str::Chars, iter::Peekable};

pub trait DetParseable<T = Self> {
    fn parse_det(input: &mut Peekable<Chars>) -> Option<T>;
}

impl DetParseable for char {
    fn parse_det(input: &mut Peekable<Chars>) -> Option<char> {
        input.next()
    }
}

fn char_if :: P

struct Digit(char);

impl DetParseable for Digit {
    fn parse_det(input: &mut Peekable<Chars>) -> Option<Self> {
        input.parse_det::<char>().filter(|c| c.is_numeric()).map(Digit)
    }
}

// impl DetParseable for u64 {
//     fn parse_det(input: &mut Chars)
// }

// Pretty bad parser
struct DetParser<'a, T> {
    parser: &'a dyn Fn(&mut Peekable<Chars>) -> Option<T>,
}

impl<'a, T: DetParseable> DetParser<'a, T> {
    fn parse(&self, input: &mut Peekable<Chars>) -> Option<T> {
        (self.parser)(input)
    }

    // remember, if this fails, the whole parser should fail, there is not possibility for alternatives here
    fn filter(&self, pred: &dyn Fn(&T) -> bool) -> Self {
        DetParser { parser: &|input| self.parse(input).filter(pred) }
    }

    fn map<B>(&self, f: &dyn Fn(T) -> B) -> DetParser<B> {
        DetParser { parser: &|input| self.parse(input).map(f) }
    }

    // this is part of why the parser is bad, we can only check the char, not the result
    fn whilst(&self, pred: &dyn Fn(&char) -> bool) -> DetParser<Vec<T>> {
        DetParser { parser: &|input| {
            let mut result = vec![];
            while input.peek().filter(|c| pred(c)).is_some() {
                result.push(self.parse(input))
            }
            result.into_iter().collect::<Option<_>>()
        }}
    }
}

pub trait DetParseInput: private::Sealed {
    fn parse_det<P: DetParseable>(&mut self) -> Option<P>;
}

impl<'a> DetParseInput for Peekable<Chars<'a>> {
    fn parse_det<P: DetParseable>(&mut self) -> Option<P> {
        P::parse_det(self)
    }
}

mod private {
    use std::{str::Chars, iter::Peekable};

    pub trait Sealed {}
    impl<'a> Sealed for Peekable<Chars<'a>> {}
}
