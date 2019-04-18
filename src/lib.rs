pub struct Rope {
    pub length: usize,
    fragment: Option<String>
}

impl Rope {
    pub fn new(raw: &str) -> Self {
        Rope {
            length: raw.len(),
            fragment: Some(String::from(raw)),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.fragment.is_some()
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
}
