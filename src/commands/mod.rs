mod dico;

use crate::parser::Context;

trait CommandPattern {
    fn from_context(ctx: Context);
    fn execute(self: Self);
}