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
                    (
                        "callStatic".to_string(),
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
                    (
                        "create".to_string(),
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
                        "callDataCopy".to_string(),
                        FunctionType::new(
                            vec![ValueType::I32, ValueType::I32, ValueType::I32],
                            None,
                        ),
                    ),
                    (
                        "getCallDataSize".to_string(),
                        FunctionType::new(Vec::new(), Some(ValueType::I32)),
                    ),
                    (
                        "getCodeSize".to_string(),
                        FunctionType::new(Vec::new(), Some(ValueType::I32)),
                    ),
                    (
                        "externalCodeCopy".to_string(),
                        FunctionType::new(
                            vec![
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                            ],
                            None,
                        ),
                    ),
                    (
                        "getExternalCodeSize".to_string(),
                        FunctionType::new(vec![ValueType::I32], Some(ValueType::I32)),
                    ),
                    (
                        "getCaller".to_string(),
                        FunctionType::new(vec![ValueType::I32], None),
                    ),
                    (
                        "getCallValue".to_string(),
                        FunctionType::new(vec![ValueType::I32], None),
                    ),
                    (
                        "getBlockDifficulty".to_string(),
                        FunctionType::new(vec![ValueType::I32], None),
                    ),
                    (
                        "getBlockCoinbase".to_string(),
                        FunctionType::new(vec![ValueType::I32], None),
                    ),
                    (
                        "getBlockNumber".to_string(),
                        FunctionType::new(Vec::new(), Some(ValueType::I64)),
                    ),
                    (
                        "getBlockGasLimit".to_string(),
                        FunctionType::new(Vec::new(), Some(ValueType::I64)),
                    ),
                    (
                        "getBlockTimestamp".to_string(),
                        FunctionType::new(Vec::new(), Some(ValueType::I64)),
                    ),
                    (
                        "getTxGasPrice".to_string(),
                        FunctionType::new(vec![ValueType::I32], None),
                    ),
                    (
                        "getTxOrigin".to_string(),
                        FunctionType::new(vec![ValueType::I32], None),
                    ),
                    (
                        "storageStore".to_string(),
                        FunctionType::new(vec![ValueType::I32, ValueType::I32], None),
                    ),
                    (
                        "storageLoad".to_string(),
                        FunctionType::new(vec![ValueType::I32, ValueType::I32], None),
                    ),
                    (
                        "log".to_string(),
                        FunctionType::new(
                            vec![
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                                ValueType::I32,
                            ],
                            None,
                        ),
                    ),
                    (
                        "getReturnDataSize".to_string(),
                        FunctionType::new(Vec::new(), Some(ValueType::I32)),
                    ),
                    (
                        "returnDataCopy".to_string(),
                        FunctionType::new(
                            vec![ValueType::I32, ValueType::I32, ValueType::I32],
                            None,
                        ),
                    ),
                    (
                        "finish".to_string(),
                        FunctionType::new(vec![ValueType::I32, ValueType::I32], None),
                    ),
                    (
                        "revert".to_string(),
                        FunctionType::new(vec![ValueType::I32, ValueType::I32], None),
                    ),
                    (
                        "selfDestruct".to_string(),
                        FunctionType::new(vec![ValueType::I32], None),
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
    fn default_interface_has_methods() {
        let iface = ImportInterfaceMap::default();
        assert_eq!(
            iface.get_func("useGas").unwrap().clone(),
            FunctionType::new(vec![ValueType::I64], None)
        );
        assert_eq!(
            iface.get_func("create").unwrap().clone(),
            FunctionType::new(
                vec![
                    ValueType::I64,
                    ValueType::I32,
                    ValueType::I32,
                    ValueType::I32,
                    ValueType::I32,
                ],
                Some(ValueType::I32)
            )
        );
    }
}
