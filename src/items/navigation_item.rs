use crate::MenuItem;

pub struct NavigationItem<'a, R: Copy> {
    title_text: &'a str,
    details: &'a str,
    return_value: R,
    marker: &'a str,
}

impl<'a, R: Copy> MenuItem for NavigationItem<'a, R> {
    type Data = R;

    fn interact(&mut self) -> R {
        self.return_value
    }

    fn title(&self) -> &str {
        self.title_text
    }

    fn details(&self) -> &str {
        self.details
    }

    fn value(&self) -> &str {
        self.marker
    }

    fn longest_value_str(&self) -> &str {
        self.value()
    }
}

impl<'a, R: Copy> NavigationItem<'a, R> {
    pub fn new(title: &'a str, value: R) -> Self {
        Self {
            title_text: title,
            return_value: value,
            details: "",
            marker: "",
        }
    }

    pub fn with_marker(self, marker: &'a str) -> Self {
        Self { marker, ..self }
    }

    pub fn with_detail_text(self, details: &'a str) -> Self {
        Self { details, ..self }
    }
}
