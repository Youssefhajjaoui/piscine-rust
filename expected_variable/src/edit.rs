pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();

    let mut matrice = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        matrice[i][0] = i;
    }
    for j in 0..=n {
        matrice[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                0
            } else {
                1
            };

            matrice[i][j] = *[
                matrice[i - 1][j] + 1,
                matrice[i][j - 1] + 1,
                matrice[i - 1][j - 1] + cost,
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    matrice[m][n]
}
