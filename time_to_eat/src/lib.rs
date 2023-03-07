/*Random food generator
*/
use rand::Rng;

//array of breakfast foods
const BREAKFASTS: [&str; 10] = [
    "eggs",
    "toast",
    "pancakes",
    "strawberry",
    "bacon",
    "cereal",
    "fruit",
    "oatmeal",
    "bagel",
    "doughnut",
];

//array of lunch foods
const LUNCHES: [&str; 10] = [
    "burger",
    "hot dog",
    "blt",
    "salad",
    "sushi",
    "wings",
    "tacos",
    "burrito",
    "mac and cheese",
    "grilled cheese",
];

//array of snack foods
const SNACKS: [&str; 10] = [
    "cookie",
    "chips",
    "pretzels",
    "yogurt",
    "nuts",
    "apple",
    "banana",
    "goldfish",
    "trail mix",
    "granola bar",
];

//array of dinner foods
const DINNERS: [&str; 10] = [
    "pizza",
    "fajitas",
    "pasta",
    "steak",
    "salmon",
    "casserole",
    "stir fry",
    "meatloaf",
    "chicken noodle soup",
    "dumplings",
];

//function returns a random breakfast food
pub fn random_breakfast() -> String {
    //randomly select a fruit
    let breakfast = BREAKFASTS[rand::thread_rng().gen_range(0..10)];
    breakfast.to_string()
}

pub fn random_lunch() -> String {
    //randomly select a fruit
    let lunch = LUNCHES[rand::thread_rng().gen_range(0..10)];
    lunch.to_string()
}

pub fn random_snack() -> String {
    //randomly select a snack
    let snack = SNACKS[rand::thread_rng().gen_range(0..10)];
    snack.to_string()
}

pub fn random_dinner() -> String {
    //randomly select a fruit
    let dinner = DINNERS[rand::thread_rng().gen_range(0..10)];
    dinner.to_string()
}
