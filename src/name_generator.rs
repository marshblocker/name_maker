use rand::Rng;
use std::fmt;

/// Determines if the first name of the random name is
/// masculine or feminine.
#[derive(PartialEq)]
pub enum Gender {
    Male,
    Female
}

/// Stores a list of names with similar type (e.g. all are first names or all
/// last names).
pub struct NameBank<'a> {
    bank: Vec<&'a str>,
    len: usize
}

/// Contains the list of first names (separated by gender) and last names (surnames).
/// It also contains methods for generating random name/s.
/// 
/// # Example
/// ```
/// use name_maker::RandomNameGenerator;
/// use name_maker::Gender;
/// 
/// let rng = RandomNameGenerator::init();
/// 
/// // Prints a random name composed of first name and last name.
/// println!("{}", rng.generate()); 
/// 
/// // Prints a random name with a masculine first name.
/// println!("{}", rng.generate_specific(Gender::Male));
/// 
/// // Returns a vector with 5 random names.
/// let random_names = rng.generate_many(5);
/// 
/// // Returns a vector with 5 random names with feminine first names.
/// let random_girls = rng.generate_many_specific(0, 5);
/// 
/// // Returns a vector with 5 random names with similar last name.
/// // The first and second element are the "parents" and the succeeding
/// // elements are their "children".
/// let family = rng.generate_family(3);
/// 
/// // It is also possible to specify the number of male and female children
/// // in the family. In this example, the family have 5 boys and 1 girl.
/// let good_luck_courting_her = rng.generate_family_specific(5, 1);
/// ```
pub struct RandomNameGenerator<'a> {
    male_first_names: NameBank<'a>,
    female_first_names: NameBank<'a>,
    last_names: NameBank<'a>
}

impl<'a> RandomNameGenerator<'a> {
    /// Initializes the vectors that contain the names needed by the library.
    pub fn init() -> RandomNameGenerator<'a> {
        let mut male_first_names_bank: Vec<&str> = Vec::new();
        let male_first_names_raw: &'static str = include_str!("male_first_names.txt");

        for male_first_name in male_first_names_raw.lines() {
            male_first_names_bank.push(male_first_name.trim());
        }

        let len = male_first_names_bank.len();

        let male_first_names = NameBank { 
            bank: male_first_names_bank, len 
        };

        let mut female_first_names_bank: Vec<&str> = Vec::new();
        let female_first_names_raw: &'static str = include_str!("female_first_names.txt");

        for female_first_name in female_first_names_raw.lines() {
            female_first_names_bank.push(female_first_name.trim());
        }

        let len = female_first_names_bank.len();

        let female_first_names = NameBank { 
            bank: female_first_names_bank, len 
        };

        let mut last_names_bank: Vec<&str> = Vec::new();
        let last_names_raw: &'static str = include_str!("last_names.txt");

        for last_name in last_names_raw.lines() {
            last_names_bank.push(last_name.trim());
        }

        let len = last_names_bank.len();

        let last_names = NameBank { 
            bank: last_names_bank, len 
        };

