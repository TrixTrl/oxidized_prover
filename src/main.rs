mod lookup_table;
mod math_definition;
mod math_execution;
mod math_function;
mod math_type;

use std::rc::Rc;

use math_function::Function;

use crate::{
    lookup_table::{LookupTable, create_lookup_table},
    math_definition::Definition,
    math_execution::Execution,
    math_function::{create_function, print_functions},
    math_type::{Type, create_type, print_types},
};

fn main() {
    let type_table: Rc<LookupTable<Type>> = Rc::from(create_lookup_table());
    let function_table: Rc<LookupTable<Function>> = Rc::from(create_lookup_table());
    let execution_table: Rc<LookupTable<Execution>> = Rc::from(create_lookup_table());

    let true_f = Definition {
        name: String::from("True"),
        input_1_type: math_definition::Definition_Field::NoValue,
        input_2_type: math_definition::Definition_Field::NoValue,
    };
    let false_f = Definition {
        name: String::from("False"),
        input_1_type: math_definition::Definition_Field::NoValue,
        input_2_type: math_definition::Definition_Field::NoValue,
    };

    let zero_f = Definition {
        name: String::from("Zero"),
        input_1_type: math_definition::Definition_Field::NoValue,
        input_2_type: math_definition::Definition_Field::NoValue,
    };
    let succ_f = Definition {
        name: String::from("Succ"),
        input_1_type: math_definition::Definition_Field::New,
        input_2_type: math_definition::Definition_Field::NoValue,
    };

    let bool_t = create_type(
        &type_table,
        &function_table,
        String::from("Bool"),
        vec![true_f, false_f],
    );
    let nat_t = create_type(
        &type_table,
        &&function_table,
        String::from("Nat"),
        vec![zero_f, succ_f],
    );

    let eq_b = create_function(
        &function_table,
        String::from("Equals_b"),
        Option::from(&bool_t),
        Option::from(&bool_t),
        &bool_t,
    );
    let eq_n = create_function(
        &function_table,
        String::from("Equals_n"),
        Option::from(&nat_t),
        Option::from(&nat_t),
        &bool_t,
    );

    let not = create_function(
        &function_table,
        String::from("Not"),
        Option::from(&bool_t),
        Option::None,
        &bool_t,
    );
    let and = create_function(
        &function_table,
        String::from("And"),
        Option::from(&bool_t),
        Option::from(&bool_t),
        &bool_t,
    );
    let or = create_function(
        &function_table,
        String::from("Or"),
        Option::from(&bool_t),
        Option::from(&bool_t),
        &bool_t,
    );

    let add = create_function(
        &function_table,
        String::from("Add"),
        Option::from(&nat_t),
        Option::from(&nat_t),
        &nat_t,
    );
    let mul = create_function(
        &function_table,
        String::from("Mul"),
        Option::from(&nat_t),
        Option::from(&nat_t),
        &nat_t,
    );
    let pow = create_function(
        &function_table,
        String::from("Pow"),
        Option::from(&nat_t),
        Option::from(&nat_t),
        &nat_t,
    );

    print_types(&type_table);
    print_functions(&function_table);
}
