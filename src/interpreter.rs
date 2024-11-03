use crate::context::Context;
use crate::Error;

#[derive(Debug, Default)]
pub struct Interpreter {}

#[derive(Debug, Default)]
pub struct Execution {
    context: Context,
}

impl Interpreter {
    pub fn execute(self, context: Context) -> Result<Execution, Error> {
        Ok(Execution { context })
    }
}
