// Goal:
// * A library crate.
// * Generate a random name or an array of random names.
// * Generate a family (An array of names with similar last name).
// * When generating a random name, the following properties must
//   be given control to the user:
//        - Gender of the random person.
// * When generating an array of random names, the following properties must
//   be given control to the user:
//        - Number of people in the array
//        - Number of male and female in the array
// * When generating a family, the following properties must
//   be given control to the user:
//        - Number of children
//        - Number of male and female children.

//! Generates name for a random person, a group of people, or a family. The main
//! component of this library is the `RandomNameGenerator`.

pub mod name_generator;

pub use name_generator::RandomNameGenerator;
pub use name_generator::RandomName;
pub use name_generator::Gender;