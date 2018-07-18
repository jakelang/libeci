use parity_wasm::elements::Module;
use checklist::CheckStatus;

pub fn chk_main_exported(module: &Module) -> CheckStatus {
    CheckStatus::Good
}

pub fn chk_mem_exported(module: &Module) -> CheckStatus {
    CheckStatus::Good
}

pub fn chk_eei_imported(module: &Module) -> CheckStatus {
    CheckStatus::Good
}
