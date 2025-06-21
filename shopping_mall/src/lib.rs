pub mod mall;
use std::{collections::HashMap, ops::Index};

pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    mall.floors
        .values()
        .flat_map(|floor| &floor.stores)
        .max_by_key(|(_, store)| store.square_meters)
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut max_salary = 0.0;
    let mut result = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                    result.clear();
                    result.push((name, employee));
                } else if (employee.salary - max_salary).abs() < f64::EPSILON {
                    result.push((name, employee));
                }
            }
        }
    }

    result
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut nbr: usize = mall.guards.len();
    for floor in &mall.floors {
        for store in &floor.1.stores {
            nbr += store.1.employees.len()
        }
    }
    nbr
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut total_size: u64 = 0;

    for floor in mall.floors.values() {
        total_size += floor.size_limit;
    }

    let required_guard_count = total_size / 200;
    let current_guard_count = mall.guards.len() as u64;

    if current_guard_count >= required_guard_count {
        return;
    }

    let needed = required_guard_count - current_guard_count;

    for (key, guard) in guards.into_iter().take(needed as usize) {
        mall.hire_guard(key, guard);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let hours_worked = employee.working_hours.1 - employee.working_hours.0;
                if hours_worked >= 10 {
                    employee.raise(employee.salary * 0.1);
                } else {
                    employee.raise(-(employee.salary * 0.1));
                }
            }
        }
    }
}
