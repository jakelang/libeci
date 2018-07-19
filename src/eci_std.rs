use parity_wasm::elements::{Module, Internal, FunctionType};
use pwasm::*;
use checklist::CheckStatus;

pub fn chk_main_exported(module: &Module) -> CheckStatus {
    has_func_export(module, "main", FunctionType::default())
}

pub fn chk_mem_exported(module: &Module) -> CheckStatus {
    match resolve_export_by_name(module, "memory") {
        Some((index, reference)) => if reference == Internal::Memory(index) {
            CheckStatus::Good
        } else {
            CheckStatus::Malformed
        },
        None => CheckStatus::Nonexistent,
    }
}

pub fn chk_eei_imported(module: &Module) -> CheckStatus {
    CheckStatus::Good
}

/*
 * Utilities
 */
pub fn has_func_export(module: &Module, name: &str, sig: FunctionType) -> CheckStatus {
    match resolve_export_by_name(module, name) {
        Some((index, reference)) => if reference == Internal::Function(index) && func_type_by_index(module, index as usize) == sig { 
            CheckStatus::Good 
        } else { 
            CheckStatus::Malformed 
        },
        None => CheckStatus::Nonexistent,
    }
}
