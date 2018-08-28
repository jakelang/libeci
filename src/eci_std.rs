/*
 * libeci: Ethereum WebAssembly ABI compliance library
 *
 * Copyright (c) 2018 Jake Lang
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use checklist::CheckStatus;
use eei::ImportInterfaceMap;
use parity_wasm::elements::{External, FunctionType, Internal, Module};
use pwasm::*;

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

/// Checks that the EEI host functions have been imported with the correct namespace. 
pub fn chk_eei_namespace(module: &Module) -> CheckStatus {
    if has_import_section(module) {
        imports_only_eei_namespace(module)
    } else {
        CheckStatus::Good
    }
}

/// Ensures that a module has not incorrectly specified a start function.
pub fn chk_no_startfn(module: &Module) -> CheckStatus {
    match module.start_section() {
        Some(_thing) => CheckStatus::Malformed,
        None => CheckStatus::Good,
    }
}

/// Verifies that the EEI has been imported with the correct function signatures.
pub fn chk_func_signatures(module: &Module) -> CheckStatus {
    if has_import_section(module) {
        eei_check_func_sigs(module)
    } else {
        CheckStatus::Good
    }
}

/*
 * Utilities
 */

/// Utility function checking that a module has an exported function with a given signature.
pub fn has_func_export(module: &Module, name: &str, sig: FunctionType) -> CheckStatus {
    match resolve_export_by_name(module, name) {
        Some((index, reference)) => if reference == Internal::Function(index)
            && func_type_by_index(module, index as usize) == sig
        {
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
        if module != "ethereum" {
            return CheckStatus::Malformed;
        }
    }
    CheckStatus::Good
}

/// Checks the imported function signatures against the EEI import list. Will fail if the module
/// doesn't have an import section.
pub fn eei_check_func_sigs(module: &Module) -> CheckStatus {
    let eei = ImportInterfaceMap::default();

    module
        .import_section()
        .unwrap()
        .entries()
        .iter()
        .map(|x| (x.field(), x.external()))
        .map(|(name, binding)| {
            (
                eei.get_func(name),
                match *binding {
                    External::Function(idx) => Some(imported_func_type_by_index(module, idx as usize)),
                    _ => None,
                },
            )
        })
        .map(|(correctsig, funcsig)| {
            if correctsig != None
                && funcsig != None
                && funcsig.unwrap().clone() == correctsig.unwrap()
            {
                CheckStatus::Good
            } else {
                CheckStatus::Malformed
            }
        })
        .find(|x| {
            if *x == CheckStatus::Malformed {
                true
            } else {
                false
            }
        })
        .unwrap_or(CheckStatus::Good)
}
