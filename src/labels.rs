pub fn card_type_name(card_type: u8) -> &'static str {
    const CARD_TYPES: [&str; 24] = [
        "Dragon",
        "Spellcaster",
        "Zombie",
        "Warrior",
        "Beast-Warrior",
        "Beast",
        "Winged Beast",
        "Fiend",
        "Fairy",
        "Insect",
        "Dinosaur",
        "Reptile",
        "Fish",
        "Sea Serpent",
        "Machine",
        "Thunder",
        "Aqua",
        "Pyro",
        "Rock",
        "Plant",
        "Magic",
        "Trap",
        "Ritual",
        "Equip",
    ];

    CARD_TYPES
        .get(usize::from(card_type))
        .copied()
        .unwrap_or("Unknown")
}

pub fn guardian_star_name(star: u8) -> &'static str {
    const GUARDIAN_STARS: [&str; 10] = [
        "Mars", "Jupiter", "Saturn", "Uranus", "Pluto", "Neptune", "Mercury", "Sun", "Moon",
        "Venus",
    ];

    if star == 0 {
        return "None";
    }

    GUARDIAN_STARS
        .get(usize::from(star - 1))
        .copied()
        .unwrap_or("Unknown")
}

pub fn attribute_name(attribute: u8) -> &'static str {
    const ATTRIBUTES: [&str; 8] = [
        "Light", "Dark", "Earth", "Water", "Fire", "Wind", "Magic", "Trap",
    ];

    ATTRIBUTES
        .get(usize::from(attribute))
        .copied()
        .unwrap_or("Unknown")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_numeric_codes_to_readable_labels() {
        assert_eq!(card_type_name(0), "Dragon");
        assert_eq!(card_type_name(23), "Equip");
        assert_eq!(card_type_name(99), "Unknown");
        assert_eq!(guardian_star_name(1), "Mars");
        assert_eq!(guardian_star_name(8), "Sun");
        assert_eq!(guardian_star_name(0), "None");
        assert_eq!(attribute_name(0), "Light");
        assert_eq!(attribute_name(7), "Trap");
    }
}
