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
    math_execution::{Execution, create_execution, print_executions},
    math_function::{create_function, print_functions},
    math_type::{Type, create_type, print_types},
};

fn main() {
    let type_table: Rc<LookupTable<Type>> = Rc::from(create_lookup_table());
    let function_table: Rc<LookupTable<Function>> = Rc::from(create_lookup_table());
    let execution_table: Rc<LookupTable<Execution>> = Rc::from(create_lookup_table());

    let true_d = Rc::from(Definition {
        name: String::from("True"),
        input_1_type: math_definition::Definition_Field::NoValue,
        input_2_type: math_definition::Definition_Field::NoValue,
    });
    let false_d = Rc::from(Definition {
        name: String::from("False"),
        input_1_type: math_definition::Definition_Field::NoValue,
        input_2_type: math_definition::Definition_Field::NoValue,
    });

    let zero_d = Rc::from(Definition {
        name: String::from("Zero"),
        input_1_type: math_definition::Definition_Field::NoValue,
        input_2_type: math_definition::Definition_Field::NoValue,
    });
    let succ_d = Rc::from(Definition {
        name: String::from("Succ"),
        input_1_type: math_definition::Definition_Field::New,
        input_2_type: math_definition::Definition_Field::NoValue,
    });

    let (bool_t, bool_defs) = create_type(
        &type_table,
        &function_table,
        String::from("Bool"),
        vec![true_d, false_d],
    );
    let (nat_t, nat_defs) = create_type(
        &type_table,
        &&function_table,
        String::from("Nat"),
        vec![zero_d, succ_d],
    );

    let true_f = Rc::clone(&bool_defs[0]);
    let false_f = Rc::clone(&bool_defs[1]);
    let zero_f = Rc::clone(&nat_defs[0]);
    let succ_f = Rc::clone(&nat_defs[1]);

    let eq_b = create_function(
        &function_table,
        String::from("Equals_b"),
        Option::from(Rc::clone(&bool_t)),
        Option::from(Rc::clone(&bool_t)),
        Rc::clone(&bool_t),
    );
    let eq_n = create_function(
        &function_table,
        String::from("Equals_n"),
        Option::from(Rc::clone(&nat_t)),
        Option::from(Rc::clone(&nat_t)),
        Rc::clone(&bool_t),
    );

    let not = create_function(
        &function_table,
        String::from("Not"),
        Option::from(Rc::clone(&bool_t)),
        Option::None,
        Rc::clone(&bool_t),
    );
    let and = create_function(
        &function_table,
        String::from("And"),
        Option::from(Rc::clone(&bool_t)),
        Option::from(Rc::clone(&bool_t)),
        Rc::clone(&bool_t),
    );
    let or = create_function(
        &function_table,
        String::from("Or"),
        Option::from(Rc::clone(&bool_t)),
        Option::from(Rc::clone(&bool_t)),
        Rc::clone(&bool_t),
    );

    let add = create_function(
        &function_table,
        String::from("Add"),
        Option::from(Rc::clone(&nat_t)),
        Option::from(Rc::clone(&nat_t)),
        Rc::clone(&nat_t),
    );
    let mul = create_function(
        &function_table,
        String::from("Mul"),
        Option::from(Rc::clone(&nat_t)),
        Option::from(Rc::clone(&nat_t)),
        Rc::clone(&nat_t),
    );
    let pow = create_function(
        &function_table,
        String::from("Pow"),
        Option::from(Rc::clone(&nat_t)),
        Option::from(Rc::clone(&nat_t)),
        Rc::clone(&nat_t),
    );

    print_types(&type_table);
    print_functions(&function_table);

    let zero = create_execution(
        &execution_table,
        Rc::clone(&zero_f),
        Option::None,
        Option::None,
    );
    let one = create_execution(
        &execution_table,
        Rc::clone(&succ_f),
        Option::from(Rc::clone(&zero)),
        Option::None,
    );

    print_executions(&execution_table);

    println!("Wait what");
}
