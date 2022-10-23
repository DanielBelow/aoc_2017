use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Clone, Debug)]
#[display("{from} => {to}")]
pub struct ParsedInput {
    from: String,
    to: String,
}

impl ParsedInput {
    fn to_rule(&self) -> Rule {
        let mut from_vec = vec![];
        for spl in self.from.split('/') {
            let data = spl.chars().collect_vec();
            from_vec.push(data);
        }

        let mut to_vec = vec![];
        for spl in self.to.split('/') {
            let data = spl.chars().collect_vec();
            to_vec.push(data);
        }

        let result = Rule {
            from: Image::new(&from_vec),
            to: Image::new(&to_vec),
        };

        assert!(result.from.is_valid());
        assert!(result.to.is_valid());

        result
    }
}

#[derive(Clone, Debug)]
pub struct Image {
    data: Vec<Vec<char>>,
}

impl Image {
    /*
    fn print(&self) {
        println!("-- IMAGE BEGIN ------------------");
        let dim = self.data.len();
        for (x, y) in iproduct!(0..dim, 0..dim) {
            print!("{}", self.data[x][y]);
            if y == dim - 1 {
                println!();
            }
        }
        println!("-- IMAGE END --------------------");
        println!();
    }
    */

    fn new(data: &[Vec<char>]) -> Self {
        let img = Self {
            data: data.to_vec(),
        };

        assert!(
            img.is_valid(),
            "Attempt to create image with invalid pixels!"
        );

        img
    }

    fn is_valid(&self) -> bool {
        self.data
            .iter()
            .flatten()
            .all(|it| *it == '#' || *it == '.')
    }

    fn flip_x(&self) -> Self {
        let mut flipped = Vec::new();

        for row in &self.data {
            let flipped_row = row.iter().rev().copied().collect_vec();
            flipped.push(flipped_row);
        }

        Image::new(&flipped)
    }

    fn flips_and_rotations(&self) -> Vec<Image> {
        let dim = self.data.len();

        let mut res = Vec::new();

        let mut add_rotations = |tile: &Self| {
            res.push(tile.clone());
            {
                let mut acc = tile.data.clone();
                for (x, y) in iproduct!(0..dim, 1..=dim) {
                    acc[y - 1][x] = tile.data[dim - y][x];
                }

                let mt = Self::new(&acc);
                res.push(mt);
            }
            {
                let mut acc = tile.data.clone();
                for (x, y) in iproduct!(1..=dim, 1..=dim) {
                    acc[y - 1][x - 1] = tile.data[dim - x][dim - y];
                }

                let mt = Self::new(&acc);
                res.push(mt);
            }

            {
                let mut acc = tile.data.clone();
                for (x, y) in iproduct!(1..=dim, 0..dim) {
                    acc[x - 1][y] = tile.data[y][dim - x];
                }

                let mt = Self::new(&acc);
                res.push(mt);
            }
        };

        add_rotations(self);

        let flipped = self.flip_x();
        add_rotations(&flipped);

        res
    }

    fn from_parts(parts: &[Vec<Image>]) -> Self {
        assert!(!parts.is_empty());

        let num_images_per_row = parts[0].len();
        assert!(num_images_per_row > 0);

        let image_size = parts[0][0].data.len();

        let total_size = num_images_per_row * image_size;

        // final image will be total_size x total_size
        let mut merged_data = vec![vec![' '; total_size]; total_size];

        for row in 0..num_images_per_row {
            for col in 0..num_images_per_row {
                let cur_image = &parts[row][col].data;

                for (x, y) in iproduct!(0..image_size, 0..image_size) {
                    merged_data[row * image_size + x][col * image_size + y] = cur_image[x][y];
                }
            }
        }

        Self::new(&merged_data)
    }

    fn split(&self) -> Vec<Vec<Image>> {
        let current_size = self.data.len();

        let mut result = vec![];

        let split_size = if current_size % 2 == 0 {
            2
        } else {
            assert_eq!(current_size % 3, 0);
            3
        };

        let num_squares = current_size / split_size;

        for row in 0..num_squares {
            let mut row_squares = vec![];
            for col in 0..num_squares {
                let mut square = vec![vec![' '; split_size]; split_size];

                for (x, y) in iproduct!(0..split_size, 0..split_size) {
                    square[x][y] = self.data[row * split_size + x][col * split_size + y];
                }

                row_squares.push(Image::new(&square));
            }

            result.push(row_squares);
        }

        result
    }

