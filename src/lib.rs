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

    pub fn get_left_right_ratio(&self) -> f32 {
        if self.is_leaf() {
            return std::f32::NAN;
        }

        let children_lengths = self.get_children_lengths();
        if children_lengths.1 > 0 {
            children_lengths.0 as f32 / children_lengths.1 as f32
        } else {
            std::f32::NAN
        }
    }

    pub fn get_right_left_ratio(&self) -> f32 {
        if self.is_leaf() {
            return std::f32::NAN;
        }

        let children_lengths = self.get_children_lengths();
        if children_lengths.0 > 0 {
            children_lengths.1 as f32 / children_lengths.0 as f32
        } else {
            std::f32::NAN
        }
    }

    pub fn build_tree(&mut self) {
        if self.is_leaf() {
            return;
        }

        self.fragment = Some(format!(
            "{}{}",
            (*self.left.take().unwrap()).to_string(),
            (*self.right.take().unwrap()).to_string(),
        ));

        self.adjust();
    }

    pub fn balance(&mut self) -> Result<(), ()> {
        if self.is_leaf() {
            return Err(());
        }

        let ratios = (
            self.get_left_right_ratio(),
            self.get_right_left_ratio()
        );

        if ratios.0.is_nan()
            || ratios.0 >= REBALANCE_RATIO
            || ratios.1.is_nan()
            || ratios.1 >= REBALANCE_RATIO
        {
            self.build_tree();
        } else {
            if let Some(ref mut child) = self.left {
                child.balance()?;
            } else {
                return Err(());
            }

            if let Some(ref mut child) = self.right {
                child.balance()?;
            } else {
                return Err(());
            }
        }

        Ok(())
    }

    pub fn to_string(&self) -> String {
        if self.is_leaf() {
            if let Some(fragment) = &self.fragment {
                String::from(&fragment[..])
            } else {
                String::new()
            }
        } else {
            let left = if let Some(ref child) = self.left {
                child.to_string()
            } else {
                String::new()
            };
            let right = if let Some(ref child) = self.right {
                child.to_string()
            } else {
                String::new()
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

    fn get_children_lengths(&self) -> (usize, usize) {
        (
            if let Some(ref child) = self.left {
                child.length
            } else {
                0
            },
            if let Some(ref child) = self.right {
                child.length
            } else {
                0
            }
        )
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
