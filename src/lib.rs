const SPLIT_LENGTH: usize = 1000;
const JOIN_LENGTH: usize = 500;
const REBALANCE_RATIO: f32 = 1.2;

pub struct Rope {
    pub length: usize,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    fragment: Option<String>,
}

impl Rope {
    pub fn new(raw: &str) -> Self {
        let mut rope = Rope {
            length: raw.len(),
            fragment: Some(String::from(raw)),
            left: None,
            right: None,
        };
        rope.adjust();

        rope
    }

    pub fn is_leaf(&self) -> bool {
        self.fragment.is_some()
    }

    pub fn to_string(&self) -> String {
        if self.is_leaf() {
            if let Some(fragment) = &self.fragment {
                String::from(&fragment[..])
            } else {
                String::new()
            }
        } else {
            let left = match &self.left {
                Some(child) => (*child).to_string(),
                None => String::new(),
            };
            let right = match &self.right {
                Some(child) => (*child).to_string(),
                None => String::new(),
            };

            String::from(format!("{}{}", left, right))
        }
    }

    fn adjust(&mut self) {
        if self.is_leaf() {
            let raw = self.fragment.take().unwrap();
            let half_length = self.length >> 2;

            if half_length >= SPLIT_LENGTH {
                self.left = Some(Box::new(Rope::new(&raw[..half_length])));
                self.right = Some(Box::new(Rope::new(&raw[half_length..])));
            } else {
                self.fragment = Some(raw);
            }
        } else {
            if self.length <= JOIN_LENGTH {
                self.fragment = Some(format!(
                    "{}{}",
                    (*self.left.take().unwrap()).to_string(),
                    (*self.right.take().unwrap()).to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_allocates_a_new_rope() {
        let rope = Rope::new("hello world");

        assert!(rope.is_leaf());
    }

    #[test]
    fn it_converts_a_rope_into_a_string() {
        let rope = Rope::new("hello world");

        assert_eq!(rope.to_string(), "hello world");
    }
}
