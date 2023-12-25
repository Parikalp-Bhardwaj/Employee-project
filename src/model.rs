use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

use std::io::Read;
use std::num::ParseIntError;

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub People: Vec<People>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct People {
    pub id: i32,
    pub fname: String,
    pub lname: String,
    pub age: i32,
    pub profession: String,
}

impl List {
    pub fn new() -> Self {
        Self { People: Vec::new() }
    }

    pub fn add_person(&mut self, i_d: i32, f_name: &str, l_name: &str, ages: i32, emp_pro: &str) {
        let ppl = People {
            id: i_d,
            fname: f_name.to_string(),
            lname: l_name.to_string(),
            age: ages,
            profession: emp_pro.to_string(),
        };
        self.People.push(ppl);
    }

    pub fn info_person_by_id(&self, i_d: i32) -> Option<&People> {
        let emp = self.People.iter().find(|val| val.id == i_d);
        emp
    }

    pub fn change_info_by_id(
        &mut self,
        i_d: i32,
        new_fname: &str,
        new_lname: &str,
        new_age: i32,
        new_profession: &str,
    ) -> Result<(), &'static str> {
        if let Some(person) = self.People.iter_mut().find(|val| val.id == i_d) {
            person.fname = new_fname.to_string();
            person.lname = new_lname.to_string();
            person.age = new_age;
            person.profession = new_profession.to_string();
            Ok(())
        } else {
            Err("Employee not found")
        }
    }

    pub fn print_all_employees(&self) {
        println!("{:?} ", self);
    }

    pub fn process_choice(&mut self, val: i32) {
        match val {
            1 => {
                // Add Person logic
                // self.add_person(1, "parikalp", "bhardwaj", 20, "developer");
                let mut new_id = String::new();
                println!("A unique Id: \n");
                io::stdin()
                    .read_line(&mut new_id)
                    .expect("Failed to read lines");
                let i_d = new_id.trim().parse::<i32>();

                let mut new_fname = String::new();

                print!("New First Name: \n");
                io::stdin()
                    .read_line(&mut new_fname)
                    .expect("Failed to read line");

                let result_fname: String =
                    new_fname.chars().filter(|c| c.is_alphabetic()).collect();
                print!("New Last Name: \n");

                let mut new_lname = String::new();
                io::stdin()
                    .read_line(&mut new_lname)
                    .expect("Failed to read line");

                let result_lname: String =
                    new_lname.chars().filter(|c| c.is_alphabetic()).collect();
                print!("New Age: \n");

                let mut new_age = String::new();
                io::stdin()
                    .read_line(&mut new_age)
                    .expect("Failed to read line");

                print!("New Profession: \n");
                let mut new_profession = String::new();
                io::stdin()
                    .read_line(&mut new_profession)
                    .expect("Failed to read line");
                let result_profession: String = new_profession
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect();
                let new_age: i32 = match new_age.trim().parse() {
                    Ok(num) => {
                        self.add_person(
                            i_d.unwrap(),
                            &result_fname,
                            &result_lname,
                            num,
                            &result_profession,
                        );
                        num
                    }
                    Err(_) => {
                        println!("InValid age Using 0 as default");
                        0
                    }
                };
            }
            2 => {
                // Emp Information logic
                println!("Enter an unsigned integer for employee information: ");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let i_d = id.trim().parse::<i32>();

                match i_d {
                    Ok(id_val) => match self.info_person_by_id(id_val) {
                        Some(emp_detail) => {
                            println!("Detail information about person: \n {:#?} \n", emp_detail);
                        }
                        None => {
                            println!("No Person Found ");
                        }
                    },
                    Err(_) => {
                        println!("Something Went Wrong");
                    }
                }
            }
            3 => {
                println!("\nEnter the employee ID to change information:");
                let mut id = String::new();
                io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read lines");
                let i_d = id.trim().parse::<i32>();

                match i_d {
                    Ok(id_val) => {
                        println!("\nEnter the new information:");
                        let mut new_fname = String::new();

                        print!("New First Name: \n");
                        io::stdin()
                            .read_line(&mut new_fname)
                            .expect("Failed to read line");

                        print!("New Last Name: \n");
                        let mut new_lname = String::new();
                        io::stdin()
                            .read_line(&mut new_lname)
                            .expect("Failed to read line");
                        print!("New Age: \n");
                        let mut new_age = String::new();
                        io::stdin()
                            .read_line(&mut new_age)
                            .expect("Failed to read line");

                        print!("New Profession: \n");
                        let mut new_profession = String::new();
                        io::stdin()
                            .read_line(&mut new_profession)
                            .expect("Failed to read line");

                        let new_age: i32 = match new_age.trim().parse() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("InValid age Using 0 as default");
                                0
                            }
                        };
                        match self.change_info_by_id(
                            id_val,
                            new_fname.trim(),
                            new_lname.trim(),
                            new_age,
                            new_profession.trim(),
                        ) {
                            Ok(data) => {
                                println!("Employees information has been changed successfully");
                            }
                            Err(err) => {
                                println!("Error: {}", err);
                            }
                        }
                    }
                    Err(_) => {
                        println!("Invalid input for employee ID");
                    }
                }
            }
            _ => {
                println!("InValid Number ");
            }
        }
    }

    // Read from json data

    pub fn save_to_json(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        // Serialize the data to JSON
        let json_data = serde_json::to_string_pretty(self)?;

        // Create or open the file at the specified path
        let mut file = File::create(file_path)?;

        // Write the JSON data to the file
        file.write_all(json_data.as_bytes())?;

        println!("Data saved to {}", file_path);

        Ok(())
    }

    pub fn load_from_json(&self, file_path: &str) -> Result<List, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;

        let list: List = serde_json::from_str(&json_data)
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

        Ok(list)
    }
}
