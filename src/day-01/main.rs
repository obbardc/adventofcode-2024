use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut left = vec![];
    let mut right = vec![];

    /* Read file into left and right vectors */
    let filename = "day01-data.txt";
    for line in std::fs::read_to_string(filename).unwrap().lines() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().ok_or("Missing left value")?.parse::<i32>()?);
        right.push(parts.next().ok_or("Missing right value")?.parse::<i32>()?);
    }

    /* Sort the lists. */
    left.sort();
    right.sort();

    let mut output = 0;
    let mut similarity = 0;

    /* Loop over the array */
    for (i, left_value) in left.iter().enumerate() {
        let right_value = right[i];

        /* Calculate the first score */
        output += if *left_value > right_value {
            left_value - right_value
        } else {
            right_value - left_value
        };

        /* Calculate the similarity score */
        similarity += left_value * right.iter().filter(|&n| *n == *left_value).count() as i32;
    }

    /* Print the output */
    println!("output: {}, similariy: {}", output, similarity);

    Ok(())
}
