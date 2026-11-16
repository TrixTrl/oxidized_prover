use std::rc::Rc;

use crate::{
    lookup_table::{CopyExact, LookupID, LookupTable},
    math_function::Function,
};

#[derive(Clone)]
pub struct Execution {
    function: LookupID<Function>,
    input_1: Option<LookupID<Execution>>,
    input_2: Option<LookupID<Execution>>,
}

pub fn create_execution(
    table: &Rc<LookupTable<Execution>>,
    function: &LookupID<Function>,
    input_1: Option<&LookupID<Execution>>,
    input_2: Option<&LookupID<Execution>>,
) -> LookupID<Execution> {
    LookupTable::add_value(
        table,
        Execution {
            function: function.copy(),
            input_1: match input_1 {
                None => Option::None,
                Some(ref x) => Option::from(x.copy()),
            },
            input_2: match input_2 {
                None => Option::None,
                Some(ref x) => Option::from(x.copy()),
            },
        },
    )
}

impl Execution {
    pub fn stringify(&self) -> String {
        let input_1 = match self.input_1 {
            None => String::from(""),
            Some(x) => x.get_value(),
        };
        String::from("")
    }
}

pub fn print_execution(e: &Execution) {}
