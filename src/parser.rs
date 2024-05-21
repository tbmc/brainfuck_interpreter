use crate::abstract_syntax_tree::Ast;

pub fn parse_code(code: &str) -> Result<Ast, String> {
    let mut root = Ast::new();
    let mut parent_node = &mut root;

    for c in code.chars() {
        match c {
            '>' | '<' | '+' | '-' | '.' | ',' => {
                let result_add = parent_node.add_new_leaf(c);
                match result_add {
                    Err(e) => {
                        return Err(e);
                    }
                    _ => {}
                }
            },
            '[' =>  {
                let result = parent_node.add_new_branch(c);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(index) => {
                        parent_node = parent_node.sub_asts.as_mut().unwrap().get_mut(index).unwrap();
                    }
                }
            },
            ']' => {
                
            },
            _ => {
                // Char is a comment, so it is ignored
            },
        }
    }
    
    return Ok(root);
}
