use std::collections::HashMap;

pub mod mall;
pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (&str, &Store) {
    mall.floors
        .iter()
        .flat_map(|(_, floor)| &(floor.stores))
        .max_by_key(|store| store.1.square_meters)
        .map(|(name, store)| ((*name).as_str(), store))
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, &Employee)> {
    let mut res = mall.floors
        .iter()
        .flat_map(|(_, floor)| &floor.stores)
        .flat_map(|(_, store)| &store.employees)
        .max_by(|a, b| a.1.salary.total_cmp(&b.1.salary))
        .map(|(name, employee)| vec![(name.as_str(), employee)])
        .unwrap();
    println!("---------------------------------------------------------------------------------------------------{:?}",res);
    while res.len() > 1 {
        res.pop();
    }
    res

}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.guards.len()
        + mall
            .floors
            .values()
            .flat_map(|floor| floor.stores.values())
            .map(|store| store.employees.len())
            .sum::<usize>()
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let total_sqm: u64 = mall.floors.values().map(|floor| floor.size_limit).sum();

    let required_guards = ((total_sqm as f64) / 200.0).ceil() as usize;
    let current_guards = mall.guards.len();

    let guards_needed = required_guards.saturating_sub(current_guards);

    let mut available_guards = guards.into_iter();

    for _ in 0..guards_needed {
        if let Some((name, guard)) = available_guards.next() {
            if !mall.guards.contains_key(&name) {
                mall.hire_guard(name, guard);
            }
        } else {
            break;
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let w_h = employee.working_hours.1 - employee.working_hours.0;
                let change = employee.salary * 0.1;
                if w_h >= 10 {
                    employee.salary += change;
                } else {
                    employee.salary -= change;
                }
            }
        }
    }
}
