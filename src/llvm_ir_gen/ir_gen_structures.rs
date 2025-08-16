use std::collections::HashMap;

use inkwell::{builder::Builder, context::Context, module::{Linkage, Module}, types::{IntType, PointerType}, values::{FunctionValue, GlobalValue, PointerValue}, AddressSpace};

// IRGenUtil
pub struct IRGenUtil<'ctx> {
    pub context: &'ctx Context,
    pub module:  Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub i16_type:  IntType<'ctx>, 
    pub i16p_type: PointerType<'ctx>,
    pub i32_type:  IntType<'ctx>,
    pub register_table: GlobalValue<'ctx>,
    pub microcode_function_map: HashMap<String, FunctionValue<'ctx>>,
    pub format_string_registry: FmtStringRegistry<'ctx>,
}

impl<'ctx> IRGenUtil<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self {
        let module = context.create_module(name);
        let builder = context.create_builder();
        let i16_type = context.i16_type();
        let i16p_type = i16_type.ptr_type(AddressSpace::default());
        let i32_type = context.i32_type();
        let microcode_function_map = HashMap::new();


        // initialize the register table...
        let array_type = i16_type.array_type(8);
        let zero_initializer = i16_type.const_int(0, false);
        let initializer = i16_type.const_array(&vec![zero_initializer; 8]);
        let register_table = module.add_global(array_type, None, "register_table");
        register_table.set_initializer(&initializer);
        register_table.set_linkage(Linkage::Internal);

        let format_string_registry = FmtStringRegistry::new();


        Self { context, module, builder, i16_type, i16p_type, i32_type, microcode_function_map, register_table, format_string_registry }
    }

    pub fn get_register_ptr_const(&self, index: u32) -> PointerValue<'ctx> {
        let zero = self.context.i32_type().const_zero();
        let idx = self.context.i32_type().const_int(index as u64, false);

        unsafe {
            self.builder
                .build_gep(
                    self.i16_type,
                    self.builder.build_gep(
                        self.i16_type.array_type(8),
                        self.register_table.as_pointer_value(),
                        &[zero],
                        "reg_base",
                    ).unwrap(),
                    &[idx],
                    "reg"
                )
                .unwrap()
        }
    }
    
    pub fn cache_microcode(&mut self, name: String, function: FunctionValue<'ctx>) {
        self.microcode_function_map.insert(name, function);
    }

    pub fn extract_microcode_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        match self.microcode_function_map.get(name) {
            Some(function) => Some(*function),
            None => None
        }
    }
}



#[derive(PartialEq, Eq, Hash)]
pub enum FormatStringKey {
    SignedInt,
    UnsignedInt,
    Newline,
}
pub struct FmtStringRegistry<'ctx> {
    registry: HashMap<FormatStringKey, PointerValue<'ctx>>,
}

impl<'ctx> FmtStringRegistry<'ctx> {
    pub fn new() -> Self {
        let registry = HashMap::new();
        Self { registry }
    }

    pub fn get_fmt_string(&self, fmt_ky: FormatStringKey) -> PointerValue<'ctx> {
        self.registry[&fmt_ky]
    }

    pub fn initialize_registry(&mut self, builder: &Builder<'ctx>) -> () {
        let fmt_str_cb = |fmt_str: &str, name: &str| {
            builder.build_global_string_ptr(fmt_str, name).expect("Failed to build global string").as_pointer_value()
        };

        self.registry.insert(FormatStringKey::UnsignedInt, fmt_str_cb("%hu", "unsigned_int_fmt"));
        self.registry.insert(FormatStringKey::SignedInt, fmt_str_cb("%hd", "signed_int_fmt"));
        self.registry.insert(FormatStringKey::Newline, fmt_str_cb("\n", "nl_fmt"));
    }
}