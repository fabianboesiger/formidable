use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Name([Option<NamePart>; 16]);

impl Name {
    fn len(&self) -> usize {
        self.0.iter().take_while(|part| part.is_some()).count()
    }

    pub fn push(mut self, part: NamePart) -> Self {
        let len = self.len();
        if len < self.0.len() {
            self.0[len] = Some(part);
        } else {
            panic!("Name parts exceed maximum length");
        }
        self
    }

    pub fn push_index(self, index: usize) -> Self {
        self.push(NamePart::Index(index))
    }

    pub fn push_key(self, key: &'static str) -> Self {
        self.push(NamePart::Key(key))
    }
}

impl<T> From<T> for Name
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let mut name = Name::default();
        let mut curr = String::new();

        for c in value.as_ref().chars() {
            if c == '[' || c == ']' {
                if !curr.is_empty() {
                    if let Ok(index) = curr.parse::<usize>() {
                        name = name.push_index(index);
                    } else {
                        name = name.push_key(Box::leak(curr.clone().into_boxed_str()));
                    }
                    curr.clear();
                }
            } else {
                curr.push(c);
            }
        }
        if !curr.is_empty() {
            if let Ok(index) = curr.parse::<usize>() {
                name = name.push_index(index);
            } else {
                name = name.push_key(Box::leak(curr.clone().into_boxed_str()));
            }
        }

        name
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name({})", self)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self
            .0
            .iter()
            .filter_map(|&o| o)
            .enumerate()
            .map(|(i, part)| match part {
                NamePart::Index(i) => format!("[{}]", i),
                NamePart::Key(k) if i == 0 => k.to_string(),
                NamePart::Key(k) => format!("[{}]", k),
            })
            .collect::<Vec<_>>()
            .join("");

        write!(f, "{}", name)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NamePart {
    Index(usize),
    Key(&'static str),
}
