#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Yellow,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_colors_map = std::collections::HashMap::new();
        for color in &self.shirts {
            *num_colors_map.entry(*color).or_insert(0) += 1;
        }
        let (color, _) = num_colors_map
            .iter()
            .max_by_key(|(_, count)| *count)
            .unwrap();
        *color
    }
}

fn demo_1() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue,ShirtColor::Red],
    };

    assert_eq!(inventory.giveaway(Some(ShirtColor::Red)), ShirtColor::Red);


    let user_pref1 = Some(ShirtColor::Yellow);
    let giveaway1 = inventory.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = inventory.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
fn main() {
    demo_1();
}
