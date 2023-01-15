use serde::{Deserialize, Serialize};
use std::{fs, vec};
use serde_json::{Value, Map, Number};
use inquire::{Text,validator::{StringValidator,Validation}, InquireError, Select};
use std::process::Command;
use prettytable::{Table,Row,Cell, row,format};

extern crate exitcode;





#[derive(Serialize,Deserialize,Debug)]
struct Workout{
    day: i8,
    workout: String,
    warm_up_sets: i8,
    working_sets: i8,
    reps: String,
    rest: String,
    RPE: String,
    repmax: String,
    status: bool,
    set1: String,
    set2: String,
    set3: String,
    set4: String,
    reps_performed1: String,
    reps_performed2: String,
    reps_performed3: String,
    reps_performed4: String,
    comments: String,
}


#[derive(Serialize,Deserialize,Debug)]
struct OneRM {
    deadlift: i16,
    overheadpress: i16,
    benchpress: i16,
    squat: i16,
    frontsquat: i16
}

#[derive(Serialize,Deserialize,Debug)]
struct Total_Information{
    workout_group: Vec<Workout>,
    onerm: OneRM}



fn main() {


    // Open the json file and populate the working struct.
    let mut workouts = {
        let mut contents = fs::read_to_string("test.json")
        .expect("Couldn't open the file. Check it out.");
        serde_json::from_str::<Total_Information>(&contents)
        .expect("Problem reading the file to json object")
    };    


    let mut positions = (0,0);
    loop{
        let option: Vec<&str> = vec!["Get Current Workout", "See Past Workouts","Input Workout Values","Get Current One Rep Max","Set One Rep Max","Exit"];

        let ans : Result<&str, InquireError> = Select::new("What do you want to do?", option).prompt();

        match ans.unwrap(){
            "Set One Rep Max" => {
                setonerp(&mut workouts);
                clearscreen();
            },
            "Get Current One Rep Max" => {
                get1rm(&mut workouts);
                clearscreen();
            }, 
            "Get Current Workout" => {
                positions= getcurrentworkout_position(&mut workouts);
                printworkout(&mut workouts,positions);
                clearscreen();
            }, 
            "See Past Workouts" =>{
                getpastworkouts(&mut workouts);
                clearscreen();
            },
            "Input Workout Values" => {
                todo!();
                clearscreen();
            },
            "Exit" =>{
                    closeworkout(&mut workouts);
                    std::process::exit(exitcode::OK);                    
                },
            _ => panic!("Error reading your choice")

        }
}

  
    
 
}


fn clearscreen(){
    println!("");
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn setonerp(workouts: &mut Total_Information){
    let mut input = String::new();
        println!("What is the Bench Press 1RM?");
        std::io::stdin().read_line(&mut input).expect("Error with the input");
        workouts.onerm.benchpress = input.trim().parse().expect("Error parsing input.");   
        input.clear();
        println!("What is the Deadlife 1RM?");
        std::io::stdin().read_line(&mut input).expect("Error with the input");
        workouts.onerm.deadlift = input.trim().parse().expect("Error parsing input.");
        input.clear();   
        println!("What is the Overhead Press 1RM?");
        std::io::stdin().read_line(&mut input).expect("Error with the input");
        workouts.onerm.overheadpress = input.trim().parse().expect("Error parsing input.");
        input.clear();   
        println!("What is the Back Squat 1RM?");
        std::io::stdin().read_line(&mut input).expect("Error with the input");
        workouts.onerm.squat = input.trim().parse().expect("Error parsing input.");        
        input.clear();
        println!("What is the Front Squat 1RM?");
        std::io::stdin().read_line(&mut input).expect("Error with the input");
        workouts.onerm.frontsquat = input.trim().parse().expect("Error parsing input.");        
        input.clear();
}

fn closeworkout(workouts: &mut Total_Information){

    _ = std::fs::write(
        "test1.json",
        serde_json::to_string_pretty(&workouts).unwrap(),
    );

}

fn getcurrentworkout_position(workouts: &mut Total_Information) -> (usize,usize){
    let mut start_position: usize= 0;
    let mut end_position: usize= 0;

    for mut n in (0..workouts.workout_group.len()).rev() {
        if workouts.workout_group[n].status {
            start_position = n + 1;
            let mut day: i8 = workouts.workout_group[start_position].day;
            for o in start_position as usize..workouts.workout_group.len(){
                if workouts.workout_group[o].day == day{
                    if o == workouts.workout_group.len()-1{
                        end_position = workouts.workout_group.len()-1;
                    }
                } else {
                    end_position = o -1;
                    break;
                }
            }
            break;
        }
        else{
            for n in 0..workouts.workout_group.len(){
                let mut day: i8 = 1;
                if workouts.workout_group[n].day != day {
                    end_position = n -1;
                    break;
                }

            }
        }
    }
(start_position,end_position)
}

fn get1rm(workouts: &mut Total_Information) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(row!["Workout", "One Rep Max"]);
    table.add_row(row!["Squat",workouts.onerm.squat]);
    table.add_row(row!["Deadlift",workouts.onerm.deadlift]);
    table.add_row(row!["Front Squat",workouts.onerm.frontsquat]);
    table.add_row(row!["Overhead Presss",workouts.onerm.overheadpress]);

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    table.printstd();

}

