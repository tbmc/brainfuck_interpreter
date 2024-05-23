use crate::abstract_syntax_tree::Ast;

fn internal_syntax_check(ast: &Ast, parent_index: usize) -> Result<(), String> {
    let parent_node = ast.data.get(parent_index).unwrap();
    
    if parent_node.sub_assets_indexes.is_empty() {
        return Err("Error, loop empty. It creates an infinite loop.".to_string());
    }

    let mut sub_index = 0;
    while sub_index < parent_node.sub_assets_indexes.len() {
        let sub_node_index = parent_node.sub_assets_indexes.get(sub_index).unwrap();
        let node = ast.data.get(*sub_node_index).unwrap();

        if node.is_leaf {
            sub_index += 1;
            continue;
        }

        if node.char != '[' {
            let formatted = format!("Invalid branch at {}. This should not happen", node.index_in_string);
            return Err(formatted);
        }
        
        internal_syntax_check(ast, *sub_node_index)?;

        sub_index += 1;
        match parent_node.sub_assets_indexes.get(sub_index) {
            None => {
                let formatted = format!("Invalid branch closing at {}. No closing found (no node after this one).", node.index_in_string);
                return Err(formatted);
            }
            Some(next_node_index) => {
                let next_node = ast.data.get(*next_node_index).unwrap();
                if next_node.char != ']' {
                    let formatted = format!("Invalid branch closing at {}. No closing found.", node.index_in_string);
                    return Err(formatted);
                }
            }
        };

        sub_index += 1;
    }

    Ok(())
}

pub fn syntax_check(ast: &Ast) -> Result<(), String> {
    internal_syntax_check(ast, 0)
}
