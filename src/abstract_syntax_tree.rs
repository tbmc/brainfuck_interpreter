pub struct Ast {
    pub char: char,
    pub sub_asts: Option<Vec<Ast>>,
}

impl Ast {
    pub fn new() -> Ast {
        return Ast {
            char: 0 as char,
            sub_asts: Some(Vec::new()),
        };
    }

    pub fn add_new_leaf(&mut self, value: char) -> Result<(), String> {
        let mut leaf = Ast::new();
        leaf.char = value;
        let mut sub = self.sub_asts.take().unwrap();
        let return_value = match self.sub_asts {
            Some(_) => {
                sub.push(leaf);
                Ok(())
            }
            None => Err("Cannot add to a leaf".to_string())
        };

        self.sub_asts = Some(sub);

        return_value
    }

    pub fn add_new_branch(&mut self, value: char) -> Result<usize, String> {
        let mut branch = Ast::new();
        branch.sub_asts = Some(Vec::new());
        branch.char = value;
        let mut sub = self.sub_asts.take().unwrap();
        let return_value = match self.sub_asts {
            Some(_) => {
                sub.push(branch);
                // let len = sub.len();
                // let v = sub.get_mut(len - 1).unwrap();
                Ok(sub.len() - 1)
            }
            None => Err("Cannot add to a leaf".to_string())
        };

        self.sub_asts = Some(sub);

        return_value
    }
}