    fn transform(&self, rules: &[Rule]) -> Self {
        if let Some(rule) = find_matching_rule(rules, self) {
            rule.to.clone()
        } else {
            unreachable!("No matching rule found!")
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rule {
    from: Image,
    to: Image,
}

impl Rule {
    fn matches(&self, other: &Image) -> bool {
        other
            .flips_and_rotations()
            .iter()
            .any(|it| it.data == self.from.data)
    }
}

#[aoc_generator(day21)]
pub fn generate(inp: &str) -> Vec<Rule> {
    inp.lines()
        .filter_map(|it| it.parse::<ParsedInput>().ok())
        .map(|it| it.to_rule())
        .collect()
}

fn find_matching_rule<'a>(rules: &'a [Rule], image: &Image) -> Option<&'a Rule> {
    rules.iter().find(|it| it.matches(image))
}

fn run_iterations(rules: &[Rule], num_iterations: usize) -> usize {
    let mut image = Image::new(&[
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ]);

    for _idx in 0..num_iterations {
        let squares = image.split();
        let transformed_squares = squares
            .iter()
            .map(|it| it.iter().map(|img| img.transform(rules)).collect_vec())
            .collect_vec();
        image = Image::from_parts(&transformed_squares);
    }

    image.data.iter().flatten().filter(|it| **it == '#').count()
}

#[aoc(day21, part1)]
pub fn part1(rules: &[Rule]) -> usize {
    run_iterations(rules, 5)
}

#[aoc(day21, part2)]
pub fn part2(rules: &[Rule]) -> usize {
    run_iterations(rules, 18)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let rule1 = Rule {
            from: Image::new(&[vec!['.', '.'], vec!['.', '#']]),
            to: Image::new(&[
                vec!['#', '#', '.'],
                vec!['#', '.', '.'],
                vec!['.', '.', '.'],
            ]),
        };

        let rule2 = Rule {
            from: Image::new(&[
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '#'],
            ]),
            to: Image::new(&[
                vec!['#', '.', '.', '#'],
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['#', '.', '.', '#'],
            ]),
        };

        let result = run_iterations(&[rule1, rule2], 2);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_rotations() {
        let test_rule = Rule {
            from: Image::new(&[
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '#'],
            ]),
            to: Image::new(&[
                vec!['#', '.', '.', '#'],
                vec!['.', '.', '.', '.'],
                vec!['.', '.', '.', '.'],
                vec!['#', '.', '.', '#'],
            ]),
        };

        let test_img = Image::new(&[
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ]);

        for img in test_img.flips_and_rotations() {
            assert!(test_rule.matches(&img));
        }
    }

    #[test]
    fn test_image_split_2x2() {
        let img = Image::new(&[
            vec!['#', '.', '.', '#'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['#', '.', '.', '#'],
        ]);

        let split = img.split();

        let expected_split_0_0 = vec![vec!['#', '.'], vec!['.', '.']];
        let expected_split_0_1 = vec![vec!['.', '#'], vec!['.', '.']];
        assert_eq!(split[0][0].data, expected_split_0_0);
        assert_eq!(split[0][1].data, expected_split_0_1);

        let expected_split_1_0 = vec![vec!['.', '.'], vec!['#', '.']];
        let expected_split_1_1 = vec![vec!['.', '.'], vec!['.', '#']];
        assert_eq!(split[1][0].data, expected_split_1_0);
        assert_eq!(split[1][1].data, expected_split_1_1);
    }

    #[test]
    fn test_image_split_2x2_multiple() {
        let img = Image::new(&[
            vec!['#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
        ]);

        let split = img.split();

        let expected_split_0_0 = vec![vec!['#', '#'], vec!['#', '.']];
        let expected_split_0_1 = vec![vec!['.', '#'], vec!['.', '#']];
        let expected_split_0_2 = vec![vec!['#', '.'], vec!['.', '.']];
        assert_eq!(split[0][0].data, expected_split_0_0);
        assert_eq!(split[0][1].data, expected_split_0_1);
        assert_eq!(split[0][2].data, expected_split_0_2);

        let expected_split_1_0 = vec![vec!['.', '.'], vec!['#', '#']];
        let expected_split_1_1 = vec![vec!['.', '.'], vec!['.', '#']];
        let expected_split_1_2 = vec![vec!['.', '.'], vec!['#', '.']];
        assert_eq!(split[1][0].data, expected_split_1_0);
        assert_eq!(split[1][1].data, expected_split_1_1);
        assert_eq!(split[1][2].data, expected_split_1_2);

        let expected_split_2_0 = vec![vec!['#', '.'], vec!['.', '.']];
        let expected_split_2_1 = vec![vec!['.', '#'], vec!['.', '.']];
        let expected_split_2_2 = vec![vec!['.', '.'], vec!['.', '.']];
        assert_eq!(split[2][0].data, expected_split_2_0);
        assert_eq!(split[2][1].data, expected_split_2_1);
        assert_eq!(split[2][2].data, expected_split_2_2);
    }

    #[test]
    fn test_merge_image_2x2() {
        let img_split_0_0 = Image::new(&[vec!['#', '.'], vec!['.', '.']]);
        let img_split_0_1 = Image::new(&[vec!['.', '#'], vec!['.', '.']]);
        let img_split_1_0 = Image::new(&[vec!['.', '.'], vec!['#', '.']]);
        let img_split_1_1 = Image::new(&[vec!['.', '.'], vec!['.', '#']]);

        let top_row = vec![img_split_0_0, img_split_0_1];
        let bottom_row = vec![img_split_1_0, img_split_1_1];

        let merged_img = Image::from_parts(&[top_row, bottom_row]);

        let expected_img = Image::new(&[
            vec!['#', '.', '.', '#'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['#', '.', '.', '#'],
        ]);

        assert_eq!(merged_img.data, expected_img.data);
    }

    #[test]
    fn test_merge_image_3x3() {
        let img_split = Image::new(&[
            vec!['#', '#', '.'],
            vec!['#', '.', '.'],
            vec!['.', '.', '.'],
        ]);

        let expected_img = Image::new(&[
            vec!['#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
        ]);

        let top_row = vec![img_split.clone(), img_split.clone()];
        let bottom_row = vec![img_split.clone(), img_split];

        let merged_img = Image::from_parts(&[top_row, bottom_row]);
        assert_eq!(merged_img.data, expected_img.data);
    }
}
