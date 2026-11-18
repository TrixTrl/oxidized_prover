use std::rc::Rc;

use crate::{
    lookup_table::{LookupID, LookupTable},
    math_type::Type,
};

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub input_1_type: Option<Rc<LookupID<Type>>>,
    pub input_2_type: Option<Rc<LookupID<Type>>>,
    pub output_type: Rc<LookupID<Type>>,
}

/*
impl CopyExact for Function {
    fn copy(&self) -> Function {
        Function {
            name: self.name.clone(),
            input_1_type: match self.input_1_type {
                None => Option::None,
                Some(ref x) => Option::from(Rc::clone(x)),
            },
            input_2_type: match self.input_2_type {
                None => Option::None,
                Some(ref x) => Option::from(Rc::clone(x)),
            },
            output_type: Rc::clone(&self.output_type),
        }
    }
}
*/

pub fn create_function(
    table: &Rc<LookupTable<Function>>,
    name: String,
    input_1_type: Option<Rc<LookupID<Type>>>,
    input_2_type: Option<Rc<LookupID<Type>>>,
    output_type: Rc<LookupID<Type>>,
) -> LookupID<Function> {
    assert_ne!(
        name,
        "".to_string(),
        "Tried defining a function with a blank name"
    );

    LookupTable::add_value(
        table,
        Function {
            name: name.clone(),
            input_1_type: match input_1_type {
                None => Option::None,
                Some(ref x) => Option::from(Rc::clone(x)),
            },
            input_2_type: match input_2_type {
                None => Option::None,
                Some(ref x) => Option::from(Rc::clone(x)),
            },
            output_type: Rc::clone(&output_type),
        },
    )
}

impl Function {
    pub fn stringify(&self) -> String {
        let name = String::from(&self.name);
        let input_1_type = String::from(match self.input_1_type {
            None => String::from(""),
            Some(ref x) => x.get_value().stringify(),
        });
        let input_2_type = {
            match self.input_2_type {
                None => String::from(""),
                Some(ref x) => x.get_value().stringify(),
            }
        };
        let ouput_type = String::from(&self.output_type.get_value().name);
        String::from(format!(
            "{name}: ({input_1_type},{input_2_type}) -> {ouput_type}"
        ))
    }
}

pub fn print_function(f: &Function) {
    let name = String::from(&f.name);
    let input_1_type = String::from(match f.input_1_type {
        None => String::from(""),
        Some(ref x) => x.get_value().stringify(),
    });
    let input_2_type = {
        match f.input_2_type {
            None => String::from(""),
            Some(ref x) => x.get_value().stringify(),
        }
    };
    let ouput_type = String::from(&f.output_type.get_value().name);
    println!("{name}: ({input_1_type},{input_2_type}) -> {ouput_type}");
}

pub fn print_functions(table: &Rc<LookupTable<Function>>) {
    println!("\nFunctions:");
    for (_k, v) in table.map.borrow().iter() {
        print_function(v);
    }
}
