use std::collections::HashSet;
fn main() {
    exist(
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        String::from("ABCCED"),
    );
}
fn exist(board: Vec<Vec<char>>, w: String) -> bool {
    let word_vec: Vec<char> = w.chars().collect();
    if word_vec.len() == 1 {
        for i in 0..board.len() {
            if board[i].contains(&word_vec[0]) {
                return true;
            }
        }
        return false;
    }
    if word_vec.len() > board.len() * board[0].len() {
        return false;
    }
    let mut all_letter_positions: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut path: Vec<Vec<i32>> = Vec::new();
    for i in 0..word_vec.len() {
        all_letter_positions.push(find_letters(word_vec[i], board.clone()));
    }
    for i in 0..word_vec.len() {
        if i != word_vec.len() - 1 {
            for parent in all_letter_positions[i].clone() {
                let children = children(all_letter_positions[i + 1].clone(), parent.clone());
                if children.len() == 0 {
                    let index = all_letter_positions[i]
                        .iter()
                        .position(|x| *x == parent)
                        .unwrap();
                    all_letter_positions[i].remove(index);
                } else {
                    path.push(parent);
                }
            }
        } else {
            for parent in all_letter_positions[i].clone() {
                let children = children(all_letter_positions[i - 1].clone(), parent.clone());
                if children.len() == 0 {
                    let index = all_letter_positions[i]
                        .iter()
                        .position(|x| *x == parent)
                        .unwrap();
                    all_letter_positions[i].remove(index);
                } else {
                    path.push(parent);
                }
            }
        }
    }

    for i in 0..all_letter_positions.len() {
        if all_letter_positions[i].is_empty() {
            println!("Doesn't exist in grid");
            return false;
        }
        if i < all_letter_positions.len() - 2 {
            if all_letter_positions[i] == all_letter_positions[i + 2]
                && all_letter_positions[i].len() == 1
            {
                return false;
            }
        }
    }

    println!("Exists in Grid");
    return true;
}
fn children(children: Vec<Vec<i32>>, parent: Vec<i32>) -> Vec<Vec<i32>> {
    let mut valid_children: Vec<Vec<i32>> = Vec::new();
    for child in children {
        if valid(parent.clone(), child.clone()) {
            valid_children.push(child);
        }
    }
    return valid_children;
}
// finds all positions of a letter on the grid
fn find_letters(letter: char, board: Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let mut letter_positions: Vec<Vec<i32>> = Vec::new();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == letter {
                letter_positions.push(vec![i as i32, j as i32]);
            }
        }
    }
    return letter_positions;
}
// checks if one letter is next to another letter on the grid and returns if true
fn valid(parent: Vec<i32>, child: Vec<i32>) -> bool {
    let result: Vec<i32> = parent
        .clone()
        .into_iter()
        .zip(child.clone())
        .map(|(a, b)| (a - b).abs())
        .collect();
    if result == vec![1, 0] || result == vec![0, 1] {
        return true;
    }
    return false;
}
