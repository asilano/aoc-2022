#![feature(linked_list_cursors)]
use std::{fs, collections::LinkedList, ops::{Deref, DerefMut}};

use itertools::Itertools;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

fn data_to_int_list(data: &String) -> LinkedList<i64> {
    data.lines().map(|n| n.parse::<i64>().unwrap()).collect::<LinkedList<i64>>()
}

fn data_to_list_orig_val_new_pos(data: &String) -> LinkedList<(i64, usize)> {
    data.lines().enumerate().map(|(i, n)| (n.parse::<i64>().unwrap() * 811589153, i)).collect::<LinkedList<(i64, usize)>>()
}

fn part_one(numbers: &mut LinkedList<i64>) -> i64 {
    let reference = numbers.iter().map(|n| *n).collect::<Vec<i64>>();
    let mut index_guess = 0;

    for number in reference {
        {
	        let mut cursor = numbers.cursor_front_mut();
            for _ in 0..index_guess { cursor.move_next(); }

	        // Is cursor pointing at this number? If not, advance until it is.
	        while *cursor.current().unwrap() != number {
	            cursor.move_next();
	            if cursor.current().is_none() { cursor.move_next(); }
	        }
            index_guess = cursor.index().unwrap();
	
	        let move_val = *cursor.current().unwrap();
	        if move_val != 0 {
	            let moving_elem = cursor.remove_current_as_list().unwrap();
	            if cursor.current().is_none() { cursor.move_next(); }
	            // cursor now points just-before the element after the moving one.
	        
	            if move_val < 0 {
	                for _ in move_val..0 {
	                    cursor.move_prev();
	                    if cursor.current().is_none() { cursor.move_prev(); }
	                }
	                // Cursor points to the elem after where we're splicing this...
	                cursor.splice_before(moving_elem);
	                // ... and still does.
//	                for _ in move_val..0 {
//	                    cursor.move_next();
//	                    if cursor.current().is_none() { cursor.move_next(); }
//	                }
	            } else {
	                for _ in 0..move_val {
	                    cursor.move_next();
	                    if cursor.current().is_none() { cursor.move_next(); }
	                }
	                // Cursor points to the elem after where we're splicing this...
	                cursor.splice_before(moving_elem);
	                // ... and still does. So we need to move back the move_val plus one
//	                for _ in 0..move_val+1 {
//	                    cursor.move_prev();
//	                    if cursor.current().is_none() { cursor.move_prev(); }
//	                }
	            }
	        }
        }
    }

    let mixed_as_vec: Vec<i64> = numbers.iter().map(|n| *n).collect();
    let zero_pos = mixed_as_vec.iter().position(|&n| n == 0).unwrap();
    let len = mixed_as_vec.len();
    let thou = mixed_as_vec.get((zero_pos + 1000) % len).unwrap();
    let twothou = mixed_as_vec.get((zero_pos + 2000) % len).unwrap();
    let threethou = mixed_as_vec.get((zero_pos + 3000) % len).unwrap();
    thou + twothou + threethou
}

fn part_two(val_where_now: &mut LinkedList<(i64, usize)>) -> i64 {
    let len = val_where_now.iter().count();
    let mut cursor = val_where_now.cursor_front_mut();
    let mut completed_cycles = 0;

    while completed_cycles < 10 {
        let elem = cursor.current().unwrap();
        let old_index = elem.1;
        let move_amt = if elem.0 > 0 {
            elem.0 % (len as i64 - 1)
        } else {
            elem.0 % (len as i64 - 1)
        };
        let mut new_index = (old_index as i64 + move_amt) % (len as i64 - 1);
        if new_index < 0 { new_index += len as i64 - 1 }
        let move_fwd = new_index as usize >= old_index;
        elem.1 = new_index as usize;
        let pos = cursor.index();
        cursor.move_next();
        while cursor.index() != pos {
            if !cursor.index().is_none() {
                let check_elem = cursor.current().unwrap();
                if move_fwd {
                    if ((old_index+1)..=(new_index as usize)).contains(&check_elem.1) {
                        check_elem.1 -= 1;
                    }
                } else {
                    if (new_index as usize..old_index).contains(&check_elem.1) {
                        check_elem.1 += 1;
                    }
                }
            }
            cursor.move_next();
        }

        cursor.move_next();
        if cursor.index().is_none() {
            completed_cycles += 1;
            cursor.move_next();
        }
    }

    cursor = val_where_now.cursor_front_mut();
    while cursor.current().unwrap().0 != 0 { cursor.move_next(); }
    
    let zero_index = cursor.current().unwrap().1;
    let thou_index = (zero_index + 1000) % len;
    let twothou_index = (thou_index + 1000) % len;
    let threethou_index = (twothou_index + 1000) % len;

    cursor = val_where_now.cursor_front_mut();
    while cursor.current().unwrap().1 != thou_index { cursor.move_next(); }
    let thou = cursor.current().unwrap().0;
    cursor = val_where_now.cursor_front_mut();
    while cursor.current().unwrap().1 != twothou_index { cursor.move_next(); }
    let twothou = cursor.current().unwrap().0;
    cursor = val_where_now.cursor_front_mut();
    while cursor.current().unwrap().1 != threethou_index { cursor.move_next(); }
    let threethou = cursor.current().unwrap().0;
    println!("{}, {}, {}", thou, twothou, threethou);
    thou + twothou + threethou
}

fn main() {
    let data = load_data();
    let numbers = data_to_int_list(&data);
    let mut part_one_numbers = numbers.clone();
    let coords = part_one(&mut part_one_numbers);
    println!("Part one {}", coords);

    let mut part_two_numbers = data_to_list_orig_val_new_pos(&data);
    let coords = part_two(&mut part_two_numbers);
    println!("Part two {}", coords);
}