fn printworkout(workouts: &mut Total_Information, positions: (usize,usize)){
    let mut table = Table::new();
    table.add_row(row!["Workout","Sets","Reps","Target Weight","Rest Time", "Comment"]);
    for n in positions.0..positions.1 {
        let mut  repmax = workouts.workout_group[n as usize].repmax.clone();
    
        if repmax == "N/A" {

            for o in (0..positions.0).rev(){
               if workouts.workout_group[o].workout == workouts.workout_group[n].workout {
                    repmax = workouts.workout_group[o].repmax.clone();
                    break;
                }
            }




            table.add_row(row![
                workouts.workout_group[n as usize].workout.to_lowercase(),
                workouts.workout_group[n as usize].working_sets,
                workouts.workout_group[n as usize].reps,
                repmax,
                workouts.workout_group[n as usize].rest,
                workouts.workout_group[n as usize].comments.to_lowercase()]
            );





        }
        else {
            let temp_rep: Vec<&str> = repmax.split("-").collect();
            if temp_rep.len() > 1 {
                repmax = temp_rep[1].to_string();
                let temp_rep: f32 = repmax.parse::<f32>().unwrap();
                if workouts.workout_group[n as usize].workout.to_lowercase().contains("squat"){
                    repmax = ((temp_rep * (workouts.onerm.squat as f32))/100.0).round().to_string();
                }
                if workouts.workout_group[n as usize].workout.to_lowercase().contains("front squat"){
                    repmax = ((temp_rep * (workouts.onerm.frontsquat as f32))/100.0).round().to_string();
                }
                if workouts.workout_group[n as usize].workout.to_lowercase().contains("deadlift"){
                    repmax = ((temp_rep * (workouts.onerm.deadlift as f32))/100.0).round().to_string();
                }
                if workouts.workout_group[n as usize].workout.to_lowercase().contains("overhead press"){
                    repmax = ((temp_rep * (workouts.onerm.overheadpress as f32))/100.0).round().to_string();
                }
            }




            table.add_row(row![
                workouts.workout_group[n as usize].workout.to_lowercase(),
                workouts.workout_group[n as usize].working_sets,
                workouts.workout_group[n as usize].reps,
                repmax,
                workouts.workout_group[n as usize].rest,
                workouts.workout_group[n as usize].comments.to_lowercase()]);
        }





}





print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
table.printstd();
}

fn getpastworkouts(workouts: &mut Total_Information){

    let mut table = Table::new();
    table.add_row(row!["Workout","Sets","Reps","Performed Weight","Workout Block"]);

    for n in 0..workouts.workout_group.len(){
        if workouts.workout_group[n].status == true {

            table.add_row(row![
                    workouts.workout_group[n].workout.to_ascii_lowercase(),
                    workouts.workout_group[n].working_sets,
                    workouts.workout_group[n].reps,
                    workouts.workout_group[n].repmax ,
                    workouts.workout_group[n].day
                ]);
        }else {
            break;
        }
    }

print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
table.printstd();
}




