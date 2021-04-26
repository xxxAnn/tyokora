pub mod dico;

use crate::parser::Context;

trait CommandPattern {
    fn from_context(ctx: Context);
    fn execute(self: Self);
}

pub fn consume(ctx: Context) {
    if ctx.cmd == String::from("note") {
        dico::exec(ctx);
    }
}

