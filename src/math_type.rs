use std::{rc::Rc, vec};

use crate::{
    lookup_table::{LookupID, LookupTable},
    math_definition::{
        Definition,
        Definition_Field::{ExistingType, New, NoValue},
    },
    math_function::{Function, create_function},
};

pub struct Type {
    pub name: String,
    pub constructors: Vec<Rc<LookupID<Function>>>,
}

impl Clone for Type {
    fn clone(&self) -> Self {
        Type {
            name: self.name.clone(),
            constructors: Vec::clone(&self.constructors),
        }
    }
}

impl Type {
    pub fn stringify(&self) -> String {
        String::from(&self.name)
    }
}

pub fn create_type(
    type_table: &Rc<LookupTable<Type>>,
    function_table: &Rc<LookupTable<Function>>,
    name: String,
    definitions: vec::Vec<Rc<Definition>>,
) -> (Rc<LookupID<Type>>, Vec<Rc<LookupID<Function>>>) {
    let type_index = LookupTable::add_value(
        type_table,
        Type {
            name: name.clone(),
            constructors: vec![],
        },
    );
    let mut lookup_ref = Rc::from(type_index);
    let mut defs: vec::Vec<Rc<LookupID<Function>>> = vec![];
    for d in definitions {
        defs.push(Rc::from(create_function(
            function_table,
            String::from(&d.name),
            match &d.input_1_type {
                NoValue => Option::None,
                ExistingType(t) => Option::from(Rc::clone(t)),
                New => Option::from(Rc::clone(&lookup_ref)),
            },
            match &d.input_2_type {
                NoValue => Option::None,
                ExistingType(t) => Option::from(Rc::clone(t)),
                New => Option::from(Rc::clone(&lookup_ref)),
            },
            Rc::clone(&lookup_ref),
        )));
    }
    Rc::make_mut(&mut lookup_ref)
        .update_value(Type {
            name,
            constructors: Vec::clone(&defs),
        });
    (lookup_ref, defs)
}

pub fn print_type(t: &Type) {
    let name = t.name.clone();
    println!("{name}");
    for c in &t.constructors {
        let definition = c.get_value().stringify();
        println!("| {definition}");
    }
}

pub fn print_types(table: &Rc<LookupTable<Type>>) {
    println!("\nTypes:");
    for (_k, v) in table.map.borrow().iter() {
        print_type(v);
    }
}
