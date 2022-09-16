/*
 *      Solution to Inda+22 Task 2
 *      By: Anders Mouanga (mouanga@kth.se)
 *
*/

// Distance is equal to lower value of absolute value of (x - W/2) and (y - H/2)
use std::io;
use std::iter;
use std::cmp;

fn main() {
    // initialize some useful variables!
    let mut width: i16 = 14;
    let mut height: i16 = 17;
    let mut x: i16 = 4;
    let mut y: i16 = 1;
    let mut x_distance: i16 = 1;
    let mut y_distance: i16 = 1;
    let mut new_line: String = "".to_string();
    let mut x_middle: i16 = 0;
    let mut y_middle: i16 = 0;
    let mut x_is_even: bool = false;
    let mut y_is_even: bool = false;
    let mut Distance: i16 = 0;

    if let 0 = width % 2 {
        // width is even!
        x_middle = width / 2;
        x_is_even = true;
    }
    else {
        // width is odd!
        x_middle = width / 2 + 1;
    }

    if let 0 = height % 2 {
        // height is even!
        y_middle = height / 2;
        y_is_even = true
    }
    else {
        // height is odd!
        y_middle = height / 2 + 1;
    }


    

    // println!("{}", "1".repeat(x as usize));


    while y <= height {
        new_line = "".to_string();
        x = 1;
        let mut y_distance: i16 = y_middle - (y - y_middle).abs();
        if(y_is_even && y >= y_middle) {
            y_distance += 1
        }
        while x <= width {
            let mut x_distance: i16 = x_middle - (x - x_middle).abs();
            if(x_is_even && x >= x_middle) {
                x_distance += 1
            }
            if(std::cmp::min(x_distance, y_distance) < 10) {
                let Distance = std::cmp::min(x_distance, y_distance);
                new_line = format!("{} {}", new_line, Distance);
            }
            else {
                new_line = format!("{} {}", new_line, ".".to_string());
            }

            x += 1
        }

        println!("{}", new_line);
        y += 1



    }








} 
