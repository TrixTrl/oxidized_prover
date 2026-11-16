use std::{rc::Rc, vec};

use crate::{
    lookup_table::{CopyExact, LookupID, LookupTable},
    math_definition::{
        Definition,
        Definition_Field::{ExistingType, New, NoValue},
    },
    math_function::{Function, create_function},
};

pub struct Type {
    pub name: String,
    pub constructors: vec::Vec<LookupID<Function>>,
}

impl Type {
    pub fn stringify(&self) -> String {
        String::from(&self.name)
    }
}

impl CopyExact for Type {
    fn copy(&self) -> Type {
        Type {
            name: self.name.clone(),
            constructors: vec![],
        }
    }
}

pub fn create_type(
    type_table: &Rc<LookupTable<Type>>,
    function_table: &Rc<LookupTable<Function>>,
    name: String,
    definitions: vec::Vec<Definition>,
) -> LookupID<Type> {
    let mut type_index = LookupTable::add_value(
        type_table,
        Type {
            name: name.clone(),
            constructors: vec![],
        },
    );
    let mut defs: vec::Vec<LookupID<Function>> = vec![];
    for d in definitions {
        defs.push(create_function(
            function_table,
            d.name,
            match &d.input_1_type {
                NoValue => Option::None,
                ExistingType(t) => Option::from(t),
                New => Option::from(&type_index),
            },
            match &d.input_2_type {
                NoValue => Option::None,
                ExistingType(t) => Option::from(t),
                New => Option::from(&type_index),
            },
            &type_index,
        ));
    }
    type_index.update_value(Type {
        name,
        constructors: defs,
    });
    return type_index;
}

pub fn print_type(t: &Type) {
    let name = t.name.clone();
    println!("{name}");
    for c in &t.constructors {
        let definition = c.copy().get_value().stringify();
        println!("| {definition}");
    }
}

pub fn print_types(table: &Rc<LookupTable<Type>>) {
    println!("Types:");
    for (_k, v) in table.map.borrow().iter() {
        print_type(v);
    }
}
