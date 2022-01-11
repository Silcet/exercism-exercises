use strum::{EnumIter, IntoEnumIterator};

pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Clone, EnumIter)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let allergies = Allergen::iter()
            .enumerate()
            .filter_map(|(index, allergen)| {
                if (0b1 << index) & score > 0 {
                    Some(allergen)
                } else {
                    None
                }
            })
            .collect();

        Allergies { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
