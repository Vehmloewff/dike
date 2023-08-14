use crate::{memory::Memory, stack::Stack};

pub fn execute_interop(stack: &Stack, memory: &Memory, fn_number: u32) {
    match fn_number {
        0 => {
            // let string = get_rusty_types_string(stack.consume().get_rusty_value(memory));
            // let value = ;
            let (string, reviver) = stack.consume().grab_rusty_value(memory);

            reviver.revive(memory, string);
        }
        _ => panic!("Unknown function number"),
    }
}

pub enum RustyValue {
    // ...
    RustyTypesString(String),
}

impl RustyValue {
    fn get_rusty_types_string(self) -> String {
        match self {
            RustyValue::RustyTypesString(internal) => internal,
            _ => panic!("Expected rusty_types::String"),
        }
        // Ref::map(value_ref, |value| match value {
        //     RustyValue::RustyTypesString(internal) => internal,
        //     _ => panic!("Expected rusty_types::String"),
        // })
    }

    fn deref(self) {}
}
