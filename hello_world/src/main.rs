fn main() {
    println!("Hello, world!");

   // average problem - Chapter 2
    let a = 13;
    let b =2.3;
    let c: f32= 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;
    assert_eq!(average, 45.1);
    println!("Test passed for average!");

    // celsius to fahrenheit - chapter 4
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed for celsius to fahrenheit!");

    // max, min, mean - chapter 5
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mean: f64;

    let mut sum = 0;
    min = numbers[0];
    max = numbers[0];

    for number in numbers {
        if number > max {
            max = number;
        } else if number < min {
            min = number;
        }
        sum += number;
    }

    mean = sum as f64 / numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed for max, min, mean!");
}

fn celsius_to_fahrenheit(celsius_temp: f64) -> f64{
    (1.8 * celsius_temp) + 32.0
}
