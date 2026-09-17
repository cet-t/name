use std::{borrow::Cow, fmt::Write};

use strfmt::{FmtError, Formatter, strfmt_map};

pub enum NameStyle<'a> {
    Raw,
    Template(std::borrow::Cow<'a, str>),
}

impl<'a> NameStyle<'a> {
    pub fn template<'b, S>(s: S) -> Self
    where
        S: Into<std::borrow::Cow<'a, str>>,
    {
        Self::Template(s.into())
    }
}

pub struct User<'a> {
    name: &'a str,
}

impl<'a> User<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn name<'b>(
        &self,
        style: NameStyle<'b>,
    ) -> Result<std::borrow::Cow<'a, str>, Box<dyn std::error::Error>> {
        let s = match style {
            NameStyle::Raw => Cow::Borrowed(self.name),
            NameStyle::Template(fmt) => {
                let format = |f: &mut Formatter| {
                    write!(f, "{}", self.name).map_err(|e| FmtError::Invalid(e.to_string()))
                };
                Cow::Owned(strfmt_map(&fmt, |mut f| format(&mut f))?)
            }
        };
        Ok(s)
    }
}
