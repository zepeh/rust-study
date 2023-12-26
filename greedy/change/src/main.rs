use std::collections::HashMap;

fn main() {
    let money_units = vec![500, 100, 50, 10];
    let price = 4040;
    let money = 5000;
    let change = money - price;

    if change > 0 {
        let change_in_units = convert_to_units(&money_units, change);

        for (&unit, &count) in &change_in_units {
            println!("unit = {}, count = {}", unit, count);
        }
    }
}

fn convert_to_units(money_units: &[i32], money_amount: i32) -> HashMap<i32, i32> {
    let mut money_in_units = HashMap::new();
    let mut remaining_amount = money_amount;

    for &unit  in money_units {
        if remaining_amount == 0 {
            break;
        }

        let count_in_unit = remaining_amount / unit;
        remaining_amount = remaining_amount % unit;
        money_in_units.insert(unit, count_in_unit);
    }

    money_in_units
}