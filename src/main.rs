use std::io;

fn main() {
    homework1();
}

fn homework1() {
    fibonacci();
}

fn fibonacci() {
    let mut before_prev_sum;
    let mut previous_sum = 1;
    let mut current_sum = 0;
    let mut n = String::new();

    println!("Please enter a number:");

    io::stdin().read_line(&mut n).expect("Unable to read line");

    let n: i32 = n.trim().parse().expect("Number not input");
    let mut idx = 0;

    while idx < n {
        print!("{},", current_sum);
        before_prev_sum = previous_sum;
        previous_sum = current_sum;
        current_sum = previous_sum + before_prev_sum;

        idx += 1;
        println!("");
    }
    println!("End of fibonacci sequence");
}
