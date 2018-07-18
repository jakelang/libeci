use parity_wasm::elements::{Module, Type, FunctionType, Internal, External};

/* Borrowed from pwasm examples */
fn type_by_index(module: &Module, index: usize) -> FunctionType {

	let function_section = module.function_section().expect("No function section found");
	let type_section = module.type_section().expect("No type section found");

	let import_section_len: usize = match module.import_section() {
			Some(import) =>
				import.entries().iter().filter(|entry| match entry.external() {
					&External::Function(_) => true,
					_ => false,
					}).count(),
			None => 0,
		};

	let function_index_in_section = index - import_section_len;

	let func_type_ref: usize = function_section.entries()[function_index_in_section].type_ref() as usize;

	match type_section.types()[func_type_ref] {
		Type::Function(ref func_type) => func_type.clone(),
	}
}

fn export_index_by_name(module: &Module, name: &str) -> usize {
    assert!(has_export_section(module));
    let idx: usize = match *module.export_section().unwrap().entries().iter()
        .find(|export|
                if export.field() == name { true } else { false })
        .unwrap()
        .internal() { //Is there any way to do this more simply?
            Internal::Function(index) => index as usize,
            Internal::Memory(index) => index as usize,
            Internal::Global(index) => index as usize,
            Internal::Table(index) => index as usize,
        };

    idx
}

pub fn has_export_section(module: &Module) -> bool {
    match module.export_section() {
        Some(_thing) => true,
        None => false,
    }
}
