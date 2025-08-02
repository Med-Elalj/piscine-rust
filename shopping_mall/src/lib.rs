use std::collections::HashMap;
mod mall;
pub use mall::{Mall, Store, Employee, Guard, Floor};


pub fn biggest_store(mall: &Mall) -> (String, Store) {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|(_, store)| store.square_meters)
        .map(|(name, store)| (name.clone(), store.clone()))
        .expect("Mall should have at least one store")
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let max_salary = mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .flat_map(|store| store.employees.values())
        .map(|employee| employee.salary)
        .fold(f64::MIN, f64::max);

    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .flat_map(|(_store_name, store)| store.employees.iter())
        .filter(|(_name, employee)| employee.salary == max_salary)
        .map(|(name, employee)| (name.clone(), *employee))
        .collect()
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let employees_count: usize = mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.employees.len())
        .sum();

    let guards_count = mall.guards.len();

    employees_count + guards_count
}

pub fn check_for_securities(mall: &mut Mall, available_guards: HashMap<String, Guard>) {
    let total_floor_size: u64 = mall.floors.values().map(|f| f.size_limit).sum();

    let required_guards = ((total_floor_size as f64) / 200.0).ceil() as usize;

    let current_guards = mall.guards.len();

    if current_guards >= required_guards {
        return;
    }
    let to_add = required_guards - current_guards;
    let mut added = 0;
    for (name, guard) in available_guards {
        if !mall.guards.contains_key(&name) {
            mall.guards.insert(name, guard);
            added += 1;
            if added == to_add {
                break;
            }
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let hours = employee.working_hours.1.saturating_sub(employee.working_hours.0);
                if hours >= 10 {
                    let raise_amount = employee.salary * 0.10;
                    employee.raise(raise_amount);
                } else {
                    let cut_amount = employee.salary * 0.10;
                    employee.cut(cut_amount);
                }
            }
        }
    }
}
