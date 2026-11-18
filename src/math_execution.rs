use std::rc::Rc;

use crate::{
    lookup_table::{LookupID, LookupTable},
    math_function::Function,
};

#[derive(Clone)]
pub struct Execution {
    function: Rc<LookupID<Function>>,
    input_1: Option<Rc<LookupID<Execution>>>,
    input_2: Option<Rc<LookupID<Execution>>>,
}

pub fn create_execution(
    table: &Rc<LookupTable<Execution>>,
    function: Rc<LookupID<Function>>,
    input_1: Option<Rc<LookupID<Execution>>>,
    input_2: Option<Rc<LookupID<Execution>>>,
) -> Rc<LookupID<Execution>> {
    Rc::from(LookupTable::add_value(
        table,
        Execution {
            function: Rc::clone(&function),
            input_1: match input_1 {
                None => Option::None,
                Some(ref x) => Option::from(Rc::clone(x)),
            },
            input_2: match input_2 {
                None => Option::None,
                Some(ref x) => Option::from(Rc::clone(x)),
            },
        },
    ))
}

impl Execution {
    pub fn stringify(&self) -> String {
        let function_name = self.function.get_value().name;
        let input_1 = match &self.input_1 {
            None => String::from(""),
            Some(x) => x.get_value().stringify(),
        };
        let input_2 = match &self.input_2 {
            None => String::from(""),
            Some(x) => x.get_value().stringify(),
        };
        String::from(format!("{function_name}({input_1},{input_2})"))
    }
}

pub fn print_execution(e: &Execution) {
    println!("{}", e.stringify());
}

pub fn print_executions(table: &Rc<LookupTable<Execution>>) {
    println!("\nExecutions:");
    for (_k, v) in table.map.borrow().iter() {
        print_execution(v);
    }
}
