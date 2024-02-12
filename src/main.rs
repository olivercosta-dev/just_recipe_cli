#![allow(unused_imports)]

use std::char::ToLowercase;
use std::error::Error;
use std::fmt::{self, Debug, DebugStruct, Display};
use Volume::*;
use Weight::*;
use std::fs::{self, read, File};
use std::io::{stdin, BufRead, BufReader, Write};
use std::{char, default, error, io, vec};


#[derive(Debug)]
struct Recipe {
    name: String,
    description: String,
    ingredients: Vec<Ingredient>,
    steps: Vec<String>,
}
#[derive(Debug)]
struct Ingredient{
    name: String,
    quantity: f32,
    unit_of_measurement: Unit
}
#[derive(Debug)]
enum Unit {
    Volume(Volume),
    Weight(Weight),
    Piece,
}
#[derive(Debug)]
enum Volume{
    Cup,
    Ounce,
    Teaspoon,
    Tablespoon,
    Milliliter,
    Liter,
}
#[derive(Debug)]
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
        write!(f, "{} {} {}", self.name, self.quantity, self.unit_of_measurement)
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

fn string_to_unit(string: &str) -> Option<Unit>{
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

fn read_default_recipes() -> Result<Vec<Recipe>, Box<dyn error::Error>> {
    
    let file_path = "./recipes/recipes.txt";
   
    let file = File::open(&file_path)?;
    
    let reader = BufReader::new(file);
   
    let mut recipes: Vec<Recipe> = vec![];
    let mut lines = reader.lines();
    while let Some(first_line) = lines.next(){
        let name = first_line?;

        let description = match lines.next(){
            Some(result) => result?,
            None => Err("No description found for this recipe!")?
        };

        let number_of_ingredients = match lines.next() {
            Some(n) => n?.parse::<u32>()?,
            None => Err("No ingredient quantity found for this recipe!")?
        };

        let mut ingredients: Vec<Ingredient> = vec![];

        for _ in 0..number_of_ingredients {
            
            let ingredient_details_string = match lines.next() {
                Some(ingredient_details_result) =>  ingredient_details_result?,
                None => Err("Ingredient number set does not match the actual ingredient count!")?
            };

            let mut ingredient_details = ingredient_details_string.split_whitespace();

            let ingredient : Ingredient = Ingredient {
                name: 
                    match ingredient_details.next() {
                        Some(ingr_name) => ingr_name.to_string(),
                        None => Err("Ingredient does not have a name!")?
                    },
                quantity: 
                    match ingredient_details.next(){
                        Some(quantity_string) => quantity_string.parse::<f32>()?,
                        None => Err("Ingredient does not have a quantity!")?
                    },
                unit_of_measurement: 
                    match ingredient_details.next(){
                        Some(unit_string) => {
                            match string_to_unit(unit_string){
                                Some(unit) => unit,
                                None => Err("Ingredient unit given invalid!")?,
                            }
                        },
                        None => Err("Ingredient does not have a unit!")?
                    }

            };
            ingredients.push(ingredient);
        }
        let step_count = match lines.next() {
            Some(n) => n?.parse::<u32>()?,
            None => Err("No step count found for this recipe!")?
        };
        
        let mut steps: Vec<String> = vec![];

        for _ in 0..step_count {
            match lines.next() {
                Some(step) => steps.push(step?),
                None => Err("Step count set does not match the actual ingredient count!")?
            }
        }
        
        let recipe:Recipe = Recipe {
            name,
            description,
            ingredients,
            steps
        };
        recipes.push(recipe);
    }
    
    Ok(recipes)
}

fn handle_user_choice(choice : MainMenuOption, recipes: &mut Vec<Recipe>) {
    
    match choice {
        MainMenuOption::ShowAllRecipes => print_recipes(recipes),
        MainMenuOption::AddNewRecipe => {
            let new_recipe = collect_recipe_from_user();
            if add_recipe_to_file(&new_recipe).is_ok() {
                recipes.push(new_recipe);
            } else {
                println!("Something went wrong.... Recipe could not be added to the file!");
            }
        }
        MainMenuOption::RemoveRecipe => println!("Coming soon"),
        MainMenuOption::Exit => println!("Coming soon"),
    }   
}
fn add_recipe_to_file(recipe : &Recipe) -> Result<(),Box<dyn Error>>{
    let mut recipe_data = String::new(); 
    recipe_data.push_str(&recipe.name);
    recipe_data.push('\n');

    recipe_data.push_str(&recipe.description);
    recipe_data.push('\n');
    
    recipe_data.push_str(&recipe.ingredients.len().to_string());
    recipe_data.push('\n');
    
    for ingredient in &recipe.ingredients{
        recipe_data.push_str(&format!("{}",ingredient));
    }
    recipe_data.push('\n');

    recipe_data.push_str(&recipe.steps.len().to_string());
    recipe_data.push('\n');

    for step in &recipe.steps {
        recipe_data.push_str(step);
        recipe_data.push('\n');
    }
    recipe_data.pop();

    let mut file  = fs::OpenOptions::new()
       .create(true)
       .append(true)
       .open("./recipes/recipes.txt")?;
    file.write_all(recipe_data.as_bytes())?;
    Ok(())
}


fn collect_recipe_from_user() -> Recipe {
    println!("Please enter the name of the recipe!");
    let mut name : &mut String = &mut String::new();
    while let Err(error) = stdin().read_line(&mut name){
        println!("Please try entering the name again: {}",error);
    };

    println!("Please enter a description for the recipe! (Must be only 1 line)");
    let mut description : &mut String = &mut String::new();
    while let Err(error) = stdin().read_line(&mut description){
        println!("Please try entering the description again: {}",error);
    };

    println!("Please enter the number of ingredients!");
    let mut number_of_ingredients : u32 = 0;
    let mut number_of_ingredients_string : &mut String = &mut String::new();
    let mut is_valid = false;
    while !is_valid {
        while let Err(error) = stdin().read_line(&mut number_of_ingredients_string){
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
    for _ in 0..number_of_ingredients {
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
        match number_of_ingredients_string.trim().parse::<u32>(){
            Ok(number) => {
                number_of_steps = number;
                is_valid = true;
            },
            Err(error) => println!("There was an error: {}, please try entering the number again!",error)
        };
    };
    let mut steps : Vec<String> = vec![];
    for _ in 0..number_of_steps {
        steps.push(read_step());
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
                            unit_of_measurement: unit
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
        if let Ok(_) = stdin().read_line(&mut buf){
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

fn validate_user_selection(selection: &mut String ) -> Result<MainMenuOption, Box<dyn error::Error>> {
    
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
            println!("{:?}",ingredient);
        }
        println!();
    }
}

