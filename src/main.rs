#![allow(unused_imports)]

use std::any::Any;
use std::char::ToLowercase;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{self, Debug, DebugStruct, Display};
use std::path::{self, Path, PathBuf};
use std::fs::{self, read, File};
use std::io::{stdin, BufRead, BufReader, Read, Write};
use std::ptr::null;
use std::{char, default, error, io, vec};

use serde::{Deserialize, Serialize};

use Volume::*;
use Weight::*;

#[derive(Debug, Deserialize, Serialize)]
struct Recipe {
    name: String,
    description: String,
    ingredients: Vec<Ingredient>,
    steps: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Ingredient{
    name: String,
    quantity: f32,
    unit: Unit
}
#[derive(Debug, Serialize, Deserialize)]
enum Unit {
    Volume(Volume),
    Weight(Weight),
    Piece,
}
#[derive(Debug, Serialize, Deserialize)]
enum Volume{
    Cup,
    Ounce,
    Teaspoon,
    Tablespoon,
    Milliliter,
    Liter,
}
#[derive(Debug, Serialize, Deserialize)]
enum Weight {
    Milligram,
    Gram,
    Decagram,
    Kilogram,
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Weight::Kilogram => write!(f, "Kilogram"),
            Weight::Decagram => write!(f, "Decagram"),
            Weight::Milligram => write!(f, "Milligram"),
            Weight::Gram => write!(f, "Gram"),
        }
    }
}
impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Volume::Liter => write!(f, "Liter"),
            Volume::Milliliter => write!(f, "Milliliter"),
            Volume::Cup => write!(f, "Cup"),
            Volume::Ounce => write!(f, "Ounce"),
            Volume::Tablespoon => write!(f, "Tablespoon"),
            Volume::Teaspoon => write!(f, "Teaspoon"),
        }
    }
}
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unit::Volume(volume) => write!(f,"{}",volume),
            Unit::Weight(weight) => write!(f,"{}",weight),
            Unit::Piece => write!(f,"Piece"),
        }
    }
}
impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.quantity, self.unit)
    }
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        str.push_str(&self.name);
        str.push('\n');
        str.push_str(&self.description);
        str.push('\n');
        for ingr in &self.ingredients {
            str.push_str(&ingr.to_string());
            str.push('\n');
        }
        for step in &self.steps {
            str.push_str(step);
            str.push('\n');
        }
        write!(f, "{}",str)
    }
}

enum MainMenuOption{
    ShowAllRecipes,
    AddNewRecipe,
    RemoveRecipe,
    Exit
}

#[derive(Debug,Clone)]
struct CharacterNotInSelectionError(char);
impl fmt::Display for CharacterNotInSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Character not in selection: {}!", self.0)
    }
}

impl error::Error for CharacterNotInSelectionError{}

#[derive(Debug,Clone)]
struct NotACharacterError<T>(T);
impl<T : Display> fmt::Display for NotACharacterError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not a character: {}", self.0)
    }
}
impl<T:Debug + Display> error::Error for NotACharacterError<T> {}

fn main() {
    let mut recipes: Vec<Recipe> = read_default_recipes().unwrap();
    println!("Welcome to the recipes CLI program!");
    println!("Here you can find recipes of all kinds, and even create your own!");
    loop {
        display_main_menu();
        handle_user_choice(get_user_choice(), &mut recipes);
    }
}

fn get_json_files_in_folder(path: &str) -> Result<Vec<PathBuf>, Box<dyn error::Error>> {
    let json_paths = 
        // Read the directory's entries
        fs::read_dir(path)?
        // Filter all invalid entries
        .filter_map(|result| result.ok())
        // Convert all directory entries to paths
        .map(|entry| entry.path())
        // Filter the ones that have a JSON extension
        .filter_map(|path| {
            if path
                .extension()
                .map_or(false, |exten| exten == "json") {
                    Some(path)
            } else {
                None // filter_map will take care of None values
            }
        })
        .collect::<Vec<PathBuf>>();
    Ok(json_paths)
}


