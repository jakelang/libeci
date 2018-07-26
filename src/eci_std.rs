use parity_wasm::elements::{Module, Internal, FunctionType};
use pwasm::*;
use checklist::CheckStatus;

/// Checks that the module's "main" function has been exported with no arguments or return values.
pub fn chk_main_exported(module: &Module) -> CheckStatus {
    has_func_export(module, "main", FunctionType::default())
}

/// Checks that the module's memory segment has been properly exported.
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

/// Checks that the EEI host functions have been imported with the correct namespace and signatures. 
pub fn chk_eei_imported(module: &Module) -> CheckStatus {
    if has_import_section(module) { imports_only_eei_namespace(module) }
    else { CheckStatus::Good }
}

/// Ensures that a module has not incorrectly specified a start function.
pub fn chk_no_startfn(module: &Module) -> CheckStatus {
    match module.start_section() {
        Some(_thing) => CheckStatus::Malformed,
        None => CheckStatus::Good,
    } 
}

/*
 * Utilities
 */

/// Utility function checking that a module has an exported function with a given signature.
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

pub fn imports_only_eei_namespace(module: &Module) -> CheckStatus {
    let importlist = get_imports(module).unwrap();

    for (module, _field) in importlist {
       if module != "ethereum" { return CheckStatus::Malformed; }
    }
    
    CheckStatus::Good
}
