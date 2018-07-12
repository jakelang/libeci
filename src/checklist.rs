use std::collections::HashMap;

/*
 * Enum describing the state of a check on an ECI property.
 * This is always initialized as "Unknown", and changed during
 * the check phase to reflect its degree of ECI conformance.
 */
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CheckStatus {
    Unknown,
    Nonexistent,
    Malformed,
    Good,
}

#[derive(Clone)]
pub struct EciChecklist {
    checklist: HashMap<String, CheckStatus>
}

impl EciChecklist {
    pub fn new() -> Self {
        EciChecklist {
            checklist: HashMap::new()
        }
    }

    pub fn default() -> Self {
        let checks: HashMap<String, CheckStatus> = 
            [("export-main".to_string(), CheckStatus::Unknown),
             ("export-memory".to_string(), CheckStatus::Unknown),
             ("eei-imports".to_string(), CheckStatus::Unknown)]
            .iter().cloned().collect();

        EciChecklist {
            checklist: checks
        }
    }

    pub fn add_check(&mut self, key: &str) {
        self.checklist.insert(key.to_string(), CheckStatus::Unknown);
    }

    pub fn set_check_status(&mut self, key: &str, val: CheckStatus) {
        *self.checklist.get_mut(&key.to_string()).unwrap() = val;
    }

    pub fn get_check_status(&self, key: &str) -> CheckStatus {
        self.checklist[&key.to_string()].clone()
    }

    pub fn check_is_good(&self, key: &str) -> bool {
        match self.get_check_status(key) {
            CheckStatus::Good => true,
            _ => false
        }
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
        checks.add_check("random-arbitrary-check");
        assert!(checks.checklist.contains_key(&"random-arbitrary-check".to_string()));
    }

    #[test]
    fn verify_check() {
        let mut checks = EciChecklist::new();
        checks.add_check("foobar");
        checks.set_check_status("foobar", CheckStatus::Good);
        assert!(checks.check_is_good("foobar"));
    }

    #[test]
    fn test_check_eq() {
        let mut checks = EciChecklist::new();
        checks.add_check("foobar");
        checks.set_check_status("foobar", CheckStatus::Nonexistent);
        assert_eq!(checks.get_check_status("foobar"), CheckStatus::Nonexistent);
        checks.set_check_status("foobar", CheckStatus::Malformed);
        assert_eq!(checks.get_check_status("foobar"), CheckStatus::Malformed);
        checks.set_check_status("foobar", CheckStatus::Good);
        assert_eq!(checks.get_check_status("foobar"), CheckStatus::Good);
    }
}
