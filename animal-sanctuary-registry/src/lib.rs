use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    // TODO: implement this function
    let animals = registry.entry(section.to_string()).or_insert_with(Vec::new);

    if !animals.contains(&animal.to_string()) {
        animals.push(animal.to_string());
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    let mut animals = registry.get(section).cloned().unwrap_or_default();

    animals.sort();

    animals.to_vec()
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    let mut animals: Vec<String> = registry.values().flatten().cloned().collect();

    animals.sort();

    animals
}
