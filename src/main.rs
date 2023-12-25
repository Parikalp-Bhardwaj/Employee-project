mod model;
mod utils;
use std::io;
fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut emp = model::List::new();
    let mut input = String::new();
    // emp.add_person(1, "Sam", "Morris", 22, "Doctor");
    // emp.add_person(2, "Jhon", "Cartin", 23, "Engineer");
    // emp.add_person(3, "Finn", "Perry", 21, "HR");
    // println!("\n");
    // println!("Enter an unsigned integer \n1. Add Person \n2. Emp Information By ID: \n3. Emp Information Edit:");
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");

    // // Parse the string into an integer
    // let ans: Result<i32, _> = input.trim().parse();
    // // Match the result
    // match ans {
    //     Ok(val) => emp.process_choice(val),
    //     Err(_) => {
    //         println!("Only Numbers 1 - 4");
    //     }
    // };

    // emp.print_all_employees();

    let list = utils::load_from_json("./output.json");
    match list {
        Ok(data) => {
            for person in &data.People {
                emp.add_person(
                    person.id,
                    &person.fname,
                    &person.lname,
                    person.age,
                    &person.profession,
                )
            }
        }
        Err(err) => println!("Error loading data from JSON file: {}", err),
    };

    emp.print_all_employees();
    // utils::load_from_json("./output.json")

    // match emp.save_to_json("./output.json") {
    //     Ok(()) => {
    //         println!("Data saved successfully");
    //     }
    //     Err(err) => {
    //         eprintln!("Error saving data: {}", err);
    //     }
    // }
    // emp.save_to_json("./output.json");
    Ok(())
}
