use parity_wasm::elements::{FunctionType, ValueType};
use std::collections::HashMap;

static EEI_REV: i32 = 3;

/// Struct mapping an imported function name to its expected signature.
pub struct ImportInterfaceMap {
    import_list: HashMap<String, FunctionType>,
}

impl ImportInterfaceMap {
    pub fn new() -> Self {
        ImportInterfaceMap {
            import_list: HashMap::new(),
        }
    }

    pub fn get_func(&self, name: &str) -> Option<&FunctionType> {
        self.import_list.get(&name.to_string()).clone()
    }
}

impl Default for ImportInterfaceMap {
    fn default() -> Self {
        ImportInterfaceMap {
            import_list: {
                let imports: HashMap<String, FunctionType> = [
                    (
                        "useGas".to_string(),
                        FunctionType::new(vec![ValueType::I64], None),
                    ),
                    (
                        "getGasLeft".to_string(),
                        FunctionType::new(Vec::new(), Some(ValueType::I64)),
                    ),
                    (
                        "getAddress".to_string(),
                        FunctionType::new(vec![ValueType::I32], None),
                    ),
                    (
                        "getExternalBalance".to_string(),
                        FunctionType::new(vec![ValueType::I32, ValueType::I32], None),
                    ),
                    (
                        "getBlockHash".to_string(),
                        FunctionType::new(
                            vec![ValueType::I64, ValueType::I32],
                            Some(ValueType::I32),
                        ),
                    ),
                    (
                        "call".to_string(),
                        FunctionType::new(
                            vec![
                                ValueType::I64,
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                            ],
                            Some(ValueType::I32),
                        ),
                    ),
                    (
                        "callCode".to_string(),
                        FunctionType::new(
                            vec![
                                ValueType::I64,
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                            ],
                            Some(ValueType::I32),
                        ),
                    ),
                    (
                        "callDelegate".to_string(),
                        FunctionType::new(
                            vec![
                                ValueType::I64,
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                            ],
                            Some(ValueType::I32),
                        ),
                    ),
                ].iter()
                    .cloned()
                    .collect();
                imports
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use eei::ImportInterfaceMap;
    use parity_wasm::elements::{FunctionType, ValueType};

    #[test]
    fn empty_interface() {
        let iface = ImportInterfaceMap::new();
        assert!(iface.import_list.is_empty());
    }

    #[test]
    fn default_interface_has_usegas() {
        let iface = ImportInterfaceMap::default();
        assert_eq!(
            iface.get_func("useGas").unwrap().clone(),
            FunctionType::new(vec![ValueType::I64], None)
        );
    }
}
