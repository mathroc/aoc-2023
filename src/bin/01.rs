advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .into_iter()
        .filter_map(|line| {
            line
                .chars()
                .into_iter()
                .filter_map(|c| char::to_digit(c, 10))
                .fold(None, |p, n| match p {
                    None => Some((n, n)),
                    Some((f, _)) => Some((f, n)),
                })
        })
        .map(|(f, l)| f * 10 + l)
        .sum())
}

trait State: Default {
    fn next(&self, _: char) -> (Self, Option<u32>) where Self: Sized;
}

#[derive(Debug)]
enum One {
    O, N, E,
}

impl Default for One {
    fn default() -> Self {
        One::O
    }
}

impl State for One {
    fn next(&self, c: char) -> (Self, Option<u32>) where Self: Sized {
        match (self, c) {
            (_, 'o') => (One::N, None),
            (One::N, 'n') => (One::E, None),
            (One::E, 'e') => (One::default(), Some(1)),
            _ => (One::default(), None),
        }
    }
}

#[derive(Debug)]
enum Two {
    T, W, O,
}

impl State for Two {
    fn next(&self, c: char) -> (Self, Option<u32>) where Self: Sized {
        match (self, c) {
            (_, 't') => (Two::W, None),
            (Two::W, 'w') => (Two::O, None),
            (Two::O, 'o') => (Two::default(), Some(2)),
            _ => (Two::default(), None),
        }
    }
}

impl Default for Two {
    fn default() -> Self {
        Two::T
    }
}

#[derive(Debug)]
enum Three {
    T, H, R, E, E2,
}

impl State for Three {
    fn next(&self, c: char) -> (Self, Option<u32>) where Self: Sized {
        match (self, c) {
            (_, 't') => (Three::H, None),
            (Three::H, 'h') => (Three::R, None),
            (Three::R, 'r') => (Three::E, None),
            (Three::E, 'e') => (Three::E2, None),
            (Three::E2, 'e') => (Three::default(), Some(3)),
            _ => (Three::default(), None),
        }
    }
}

impl Default for Three {
    fn default() -> Self {
        Three::T
    }
}

#[derive(Debug)]
enum Four {
    F, O, U, R
}

impl State for Four {
    fn next(&self, c: char) -> (Self, Option<u32>) where Self: Sized {
        match (self, c) {
            (_, 'f') => (Four::O, None),
            (Four::O, 'o') => (Four::U, None),
            (Four::U, 'u') => (Four::R, None),
            (Four::R, 'r') => (Four::default(), Some(4)),
            _ => (Four::default(), None),
        }
    }
}

impl Default for Four {
    fn default() -> Self {
        Four::F
    }
}

#[derive(Debug)]
enum Five {
    F, I, V, E
}

impl State for Five {
    fn next(&self, c: char) -> (Self, Option<u32>) where Self: Sized {
        match (self, c) {
            (_, 'f') => (Five::I, None),
            (Five::I, 'i') => (Five::V, None),
            (Five::V, 'v') => (Five::E, None),
            (Five::E, 'e') => (Five::default(), Some(5)),
            _ => (Five::default(), None),
        }
    }
}

impl Default for Five {
    fn default() -> Self {
        Five::F
    }
}

#[derive(Debug)]
enum Six {
    S, I, X
}

impl State for Six {
    fn next(&self, c: char) -> (Self, Option<u32>) where Self: Sized {
        match (self, c) {
            (_, 's') => (Six::I, None),
            (Six::I, 'i') => (Six::X, None),
            (Six::X, 'x') => (Six::default(), Some(6)),
            _ => (Six::default(), None),
        }
    }
}

impl Default for Six {
    fn default() -> Self {
        Six::S
    }
}

#[derive(Debug)]
enum Seven {
    S, E, V, E2, N
}

impl State for Seven {
    fn next(&self, c: char) -> (Self, Option<u32>) where Self: Sized {
        match (self, c) {
            (_, 's') => (Seven::E, None),
            (Seven::E, 'e') => (Seven::V, None),
            (Seven::V, 'v') => (Seven::E2, None),
            (Seven::E2, 'e') => (Seven::N, None),
            (Seven::N, 'n') => (Seven::default(), Some(7)),
            _ => (Seven::default(), None),
        }
    }
}

impl Default for Seven {
    fn default() -> Self {
        Seven::S
    }
}

#[derive(Debug)]
enum Eight {
    E, I, G, H, T
}

impl State for Eight {
    fn next(&self, c: char) -> (Self, Option<u32>) where Self: Sized {
        match (self, c) {
            (_, 'e') => (Eight::I, None),
            (Eight::I, 'i') => (Eight::G, None),
            (Eight::G, 'g') => (Eight::H, None),
            (Eight::H, 'h') => (Eight::T, None),
            (Eight::T, 't') => (Eight::default(), Some(8)),
            _ => (Eight::default(), None),
        }
    }
}

impl Default for Eight {
    fn default() -> Self {
        Eight::E
    }
}

#[derive(Debug)]
enum Nine {
    N, I, N2, E
}

impl State for Nine {
    fn next(&self, c: char) -> (Self, Option<u32>) where Self: Sized {
        match (self, c) {
            (Nine::N2, 'n') => (Nine::E, None),
            (_, 'n') => (Nine::I, None),
            (Nine::I, 'i') => (Nine::N2, None),
            (Nine::E, 'e') => (Nine::default(), Some(9)),
            _ => (Nine::default(), None),
        }
    }
}

impl Default for Nine {
    fn default() -> Self {
        Nine::N
    }
}

#[derive(Debug)]
struct Global {
    one: One,
    two: Two,
    three: Three,
    four: Four,
    five: Five,
    six: Six,
    seven: Seven,
    eight: Eight,
    nine: Nine,
}

impl State for Global {
    fn next(&self, c: char) -> (Self, Option<u32>) where Self: Sized {
        let one = self.one.next(c);
        let two = self.two.next(c);
        let three = self.three.next(c);
        let four = self.four.next(c);
        let five = self.five.next(c);
        let six = self.six.next(c);
        let seven = self.seven.next(c);
        let eight = self.eight.next(c);
        let nine = self.nine.next(c);

        (Self {
            one: one.0,
            two: two.0,
            three: three.0,
            four: four.0,
            five: five.0,
            six: six.0,
            seven: seven.0,
            eight: eight.0,
            nine: nine.0,
        }, one.1.or(two.1).or(three.1).or(four.1).or(five.1).or(six.1).or(seven.1).or(eight.1).or(nine.1))
    }

}

impl Default for Global {
    fn default() -> Self {
        Self {
            one: One::default(),
            two: Two::default(),
            three: Three::default(),
            four: Four::default(),
            five: Five::default(),
            six: Six::default(),
            seven: Seven::default(),
            eight: Eight::default(),
            nine: Nine::default(),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .filter_map(|line|
                line
                    .chars()
                    .into_iter()
                    .scan(Global::default(), |p, c| {
                        match char::to_digit(c, 10) {
                            Some(n) => {
                                *p = Global::default();
                                Some(Some(n))
                            },
                            None => {
                                let r;
                                (*p, r) = p.next(c);

                                Some(r)
                            },
                        }
                    })
                    .filter_map(|f| f)
                    .fold(None, |p, n| match p {
                        None => Some((n, n)),
                        Some((f, _)) => Some((f, n)),
                    })
            )
            .map(|(f, l)| f * 10 + l)
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