fn string_to_unit(string: &str) -> Option<Unit> {
    match string.to_lowercase().as_str() {
        "piece" | "pieces" => Some(Unit::Piece),
        "cup" | "cups" => Some(Unit::Volume(Volume::Cup)),
        "teaspoon" | "teaspoons" => Some(Unit::Volume(Volume::Teaspoon)),
        "tablespoon" | "tablespoons" => Some(Unit::Volume(Volume::Tablespoon)),
        "liter" | "liters" => Some(Unit::Volume(Volume::Liter)),
        "ounce" | "ounces" => Some(Unit::Volume(Volume::Ounce)),
        "milliliter" | "milliliters" => Some(Unit::Volume(Volume::Milliliter)),
        "gram" | "grams" => Some(Unit::Weight(Weight::Gram)),
        "milligram" | "milligrams" => Some(Unit::Weight(Weight::Milligram)),
        "decagram" | "decagrams" => Some(Unit::Weight(Weight::Decagram)),
        "kilogram" | "kilograms" => Some(Unit::Weight(Weight::Kilogram)),
        _ => None, 
    }
    
}

fn read_default_recipes() -> Result<Vec<Recipe>, Box<dyn std::error::Error>>{
    let file_path = "./recipes";
    let json_file_paths = get_json_files_in_folder(file_path)?;

    let mut recipes = vec![];

    for json_file_path in json_file_paths {
        recipes.push(read_recipe_from_json(json_file_path)?);
    }
    Ok(recipes)
}   


fn read_recipe_from_json<P: AsRef<Path>>(path: P) -> Result<Recipe, Box<dyn  error::Error>> {
 
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok (
        serde_json::from_reader(reader)?
    )
}

fn handle_user_choice(choice : MainMenuOption, recipes: &mut Vec<Recipe>) {
    
    match choice {
        MainMenuOption::ShowAllRecipes => print_recipes(recipes),
        MainMenuOption::AddNewRecipe => {
            let new_recipe = collect_recipe_from_user();
            let recipe_adding_result = add_recipe_json(&new_recipe); 
            if recipe_adding_result.is_ok() {
                recipes.push(new_recipe);
            }
            else {
                println!("Something went wrong... Recipe could not be added to the file!");
                println!("{:?}", recipe_adding_result);
            }
        }
        MainMenuOption::RemoveRecipe => {
            println!("Please enter the name of the recipe!");
            let name : &mut String = &mut String::new();
            while let Err(error) = stdin().read_line(name) {
                println!("Please try entering the name again: {}",error);
            };
            let remove_result = remove_recipe(name.trim(), recipes);
            if remove_result.is_ok() {
                println!("Succesfully deleted!");
            } else {
                println!("{:?}",remove_result);
            }
        },
        MainMenuOption::Exit => std::process::exit(0),
    }   
}

/*
*/

fn remove_recipe(name : &str, recipes:  &mut Vec<Recipe>) -> std::result::Result<(), Box<dyn std::error::Error>>{
    
    for (index, recipe) in recipes.iter().enumerate() {
        if recipe.name.to_lowercase().eq(&name.to_lowercase()) {
            delete_recipe_json(recipe)?;
            recipes.remove(index);
            return Ok(());
        }
    }
    Err("Recipe not found")?
}

fn delete_recipe_json(recipe : &Recipe) -> std::result::Result<(), Box<dyn std::error::Error>>{
    let file_path = "./recipes/";
    let file_name = 
        &recipe.name
        .to_lowercase()
        .replace(' ', "_");
    
    println!("File name: {}.json", file_name);
    fs::remove_file(format!("{}{}.json",file_path,file_name))?;
    Ok(())
}

