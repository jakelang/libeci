use checklist::EciChecklist;
use parity_wasm::elements::{Module, deserialize_buffer};

#[derive(Clone)]
///Structure initialized with inputted WASM code and a checklist struct.
pub struct EcicChecker {
    code: Vec<u8>,
    checks: EciChecklist
}

impl EcicChecker {
    /*
     * Basic context constructors
     */
/// Constructs an empty checker struct with no code and no checks.
    pub fn new() -> Self {
        EcicChecker::empty()
    }

/// Constructs an empty checker struct with no code and no checks.
    pub fn empty() -> Self {
        EcicChecker {
            code: Vec::new(),
            checks: EciChecklist::new()
        }
    }

/// Initializes with inputted code and the default ECI checks.
    pub fn default(input: &Vec<u8>) -> Self {
        EcicChecker {
            code: input.clone(),
            checks: EciChecklist::default()
        }
    }

/// Returns the length of the WASM bytecode.
    pub fn code_len(&self) -> usize {
        self.code.len()
    }

/// Deserializes the WASM code and executes all checks in the checklist.
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
    fn test_main_export() {
        let wasm: Vec<u8> = vec!(0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01, 0x60,
            0x00, 0x00, 0x03, 0x02, 0x01, 0x00, 0x07, 0x08, 0x01, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x00, 0x00, 
            0x0a, 0x04, 0x01, 0x02, 0x00, 0x0b);
        let mut checker = EcicChecker::default(&wasm);
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Unknown);
        checker.fire();
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Good);
    }

    #[test]
    fn test_main_export_malformed_with_param() {
        let wasm: Vec<u8> = vec!(  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x05, 0x01, 0x60,
            0x01, 0x7f, 0x00, 0x03, 0x02, 0x01, 0x00, 0x07, 0x08, 0x01, 0x04, 0x6d,
            0x61, 0x69, 0x6e, 0x00, 0x00, 0x0a, 0x04, 0x01, 0x02, 0x00, 0x0b);
        let mut checker = EcicChecker::default(&wasm);
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Unknown);
        checker.fire();
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Malformed);
    }

    #[test]
    fn test_main_export_nonexistent() {
        let wasm: Vec<u8> = vec!(0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00);
        let mut checker = EcicChecker::default(&wasm);
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Unknown);
        checker.fire();
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Nonexistent);
    }

    #[test]
    fn test_main_export_is_a_memory() {
        let wasm: Vec<u8> = vec!(0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x05, 0x03, 0x01, 0x00,
            0x00, 0x07, 0x08, 0x01, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x02, 0x00);
        let mut checker = EcicChecker::default(&wasm);
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Unknown);
        checker.fire();
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Malformed);
    }

    #[test]
    fn test_main_export_malformed_with_return() {
        let wasm: Vec<u8> = vec!(0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x05, 0x01, 0x60,
            0x00, 0x01, 0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x08, 0x01, 0x04, 0x6d,
            0x61, 0x69, 0x6e, 0x00, 0x00, 0x0a, 0x06, 0x01, 0x04, 0x00, 0x41, 0x00, 0x0b);
        let mut checker = EcicChecker::default(&wasm);
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Unknown);
        checker.fire();
        assert_eq!(checker.checks.get_check_status("export-main"), CheckStatus::Malformed);
    }

    #[test]
    fn test_memory_export() {
        let wasm: Vec<u8> = vec!(0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x05, 0x03, 0x01, 0x00,
            0x00, 0x07, 0x0a, 0x01, 0x06, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x02, 0x00);
        let mut checker = EcicChecker::default(&wasm);
        assert_eq!(checker.checks.get_check_status("export-memory"), CheckStatus::Unknown);
        checker.fire();
        assert_eq!(checker.checks.get_check_status("export-memory"), CheckStatus::Good);
    }

    #[test]
    fn test_memory_export_malformed() {
        let wasm: Vec<u8> = vec!(  0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01, 0x60,
            0x00, 0x00, 0x03, 0x02, 0x01, 0x00, 0x07, 0x0a, 0x01, 0x06, 0x6d, 0x65,
            0x6d, 0x6f, 0x72, 0x79, 0x00, 0x00, 0x0a, 0x04, 0x01, 0x02, 0x00, 0x0b);
        let mut checker = EcicChecker::default(&wasm);
        assert_eq!(checker.checks.get_check_status("export-memory"), CheckStatus::Unknown);
        checker.fire();
        assert_eq!(checker.checks.get_check_status("export-memory"), CheckStatus::Malformed);
    }

    #[test]
    fn test_memory_export_nonexistent() {
        let wasm: Vec<u8> = vec!(0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01, 0x60,
            0x00, 0x00, 0x03, 0x02, 0x01, 0x00, 0x07, 0x08, 0x01, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x00, 0x00, 
            0x0a, 0x04, 0x01, 0x02, 0x00, 0x0b);
        let mut checker = EcicChecker::default(&wasm);
        assert_eq!(checker.checks.get_check_status("export-memory"), CheckStatus::Unknown);
        checker.fire();
        assert_eq!(checker.checks.get_check_status("export-memory"), CheckStatus::Nonexistent);
    }
}
