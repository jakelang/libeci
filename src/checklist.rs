use std::collections::HashMap;

/*
 * Enum describing the state of a check on an ECI property.
 * This is always initialized as "Unknown", and changed during
 * the check phase to reflect its degree of ECI conformance.
 */
#[derive(Copy, Clone)]
pub enum CheckStatus {
    Unknown,
    Nonexistent,
    Malformed,
    Good,
}

pub struct EciChecklist {
    checklist: HashMap<String, CheckStatus>
}

impl EciChecklist {
    pub fn new() -> Self {
        EciChecklist {
            checklist: HashMap::new()
        }
    }

    pub fn eci_default() -> Self {
        let checks: HashMap<String, CheckStatus> = 
            [("export-main".to_string(), CheckStatus::Unknown),
             ("export-memory".to_string(), CheckStatus::Unknown),
             ("eei-imports".to_string(), CheckStatus::Unknown)]
            .iter().cloned().collect();

        EciChecklist {
            checklist: checks
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
        let checks = EciChecklist::eci_default();
        assert!(checks.checklist.contains_key(&"export-main".to_string()));
        assert!(checks.checklist.contains_key(&"export-memory".to_string()));
        assert!(checks.checklist.contains_key(&"eei-imports".to_string()));
    }
}
