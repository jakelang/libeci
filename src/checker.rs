use checklist::EciChecklist;

#[derive(Clone)]
pub struct EcicChecker {
    code: Vec<u8>,
    checks: EciChecklist
}

impl EcicChecker {
    /*
     * Basic context constructors
     */
    pub fn new() -> Self {
        EcicChecker::empty()
    }

    pub fn empty() -> Self {
        EcicChecker {
            code: Vec::new(),
            checks: EciChecklist::new()
        }
    }

    pub fn default(input: &Vec<u8>) -> Self {
        EcicChecker {
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
    use checker::EcicChecker;

    #[test]
    fn empty_code() {
        let ctx = EcicChecker::new();
        assert!(ctx.code.is_empty());
    }
    
    #[test]
    fn some_code() {
        let wasm = vec!(0x00, 0x77, 0x61, 0x73, 0x6d);
        let ctx = EcicChecker::default(&wasm);
        assert_eq!(5, wasm.len());
        assert_eq!(5, ctx.code_len());
    }
}
