use crate::MenuItem;

pub trait SelectValue: Sized + Copy + PartialEq {
    fn next(&self) -> Self;
    fn name(&self) -> &'static str;
}

impl SelectValue for bool {
    fn next(&self) -> Self {
        !*self
    }

    fn name(&self) -> &'static str {
        match *self {
            // true => "O",
            // false => "O\r+\r#", // this only works for certain small fonts, unfortunately
            false => "[ ]",
            true => "[X]",
        }
    }
}

pub struct Select<'a, R, S: SelectValue> {
    title_text: &'a str,
    details: &'a str,
    convert: fn(S) -> R,
    value: S,
}

impl<'a, S: SelectValue> Select<'a, (), S> {
    pub fn new(title: &'a str, value: S) -> Self {
        Self {
            title_text: title,
            value,
            convert: |_| (),
            details: "",
        }
    }
}

impl<'a, R, S: SelectValue> Select<'a, R, S> {
    pub fn with_value_converter<R2: Copy>(self, convert: fn(S) -> R2) -> Select<'a, R2, S> {
        Select {
            convert,
            title_text: self.title_text,
            value: self.value,
            details: self.details,
        }
    }

    pub fn with_detail_text(self, details: &'a str) -> Self {
        Self { details, ..self }
    }
}

impl<'a, R, S: SelectValue> MenuItem for Select<'a, R, S> {
    type Data = R;

    fn interact(&mut self) -> R {
        self.value = self.value.next();
        (self.convert)(self.value)
    }

    fn title(&self) -> &str {
        self.title_text
    }

    fn details(&self) -> &str {
        self.details
    }

    fn value(&self) -> &str {
        self.value.name()
    }

    fn longest_value_str(&self) -> &str {
        let initial = self.value;
        let mut longest_str = initial.name();

        let mut current = initial.next();
        while current != initial {
            if current.name().len() > longest_str.len() {
                longest_str = current.name();
            }
            current = current.next();
        }

        longest_str
    }
}
