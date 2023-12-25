mod model;
mod utils;
use std::io;
fn main() -> io::Result<()> {
    let mut emp = model::List::new();
    let mut input = String::new();
    // println!("\n");

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

    println!("Enter an unsigned integer \n1. Add Person \n2. Emp Information By ID: \n3. Emp Information Edit: \n4. Remove Employee");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the string into an integer
    let ans: Result<i32, _> = input.trim().parse();
    // Match the result
    match ans {
        Ok(val) => emp.process_choice(val),
        Err(_) => {
            println!("Only Numbers 1 - 4");
        }
    };

    emp.print_all_employees();

    match emp.save_to_json("./output.json") {
        Ok(()) => {
            println!("Data saved successfully");
        }
        Err(err) => {
            eprintln!("Error saving data: {}", err);
        }
    }
    Ok(())
}
