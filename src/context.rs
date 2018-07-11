use checklist::EciChecklist;

pub struct EcicContext {
    code: Vec<u8>,
    checks: EciChecklist
}

impl EcicContext {
    /*
     * Basic context constructors
     */
    pub fn new() -> Self {
        EcicContext::empty()
    }

    pub fn empty() -> Self {
        EcicContext {
            code: Vec::new(),
            checks: EciChecklist::new()
        }
    }

    pub fn default(input: &Vec<u8>) -> Self {
        EcicContext {
            code: input.clone(),
            checks: EciChecklist::default()
        }
    }

    pub fn code_len(&self) -> usize {
        self.code.len()
    }
}

#[cfg(test)]
mod tests {
    use context::EcicContext;

    #[test]
    fn empty_code() {
        let ctx = EcicContext::new();
        assert!(ctx.code.is_empty());
    }
    
    #[test]
    fn some_code() {
        let wasm = vec!(0x00, 0x77, 0x61, 0x73, 0x6d);
        let ctx = EcicContext::default(&wasm);
        assert_eq!(5, wasm.len());
        assert_eq!(5, ctx.code_len());
    }
}
