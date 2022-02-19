# random_names
Generates name for a random person, a group of people, or a family.

## Install
Add the following line below the Dependency section in your Cargo.toml file:
```
random_names = "0.1.0"
```

## Usage
```rust
use random_names::RandomNameGenerator;
use random_names::Gender;

let rng = RandomNameGenerator::init();

// Prints a random name composed of first name and last name.
println!("{}", rng.generate()); 

// Prints a random name with a masculine first name.
println!("{}", rng.generate_specific(Gender::Male));

// Returns a vector with 5 random names.
let random_names = rng.generate_many(5);

// Returns a vector with 5 random names with feminine first names.
let random_girls = rng.generate_many_specific(0, 5);

// Returns a vector with 5 random names with similar last name.
// The first and second element are the "parents" and the succeeding
// elements are their "children".
let family = rng.generate_family(3);

// It is also possible to specify the number of male and female children
// in the family. In this example, the family have 5 boys and 1 girl.
let good_luck_courting_her = rng.generate_family_specific(5, 1);
```
For more info, visit the [Cargo page](https://crates.io/crates/random_names) to 
view the official documentation.

## Credits
* First name data (both male and female) from [Mark Kantrowitz](https://www.cs.cmu.edu/afs/cs/Web/Groups/AI/areas/nlp/corpora/names/male.txt).
* Last name data from [smashew's NameDatabases](https://github.com/smashew/NameDatabases/blob/master/NamesDatabases/surnames/us.txt).