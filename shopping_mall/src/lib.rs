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

pub fn highest_paid_employee(mall: &Mall) -> Vec<Employee> {
    let mut maxsalair: f64 = 0.;
    let mut vec: Vec<Employee> = vec![];
    for (floor_name, floor) in &mall.floors {
        for (store_name, store) in &floor.stores {
            for (employe) in &store.employees {
                if employe.1.salary > maxsalair {
                    maxsalair = employe.1.salary
                }
            }
        }
    }

    for (floor_name, floor) in &mall.floors {
        for (store_name, store) in &floor.stores {
            for (employe) in &store.employees {
                if employe.1.salary == maxsalair {
                    vec.push(*employe.1);
                }
            }
        }
    }

    vec
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
                }
            }
        }
    }
}

