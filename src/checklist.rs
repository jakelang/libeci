use std::collections::HashMap;
use parity_wasm::elements::Module;
use eci_std;

/// Enum describing the state of a check. Always initialized as "Unknown."
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CheckStatus {
    Unknown,
    Nonexistent,
    Malformed,
    Good,
}

/// Struct containing data about the status of a check, and a function pointer implements the check.
#[derive(Clone)]
pub struct Check {
    status: CheckStatus,
    do_check: fn(&Module) -> CheckStatus,
}

/// Checklist structure containing a string-to-check map.
#[derive(Clone)]
pub struct EciChecklist {
    pub checklist: HashMap<String, Check>
}

impl EciChecklist {
/// Constructs an empty checklist.
    pub fn new() -> Self {
        EciChecklist {
            checklist: HashMap::new()
        }
    }

/// Constructs a checklist with the standard ewasm ECI checks.
    pub fn default() -> Self {
        let checks: HashMap<String, Check> = 
            [("export-main".to_string(), Check { status: CheckStatus::Unknown, do_check: eci_std::chk_main_exported }),
             ("export-memory".to_string(), Check { status: CheckStatus::Unknown, do_check: eci_std::chk_mem_exported }),
             ("eei-imports".to_string(), Check { status: CheckStatus::Unknown, do_check: eci_std::chk_eei_imported }),
             ("no-startfn".to_string(), Check { status: CheckStatus::Unknown, do_check: eci_std::chk_no_startfn })]
            .iter().cloned().collect();

        EciChecklist {
            checklist: checks
        }
    }

/// Adds a check with the given ID and function implementing said check.
    pub fn add_check(&mut self, key: &str, checkfn: fn(&Module) -> CheckStatus) {
        self.checklist.insert(key.to_string(), Check { status: CheckStatus::Unknown, do_check: checkfn });
    }

/// Sets the status of a check.
    pub fn set_check_status(&mut self, key: &str, val: CheckStatus) {
        self.checklist.get_mut(&key.to_string()).unwrap().status = val;
    }

/// Returns the status of a check.
    pub fn get_check_status(&self, key: &str) -> CheckStatus {
        self.checklist[&key.to_string()].status.clone()
    }

/// Returns a simple boolean value describing whether a check is good or not.
    pub fn check_is_good(&self, key: &str) -> bool {
        match self.get_check_status(key) {
            CheckStatus::Good => true,
            _ => false
        }
    }

/// Returns a function pointer to the implementation of a check.
    pub fn get_checker(&self, key: &str) -> fn(&Module) -> CheckStatus {
        self.checklist[&key.to_string()].do_check
    }
}

#[cfg(test)]
mod tests {
    use checklist::*;

    #[test]
    fn empty_checklist() {
        let checks = EciChecklist::new();
        assert!(checks.checklist.is_empty());
    }

    #[test]
    fn default_checks() {
        let checks = EciChecklist::default();
        assert!(checks.checklist.contains_key(&"export-main".to_string()));
        assert!(checks.checklist.contains_key(&"export-memory".to_string()));
        assert!(checks.checklist.contains_key(&"eei-imports".to_string()));
    }

    #[test]
    fn insert_arbitrary_check() {
        let mut checks = EciChecklist::new();
        checks.add_check("random-arbitrary-check", eci_std::chk_main_exported);
        assert!(checks.checklist.contains_key(&"random-arbitrary-check".to_string()));
    }

    #[test]
    fn verify_check() {
        let mut checks = EciChecklist::new();
        checks.add_check("foobar", eci_std::chk_main_exported);
        checks.set_check_status("foobar", CheckStatus::Good);
        assert!(checks.check_is_good("foobar"));
    }

    #[test]
    fn test_check_eq() {
        let mut checks = EciChecklist::new();
        checks.add_check("foobar", eci_std::chk_main_exported);
        checks.set_check_status("foobar", CheckStatus::Nonexistent);
        assert_eq!(checks.get_check_status("foobar"), CheckStatus::Nonexistent);
        checks.set_check_status("foobar", CheckStatus::Malformed);
        assert_eq!(checks.get_check_status("foobar"), CheckStatus::Malformed);
        checks.set_check_status("foobar", CheckStatus::Good);
        assert_eq!(checks.get_check_status("foobar"), CheckStatus::Good);
    }
}
