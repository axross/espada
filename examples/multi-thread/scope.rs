#[derive(Debug, Clone, Copy)]
pub struct CalculationScope {
    pub turn_from: u8,
    pub river_from: u8,
    pub turn_to: u8,
    pub river_to: u8,
}

pub fn calculate_scopes(count: u32) -> Vec<CalculationScope> {
    let mut scopes: Vec<CalculationScope> = vec![];
    let mut prev_t = 0_u8;
    let mut prev_r = 1_u8;

    for i in 0..count {
        let x: f32 = 48.0 - 48.0 * (1.0 - (i as f32 + 1.0) / count as f32).sqrt();

        let turn_to = x.floor() as u8;
        let river_to = ((48 - turn_to) as f32 * (x % 1.0)).ceil() as u8 + turn_to + 1;

        scopes.push(CalculationScope {
            turn_from: prev_t,
            turn_to,
            river_from: prev_r,
            river_to,
        });

        prev_t = turn_to;
        prev_r = river_to;
    }

    scopes
}

#[cfg(test)]
mod tests_calculate_scopes {
    use super::*;

    #[test]
    fn it_divides_into_4_scopes() {
        let scopes = calculate_scopes(4);

        assert_eq!(scopes.len(), 4);
        assert_eq!(scopes[0].turn_from, 0);
        assert_eq!(scopes[0].river_from, 1);
        assert_eq!(scopes[0].turn_to, 6);
        assert_eq!(scopes[0].river_to, 26);
        assert_eq!(scopes[1].turn_from, 6);
        assert_eq!(scopes[1].river_from, 26);
        assert_eq!(scopes[1].turn_to, 14);
        assert_eq!(scopes[1].river_to, 18);
        assert_eq!(scopes[2].turn_from, 14);
        assert_eq!(scopes[2].river_from, 18);
        assert_eq!(scopes[2].turn_to, 24);
        assert_eq!(scopes[2].river_to, 25);
        assert_eq!(scopes[3].turn_from, 24);
        assert_eq!(scopes[3].river_from, 25);
        assert_eq!(scopes[3].turn_to, 48);
        assert_eq!(scopes[3].river_to, 49);
    }

    #[test]
    fn it_divides_into_10_scopes() {
        let scopes = calculate_scopes(10);

        assert_eq!(scopes.len(), 10);
        assert_eq!(scopes[0].turn_from, 0);
        assert_eq!(scopes[0].river_from, 1);
        assert_eq!(scopes[0].turn_to, 2);
        assert_eq!(scopes[0].river_to, 25);
        assert_eq!(scopes[1].turn_from, 2);
        assert_eq!(scopes[1].river_from, 25);
        assert_eq!(scopes[1].turn_to, 5);
        assert_eq!(scopes[1].river_to, 9);
        assert_eq!(scopes[2].turn_from, 5);
        assert_eq!(scopes[2].river_from, 9);
        assert_eq!(scopes[2].turn_to, 7);
        assert_eq!(scopes[2].river_to, 43);
        assert_eq!(scopes[3].turn_from, 7);
        assert_eq!(scopes[3].river_from, 43);
        assert_eq!(scopes[3].turn_to, 10);
        assert_eq!(scopes[3].river_to, 43);
        assert_eq!(scopes[4].turn_from, 10);
        assert_eq!(scopes[4].river_from, 43);
        assert_eq!(scopes[4].turn_to, 14);
        assert_eq!(scopes[4].river_to, 18);
        assert_eq!(scopes[5].turn_from, 14);
        assert_eq!(scopes[5].river_from, 18);
        assert_eq!(scopes[5].turn_to, 17);
        assert_eq!(scopes[5].river_to, 38);
        assert_eq!(scopes[6].turn_from, 17);
        assert_eq!(scopes[6].river_from, 38);
        assert_eq!(scopes[6].turn_to, 21);
        assert_eq!(scopes[6].river_to, 42);
        assert_eq!(scopes[7].turn_from, 21);
        assert_eq!(scopes[7].river_from, 42);
        assert_eq!(scopes[7].turn_to, 26);
        assert_eq!(scopes[7].river_to, 39);
        assert_eq!(scopes[8].turn_from, 26);
        assert_eq!(scopes[8].river_from, 39);
        assert_eq!(scopes[8].turn_to, 32);
        assert_eq!(scopes[8].river_to, 47);
        assert_eq!(scopes[9].turn_from, 32);
        assert_eq!(scopes[9].river_from, 47);
        assert_eq!(scopes[9].turn_to, 48);
        assert_eq!(scopes[9].river_to, 49);
    }
}
