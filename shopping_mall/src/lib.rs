pub mod mall;
pub use mall::*;

pub fn biggest_store(mall: &Mall) -> Store {
    mall.floors
        .iter()
        .flat_map(|floor| (floor.1).stores.clone())
        .map(|store| store.1)
        .max_by_key(|store| store.square_meters)
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<Employee> {
    let mall_iter = mall
        .floors
        .iter()
        .flat_map(|floor| (floor.1).stores.clone())
        .flat_map(|store| store.1.employees)
        .map(|employee|  employee.1)
        .max_by_key(|employee| (employee.salary*100.0) as i64)
        .unwrap();
    let res : Vec<Employee> = vec![mall_iter];
    res
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.guards.len()
        + mall
            .floors
            .iter()
            .flat_map(|floor| (floor.1).stores.clone())
            // .flat_map(|floor| floor.1.stores)
            .fold(0_usize, |acc, x| acc + x.1.employees.len())
}


pub fn check_for_securities(mall: &mut Mall, guards: Vec<(String, Guard)>) {
    // Calculate total square meters across all floors
    let total_sqm: u64 = mall.floors
        .values()
        .map(|floor| floor.size_limit)
        .sum();

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
            break; // no more available guards
        }
    }
}
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.iter_mut() {
        for store in floor.1.stores.iter_mut() {
            for employee in store.1.employees.iter_mut() {
                let employee =  employee.1;
                let w_h = employee.working_hours.1 - employee.working_hours.0;
                let percentage = employee.salary * 0.1;
                if w_h > 10 {
                    employee.salary += percentage
                } else {
                    employee.salary -= percentage
                }
            }
        }
    }
}