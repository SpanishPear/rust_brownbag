// an example program 
// to show generics and 
// zero cost abstracionts



fn main() {

    let int_slice = [1, 2, 3, 4, 5];
    let float_slice = [1.0, 2.0, 3.0, 4.0, 5.0];

    let int_sum = my_generic_sum_of_slice(&int_slice);
    let float_sum = my_generic_sum_of_slice(&float_slice);

    println!("Sum of int slice: {}", int_sum);
    println!("Sum of float slice: {}", float_sum);
    
}

fn my_generic_sum_of_slice<T>(slice: &[T]) -> T {
    // TODO: implement this function
    let mut sum = 0;
    for i in slice {
        sum += i;
    }
    sum
}

