use std::collections::HashMap;

pub mod mall;
pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    mall.floors
        .iter()
        .flat_map(|(_, floor)| floor.stores.iter())  // iter over (&String, &Store)
        .max_by_key(|(_, store)| store.square_meters)
        .unwrap()
}


pub fn highest_paid_employee(mall: &Mall) -> Vec<&Employee> {
    let employees = mall
        .floors
        .iter()
        .flat_map(|(_, floor)| floor.stores.iter())
        .flat_map(|(_, store)| store.employees.iter())
        .map(|(_, employee)| employee);

    let max_salary = employees.clone().map(|e| e.salary).fold(0.0, f64::max);

    employees
        .filter(|e| e.salary == max_salary)
        .collect()
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