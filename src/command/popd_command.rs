use super::*;

use std::path::Path;

/// Popd command pops the top-most directory of the stack and changes CWD to it.
pub struct PopdCommand;

impl Command for PopdCommand {
    fn execute(&mut self, prompt: &mut Prompt) -> Result<bool, i32> {
        let path = prompt.context.borrow_mut().dir_stack.pop();
        if let Some(path) = &path {
            prompt.set_cwd(Path::new(&path));
            prompt.context.borrow().print_short_dir_stack();
        } else {
            println!("Directory stack is empty");
        }

        Ok(true)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CommandAliases for PopdCommand {
    fn aliases() -> Vec<String> {
        vec!["popd".to_string()]
    }
}
