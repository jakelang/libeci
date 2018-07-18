use checklist::EciChecklist;
use parity_wasm::elements::{Module, Deserialize, deserialize_buffer};

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

    pub fn fire(&mut self) {
        let module = deserialize_buffer::<Module>(&mut self.code).unwrap();
        
        let check_ids: Vec<String> = self.checks.checklist
            .keys()
            .map(|x| x.clone())
            .collect();

        for check in check_ids {
            let checkresult = self.checks.get_checker(check.as_str())(&module);
            self.checks.set_check_status(check.as_str(), checkresult);
        }
    }
}

#[cfg(test)]
mod tests {
    use checker::EcicChecker;
    use checklist::CheckStatus;

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

    #[test]
    fn test_checking() {
        let wasm: Vec<u8> = vec!(0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00);
        let mut checker = EcicChecker::default(&wasm);
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Unknown);
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Unknown);
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Unknown);
        checker.fire();
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Good);
        assert_eq!(checker.checks.get_check_status("export-memory"), CheckStatus::Good);
        assert_eq!(checker.checks.get_check_status("eei-imports"), CheckStatus::Good);
    }
}
