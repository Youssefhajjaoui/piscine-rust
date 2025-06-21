use json::{object, JsonValue};

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        let cal_value = food.calories.1[..food.calories.1.len() - 4]
            .parse::<f64>()
            .unwrap_or(0.0);
        cals += cal_value * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }

    object! {
        cals: (cals * 100 as f64 ).round() / 100.,
        carbs: (carbs* 100 as f64 ).round() / 100.,
        proteins: (proteins* 100 as f64 ).round() / 100.,
        fats: (fats* 100 as f64 ).round() / 100.
    }
}
