use std::{collections::HashMap, fs::File};

use crate::data::{item::Item, perk::Perk};

const STARTING_ITEMS: &[&str] = &["ibuprofen", "makeshift_grenade"];

// Contains all of the data thingies.
#[derive(Debug, Default)]
pub struct Database {
    pub items: HashMap<String, Item>,
    pub perks: HashMap<String, Perk>,
}

impl Database {
    pub fn load() -> Self {
        // Get all items from the ron file, then turn them into a hashmap
        let items_vec: Vec<Item> = ron::de::from_reader(
            File::open("assets/items.ron").expect("Items database not found")
        ).expect("Couldn't read items database");
        
        let items: HashMap<String, Item> = items_vec
            .into_iter()
            .map(|item| (item.key.clone(), item))
            .collect();

        let perks_vec: Vec<Perk> = ron::de::from_reader(
            File::open("assets/perks.ron").expect("Perks database not found")
        ).expect("Couldn't read perks database");
        
        let perks: HashMap<String, Perk> = perks_vec
            .into_iter()
            .map(|perk| (perk.key.clone(), perk))
            .collect();

        Self {
            items,
            perks,
        }
    }
    
    // Used in game_setup to get all the starting items
    pub fn get_starting_items(&self) -> Vec<Item> {
        STARTING_ITEMS.iter()
            .filter_map(|&key| self.items.get(key).cloned())
            .collect()
    }
}
