use std::collections::HashSet;

//Soduku board validator
fn main() {
    let soduku1 = Soduku {
        board: vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ],
    };

    soduku1.validate_board();
}
struct Soduku {
    board: Vec<Vec<i32>>,
}
impl Soduku {
    fn validate_board(&self) -> bool {
        let mut columns: Vec<Vec<i32>> = Vec::new();
        //validate rows
        for _i in 0..9 {
            // assigns each row to a var
            let mut row = self.board[_i].clone();
            //creates an empty vector that collects the values for the relevant column
            let mut column: Vec<i32> = vec![];
            // takes the ith element of each row and pushes it into the column vector
            for _j in 0..9 {
                let temp_row = self.board[_j].clone();
                column.push(temp_row[_i]);
            }
            // adds columns to vector columns
            columns.push((&column).to_vec());
            // removes all 0s from column
            column.retain(|&item| item != 0);
            if !(check_row(column)) {
                return false; // if duplicate
            }
            // removes all empty spaces
            row.retain(|&item| item != 0);
            // hash set used to check if a row contains duplicate values
            if !check_row(row) {
                return false;
            }
        }
        for _k in 0..9 {
            //validate boxes
            let boxes = get_boxes(columns.clone());
            let mut b = boxes[_k].clone();
            b.retain(|&item| item != 0);
            if !check_row(b.to_vec()) {
                return false;
            }
        }

        println!("VALID");
        return true;
    }
}

// checks if vector has a duplicate
fn check_row(vector: Vec<i32>) -> bool {
    let mut hash_set = HashSet::new();
    for _j in 0..vector.len() {
        // returns false if value already in hash
        if !hash_set.insert(&vector[_j]) {
            println!("{} is a duplicate in {:?}: INVALID", &vector[_j], vector);
            return false;
        }
    }
    return true;
}
fn get_boxes(columns: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut boxes: Vec<Vec<i32>> = Vec::new();
    let mut pointer: i32 = -1;
    for _j in 0..3 {
        // creates a vector containing 3 columns to be split
        let pre_box = vec![
            &columns[(pointer + 1) as usize],
            &columns[(pointer + 2) as usize],
            &columns[(pointer + 3) as usize],
        ];
        let mut box1: Vec<i32> = Vec::new();
        let mut box2: Vec<i32> = Vec::new();
        let mut box3: Vec<i32> = Vec::new();
        // loops through 3 columns splitting each and assigning each split part to the necessary box
        for item in pre_box {
            // splits column into vectors of size 3
            let mut split_column: Vec<Vec<i32>> = item.chunks(3).map(|s| s.into()).collect();
            box1.append(&mut split_column[0]); // adds top 3 elements from column into box 1
            box2.append(&mut split_column[1]); // adds middle 3 elements from column into box 2
            box3.append(&mut split_column[2]); // adds bottom 3 elements from column into box 3
        }
        // adds each box into boxes
        boxes.push(box1.to_vec());
        boxes.push(box2.to_vec());
        boxes.push(box3.to_vec());
        pointer += 3; // increments so that the next 3 columns can be fetched
    }
    return boxes;
}