fn add_recipe_json(recipe: &Recipe) -> std::result::Result<(), Box<dyn error::Error>> {
    let file_name = 
        recipe.name
        .to_lowercase()
        .replace(' ', "_");
    let file_path = format!("./recipes/{}.json",file_name);
    let json = serde_json::to_string(recipe)?;
    
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

 
fn collect_recipe_from_user() -> Recipe {
    println!("Please enter the name of the recipe!");
    let name : &mut String = &mut String::new();
    while let Err(error) = stdin().read_line(name){
        println!("Please try entering the name again: {}",error);
    };

    println!("Please enter a description for the recipe! (Must be only 1 line)");
    let description : &mut String = &mut String::new();
    while let Err(error) = stdin().read_line(description){
        println!("Please try entering the description again: {}",error);
    };

    println!("Please enter the number of ingredients!");
    let mut number_of_ingredients : u32 = 0;
    let number_of_ingredients_string : &mut String = &mut String::new();
    let mut is_valid = false;
    while !is_valid {
        while let Err(error) = stdin().read_line(number_of_ingredients_string){
            println!("Please try entering the number again: {}", error);
        };
        match number_of_ingredients_string.trim().parse::<u32>(){
            Ok(number) => {
                number_of_ingredients = number;
                is_valid = true;
            },
            Err(error) => println!("There was an error: {}, please try entering the number again!",error)
        };
    };
    

    let mut ingredients : Vec<Ingredient> = vec![];
    for _ in 1..=number_of_ingredients {
        println!("Please enter the ingredient! (Ingredient names cannot have whitespaces. Use \"_\" instead");
        println!("Ingredient must have the following format:\nName Quantity Unit");
        ingredients.push(read_ingredient());
    }

    is_valid = false;
    let mut number_of_steps_string = String::new();
    let mut number_of_steps = 0;
    println!("Please enter the number of steps!");
    while !is_valid {
        while let Err(error) = stdin().read_line(&mut number_of_steps_string){
            println!("Please try entering the number again: {}", error);
        };
        match number_of_steps_string.trim().parse::<u32>(){
            Ok(number) => {
                number_of_steps = number;
                is_valid = true;
            },
            Err(error) => println!("There was an error: {}, please try entering the number again!",error)
        };
    };
    
    let mut steps : Vec<String> = vec![];
    for index in 1..=number_of_steps {
        println!("Please enter step number {}", index);
        steps.push(read_step().trim().to_string());
    }
    return Recipe {
        name: String::from(name.as_str().trim()),
        description: String::from(description.as_str().trim()),
        ingredients,
        steps
    }


}

fn read_ingredient() -> Ingredient {
    loop {
        let mut ingredient_line_buf = String::new();
        while let Err(error) = stdin().read_line(&mut ingredient_line_buf){
            println!("Please try entering the ingredient again: {}",error);
        };
        let mut split_ingr_buf = ingredient_line_buf.split_whitespace();
        if split_ingr_buf.clone().count() != 3 {
            println!("Please enter a valid line!")
        } else {
            let name_option = split_ingr_buf.next();
            let quantity_option = split_ingr_buf.next();
            let unit_option = split_ingr_buf.next();
            if let (Some(name), Some(unit_str), Some(quantity_str)) 
                = (name_option, unit_option, quantity_option) {
                if let (Some(unit), Ok(quantity)) = 
                    (string_to_unit(unit_str), quantity_str.parse::<f32>()){
                        return Ingredient {
                            name: name.to_string(),
                            quantity,
                            unit
                        };
                } else {
                    println!("Unit or quanity format incorrect!");
                    continue;
                }
            } else {
                println!("Ingredient detail missing!");
                continue;
            }
        }
    };
}

fn read_step() -> String {
    loop {
        let mut buf = String::new();
        if stdin().read_line(&mut buf).is_ok(){
            return buf;
        } else {
            println!("Step format wrong! Please try again!");
        }
    }
}

// TODO (oliver): Make a way to exit the program. Perhaps use ESC key.
fn get_user_choice() -> MainMenuOption {
    loop {
        let mut line = String::new(); 
        let read_result = stdin().read_line(&mut line);
        if let Ok(_result) = read_result {
            let validation_result = validate_user_selection(&mut line);
            match validation_result {
                Ok(choice) => return choice,
                Err(err) => println!("{}",err),
            }
        } else {
          println!("Line is invalid, please try again!");  
        }
    }
}

fn validate_user_selection(selection: &mut String ) -> std::result::Result<MainMenuOption, Box<dyn error::Error>> {
    
    while selection.ends_with('\n') || selection.ends_with('\r') {
        selection.pop();
    }

    if selection.chars().count() > 1 {
        Err(NotACharacterError(selection.clone()))?
    }

    match selection.to_lowercase().chars().next() {
        Some('a') => Ok(MainMenuOption::ShowAllRecipes),
        Some('b') => Ok(MainMenuOption::AddNewRecipe),
        Some('c') => Ok(MainMenuOption::RemoveRecipe),
        Some('q') => Ok(MainMenuOption::Exit),
        Some(char) => Err(CharacterNotInSelectionError(char))?,
        None => Err("No character found")?,
    }
}

// PRINTING FUNCTIONS
fn display_main_menu(){
    println!("What do you want to do?");
    println!("A) See all Recipes");
    println!("B) Add a new recipe");
    println!("C) Remove an existing recipe");
    println!("Q) Exit");
}

fn print_recipes(recipes: &Vec<Recipe>){
    for recipe in recipes{
        println!("{}",recipe.name);
        println!("{}",recipe.description);
        for ingredient in &recipe.ingredients {
            println!("{}",ingredient);
        }
        for step in &recipe.steps {
            println!("{}", step);
        }
        println!();
    }
}