        RandomNameGenerator { male_first_names, female_first_names, last_names }
    }

    /// Returns a random name with a random gender.
    pub fn generate(&self) -> RandomName {
        let gender = Self::get_random_gender();

        self.generate_specific(gender)
    }

    /// Returns a random name. Its gender must be specified.
    pub fn generate_specific(&self, gender: Gender) -> RandomName{
        let first_name = self.generate_first_name_specific(gender);
        let last_name = self.generate_last_name();

        RandomName { first_name, last_name }
    }

    /// Returns a vector of random names with random genders. If
    /// `amount` is initialized with `0`, then this returns `None`.
    pub fn generate_many(&self, amount: u32) -> Option<Vec<RandomName>> {
        if amount == 0 { return None; }

        let mut random_names = Vec::new();
        
        for _ in 0..amount {
            random_names.push(self.generate());
        }

        Some(random_names)
    }

    /// Returns a vector of random names. The number of male and female names
    /// must be specified. If `male_amount` and `female_amount` are both initialized
    /// to `0`, then this returns `None`. Note that the vector is initially populated
    /// with random male names before the random female names. Hence, it is
    /// easy to separate both genders if needed.
    pub fn generate_many_specific(
        &self, 
        male_amount: u32, 
        female_amount: u32
    ) -> Option<Vec<RandomName>> {
        if male_amount == 0 && female_amount == 0 { return None; }

        let mut random_names = Vec::new();

        for _ in 0..male_amount {
            random_names.push(self.generate_specific(Gender::Male));
        }

        for _ in 0..female_amount {
            random_names.push(self.generate_specific(Gender::Female));
        }

        Some(random_names)
    }

    /// Returns a vector of random names with same last name.
    /// Note that, unlike `generate_many` and `generate_many_specific`, 
    /// `generate_family` does not return `None` even if `children_amount` is initialized
    /// with `0`. This is because two random names, the "father" and the "mother",
    /// are inserted to the vector before it is populated with the "children".
    pub fn generate_family(&self, children_amount: u32) -> Vec<RandomName> {
        let mut random_family = Vec::new();

        let family_last_name = self.generate_last_name();

        let father = self.generate_family_member(family_last_name.clone(), Gender::Male);
        let mother = self.generate_family_member(family_last_name.clone(), Gender::Female);
        
        random_family.push(father);
        random_family.push(mother);

        for _ in 0..children_amount {
            let gender = Gender::Male;
            let child = self.generate_family_member(family_last_name.clone(), gender);
            
            random_family.push(child);
        }

        random_family
    }

    /// Similar to `generate_family_specific`, the only difference is that the number
    /// of male and female children must be specified. Also, similar to `generate_many_specific`,
    /// the male children are generated first before the female children.
    pub fn generate_family_specific(
        &self, 
        male_children_amount: u32, 
        female_children_amount: u32
    ) -> Vec<RandomName> {
        let mut random_family = Vec::new();

        let family_last_name = self.generate_last_name();

        let father = self.generate_family_member(family_last_name.clone(), Gender::Male);
        let mother = self.generate_family_member(family_last_name.clone(), Gender::Female);

        random_family.push(father);
        random_family.push(mother);

        for _ in 0..male_children_amount {
            let child = self.generate_family_member(family_last_name.clone(), Gender::Male);
            
            random_family.push(child);
        }

        for _ in 0..female_children_amount {
            let child = self.generate_family_member(family_last_name.clone(), Gender::Female);
            
            random_family.push(child);
        }

        random_family
    }

    /// Returns a default name. This can be used as a placeholder.
    /// 
    /// # Example
    /// ```
    /// use name_maker::RandomNameGenerator;
    /// 
    /// let default = RandomNameGenerator::generate_default_name();
    /// 
    /// assert_eq!("John".to_string(), default.first_name);
    /// assert_eq!("Doe".to_string(), default.last_name);
    /// 
    /// // Prints "John Doe"
    /// println!("{}", default);
    /// ```
    pub fn generate_default_name() -> RandomName {
        RandomName {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        }
    }

    fn get_random_gender() -> Gender {
        if rand::random() {
            Gender::Male
        } else {
            Gender::Female
        }
    }

    fn get_rand_index(len: usize) -> usize {
        rand::thread_rng().gen_range(0..len)
    }

    fn generate_first_name_specific(&self, gender: Gender) -> String {
        let index: usize;
        let first_name: String;

        if gender == Gender::Male {
            index = Self::get_rand_index(self.male_first_names.len);
            first_name = self.male_first_names.bank
                .get(index)
                .unwrap()
                .to_string();
        } else {
            index = Self::get_rand_index(self.female_first_names.len);
            first_name = self.female_first_names.bank
                .get(index)
                .unwrap()
                .to_string();
        }

        first_name
    }

    fn generate_last_name(&self) -> String {
        let index = Self::get_rand_index(self.last_names.len);
        let last_name = self.last_names.bank
            .get(index)
            .unwrap()
            .to_string();

        last_name
    }

    fn generate_family_member(&self, family_last_name: String, gender: Gender) -> RandomName {
        RandomName {
            first_name: self.generate_first_name_specific(gender),
            last_name: family_last_name
        }
    }
}


/// Contains the first name component and the last name component of a random
/// name generated by the [`RandomNameGenerator`].
#[derive(Debug)]
pub struct RandomName {
    pub first_name: String,
    pub last_name: String
}

impl fmt::Display for RandomName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}
