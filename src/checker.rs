use context::*;

pub struct EcicChecker {
    context: EcicContext
}

impl EcicChecker {
    pub fn new() -> Self {
        EcicChecker {
            context: EcicContext::new()
        }
    }

    pub fn init_from_ctx(ctx: EcicContext) -> Self {
        EcicChecker {
            context: ctx.clone()
        }
    }
/* Deserialize, do checks, fix(later), reserialize code and return 
    pub fn fire() -> Vec<u8> {

    }
*/
}
