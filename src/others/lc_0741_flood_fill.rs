use std::collections::VecDeque;

pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let key = image[sr as usize][sc as usize];
    if key == color {
        return image;
    }

    let height = image.len() as i32;
    let width = image[0].len() as i32;

    image[sr as usize][sc as usize] = color;
    let mut position_list: Vec<(i32, i32)> = Vec::new();
    position_list.push((sr, sc));

    while let Some(position) = position_list.pop() {
        // left, right, up, to
        let left = (position.0, 0.max(position.1 - 1));
        let right = (position.0, (width - 1).min(position.1 + 1));
        let up = (0.max(position.0 - 1), position.1);
        let down = ((height - 1).min(position.0 + 1), position.1);

        for (x, y) in [left, right, up, down] {
            if image[x as usize][y as usize] == key {
                position_list.push((x, y));
                image[x as usize][y as usize] = color;
            }
        }
    }

    image
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
            vec![vec![0, 0, 0], vec![0, 0, 0]]
        )
    }
}
