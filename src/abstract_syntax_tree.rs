pub struct AstNode {
    pub char: char,
    pub sub_assets_indexes: Vec<usize>,
    pub parent_index: Option<usize>,
    pub is_leaf: bool,
    pub index_in_string: usize,
}

pub struct Ast {
    pub data: Vec<AstNode>,
}

impl Ast {
    pub fn new() -> Self {
        let mut ast = Ast {
            data: Vec::new(),
        };
        let root = AstNode {
            char: 0 as char,
            sub_assets_indexes: Vec::new(),
            parent_index: None,
            is_leaf: false,
            index_in_string: 0,
        };
        ast.data.push(root);

        ast
    }

    pub fn add_new_leaf(&mut self, value: char, parent_index: usize, index_in_string: usize) -> usize {
        let leaf = AstNode::new(value, parent_index, true, index_in_string);
        let leaf_index = self.data.len();
        self.data.push(leaf);

        let ast_parent_node = match self.data.get_mut(parent_index) {
            None => {
                panic!("This should never happen. Parent index out of range {}", parent_index);
            }
            Some(parent) => parent,
        };

        ast_parent_node.sub_assets_indexes.push(leaf_index);

        leaf_index
    }

    pub fn add_new_branch(&mut self, value: char, parent_index: usize, index_in_string: usize) -> usize {
        let branch = AstNode::new(value, parent_index, false, index_in_string);
        let branch_index = self.data.len();
        self.data.push(branch);

        let ast_parent_node = match self.data.get_mut(parent_index) {
            None => {
                panic!("This should never happen. Parent index out of range {}", parent_index);
            }
            Some(parent) => parent,
        };

        ast_parent_node.sub_assets_indexes.push(branch_index);

        branch_index
    }

    pub fn close_branch(&mut self, value: char, parent_index: usize, index_in_string: usize) -> usize {
        let grand_parent_index = match self.data.get(parent_index).unwrap().parent_index {
            None => {
                panic!("This should never happen. Parent index out of range {}", parent_index);
            }
            Some(parent_index) => parent_index,
        };

        self.add_new_leaf(value, grand_parent_index, index_in_string);
        // let mut grand_parent_node = self.data.get_mut(grand_parent_index).unwrap();
        // self.add_new_leaf(value, grand_parent_index);
        grand_parent_index
    }
}

impl AstNode {
    pub fn new(value: char, parent_index: usize, is_leaf: bool, index_in_string: usize) -> Self {
        return AstNode {
            char: value,
            sub_assets_indexes: Vec::new(),
            parent_index: Some(parent_index),
            is_leaf,
            index_in_string,
        };
    }
}

pub fn parse_code(code: &str) -> Result<Ast, String> {
    let mut ast = Ast::new();
    let mut parent_index = 0;

    for (i, c) in code.chars().enumerate() {
        match c {
            '>' | '<' | '+' | '-' | '.' | ',' => {
                // let parent_node = ast.data.get_mut(parent_index).unwrap();
                ast.add_new_leaf(c, parent_index, i);
            }
            '[' => {
                // let parent_node = ast.data.get_mut(parent_index).unwrap();
                // parent_index = parent_node.add_new_branch(c);
                parent_index = ast.add_new_branch(c, parent_index, i);
            }
            ']' => {
                parent_index = ast.close_branch(c, parent_index, i);
            }
            _ => {
                // Char is a comment, so it is ignored
            }
        }
    }

    Ok(ast)
}
