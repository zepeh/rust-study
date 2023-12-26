struct Stuff {
    name: String,
    weight: f32,
    price: i32,
}

fn main() {
    let weight = 35.0f32;

    let mut stuffs = vec![
        Stuff{
            name: String::from("Wine"),
            price: 1000,
            weight: 10.0
        },
        Stuff{
            name: String::from("Ring"),
            price: 2500,
            weight: 3.0
        },
        Stuff{
            name: String::from("Computer"),
            price: 1500,
            weight: 10.0
        },
        Stuff{
            name: String::from("Picture"),
            price: 3000,
            weight: 30.0
        }];

    let stuffs_in_backpack = fill_backpack(weight, &mut stuffs);

    for stuff in &stuffs_in_backpack {
        println!("{}", stuff.name);
    }

    assert_eq!(vec!["Computer", "Wine"], stuffs
        .iter()
        .map(|s| s.name.clone())
        .collect::<Vec<_>>());

    assert_eq!(vec!["Picture", "Ring"], stuffs_in_backpack
        .iter()
        .map(|s| s.name.clone())
        .collect::<Vec<_>>());
}

fn fill_backpack(weight: f32, stuffs: &mut Vec<Stuff>) -> Vec<Stuff> {
    let mut stuffs_in_backpack : Vec<Stuff> = vec![];
    let mut remaining_weight = weight;
    let mut remaining_stuffs: Vec<Stuff> = vec![];

    while !stuffs.is_empty() {
        remaining_stuffs.push(stuffs.remove(0));
    }

    remaining_stuffs.sort_by(|a, b| b.price.cmp(&a.price));

    while remaining_weight > 0f32 && !remaining_stuffs.is_empty() {
        let stuff = remaining_stuffs.remove(0);

        if remaining_weight < stuff.weight {
            stuffs.push(stuff);
        } else {
            remaining_weight -= stuff.weight;
            stuffs_in_backpack.push(stuff);
        }
    }

    stuffs_in_backpack
}
