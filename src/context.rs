use eci::CheckStatus;

pub struct EcicContext {
    code: Vec<u8>,
    check_export_main: CheckStatus,
    check_export_mem: CheckStatus,
    check_imports: CheckStatus 
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
            check_export_main: CheckStatus::Unknown,
            check_export_mem: CheckStatus::Unknown,
            check_imports: CheckStatus::Unknown
        }
    }

    pub fn from_vec(input: &Vec<u8>) -> Self {
        EcicContext {
            code: input.clone(),
            check_export_main: CheckStatus::Unknown,
            check_export_mem: CheckStatus::Unknown,
            check_imports: CheckStatus::Unknown
        }
    }

    pub fn code_len(&self) -> usize {
        self.code.len()
    }
}

#[cfg(test)]
mod tests {
    use context::EcicContext;
    use eci::CheckStatus;

    #[test]
    fn empty_code() {
        let ctx = EcicContext::new();
        assert_eci_checks_initialized(&ctx);
        assert!(ctx.code.is_empty());
    }
    
    #[test]
    fn some_code() {
        let wasm = vec!(0x00, 0x77, 0x61, 0x73, 0x6d);
        let ctx = EcicContext::from_vec(&wasm);
        assert_eci_checks_initialized(&ctx);
        assert_eq!(5, wasm.len());
        assert_eq!(5, ctx.code_len());
    }

    fn assert_eci_checks_initialized(ctx: &EcicContext) {
        match ctx.check_export_main {
            CheckStatus::Unknown => (),
            _ => assert!(false)
        }

        match ctx.check_export_mem {
            CheckStatus::Unknown => (),
            _ => assert!(false)
        }

        match ctx.check_imports {
            CheckStatus::Unknown => (),
            _ => assert!(false)
        }
    }
}
