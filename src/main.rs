#![allow(unused_imports)]

use std::error::Error;
use std::fmt::{self, Debug, Display};
use Volume::*;
use Weight::*;
use std::fs::{read, File};
use std::io::{stdin, BufRead, BufReader};
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
    quantity: f32  ,
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
    let recipes: Vec<Recipe> = read_default_recipes().unwrap();
    println!("Welcome to the recipes CLI program!");
    println!("Here you can find recipes of all kinds, and even create your own!");
    loop {
        println!("What do you want to do?");
        print_main_menu_options();
        let selected = user_select_option();
        println!("Succesful selection: {}", selected);
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

fn user_select_option() -> char {
    loop {
        let mut line = String::new(); 
        let read_result = stdin().read_line(&mut line);
        if let Ok(_result) = read_result {
            let validation_result = validate_user_selection(&mut line);
            match validation_result {
                Ok(character) => return character,
                Err(err) => println!("{}",err),
            }
        } else {
          println!("Line is invalid, please try again!");  
        }
    }
}

fn validate_user_selection(selection: &mut String ) -> Result<char, Box<dyn error::Error>> {
    
    while selection.ends_with('\n') || selection.ends_with('\r') {
        selection.pop();
    }

    if selection.chars().count() > 1 {
        Err(NotACharacterError(selection.clone()))?
    }

    match selection.to_lowercase().chars().next() {
        Some('a') => Ok('a'),
        Some('b') => Ok('b'),
        Some('c') => Ok('c'),
        Some('d') => Ok('d'),
        Some(char) => Err(CharacterNotInSelectionError(char))?,
        None => Err("No character found")?,
    }
}

// PRINTING FUNCTIONS
fn print_main_menu_options(){
    println!("A) See all Recipes");
    println!("B) Add a new recipe");
    println!("C) Remove an existing recipe");
    println!("Press anything else to exit!");
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

