use std::{env, process};
use std::error::Error;
use serde::Deserialize;
use csv;

#[derive(Debug,Deserialize)]
#[allow(non_snake_case)]
struct Values {
    // name of the columns in the csv
    X: f32,
    Y: f32
}

fn main() {
    // read the file path
    let args:Vec<String> = env::args().collect();
    let file_path:&String = &args[1];
    let csv_values = open_csv(file_path);

    // vectors to store X,Y values from the csv
    let mut x_values:Vec<f32> = Vec::new();
    let mut y_values:Vec<f32> = Vec::new();

    // assign values to variables
    if let Ok((x,y)) = &csv_values{
        x_values = x.to_vec();
        y_values = y.to_vec();
    }

    // if fails, exit program
    if let Err(ref err) = &csv_values{
        println!("error:{}",err);
        process::exit(1);
    }


    let slope:f32;
    let intercept:f32;
    (slope,intercept) = least_of_squares(&x_values,&y_values);
    let r_squared_value:f32 = r_squared(&x_values,&y_values,&slope,&intercept);

    println!("Slope:\t\t{}\nIntercept:\t{}\nR-squared:\t{}",slope,intercept,r_squared_value);

}

fn open_csv(file_path:&str) -> Result<(Vec<f32>,Vec<f32>), Box<dyn Error>>{

    // open the file 
    let mut rdr = csv::Reader::from_path(file_path)?;

    //  declare vectors to store the data
    let mut x_values:Vec<f32> = Vec::new();
    let mut y_values:Vec<f32> = Vec::new();
    for result in rdr.deserialize() {
        let record:Values = result?;
        x_values.push(record.X);
        y_values.push(record.Y);
    }
    Ok((x_values,y_values))
    
}

fn least_of_squares(x_values:&Vec<f32>,y_values:&Vec<f32>) -> (f32,f32){
    // finds the line of best fit
    // Returns the slope and intercept
    let no_rows:usize = x_values.len();
    let mut temp_x:f32;
    let mut temp_y:f32;
    let mut sum_of_x_squared:f32 = 0.0;
    let mut sum_of_x_y:f32 = 0.0;
    let mut sum_of_x:f32 = 0.0;
    let mut sum_of_y:f32 = 0.0;
    for item in 0..no_rows{
        // square x and get x*y
        temp_x = x_values[item];
        temp_y = y_values[item];

        // sum(x_values)
        sum_of_x = sum_of_x + temp_x;
        sum_of_y = sum_of_y + temp_y;

        // sum(x^2) and sum(x*y)
        sum_of_x_squared = sum_of_x_squared + (temp_x* temp_x);
        sum_of_x_y = sum_of_x_y + (temp_x* temp_y);

    }

    // formula for slope and intercept
    let slope:f32 = (((no_rows as f32)*sum_of_x_y) - (sum_of_x*sum_of_y))/(((no_rows as f32)*sum_of_x_squared) - (sum_of_x*sum_of_x));
    let intercept:f32 = (sum_of_y-(slope*sum_of_x))/(no_rows as f32);

    return (slope,intercept);
    
}

fn r_squared(x_values:&Vec<f32>,y_values:&Vec<f32>,slope:&f32,intercept:&f32) -> f32{
    let mut x:f32;
    let mut y:f32;
    let mut sum_of_squares_regression:f32 = 0.0;
    let mut total:f32 = 0.0;
    let mut counter:f32 = 0.0;
    let mean:f32;
    let no_rows:usize = x_values.len();
    
    // loop to get sum of squares regression (SSR) and mean values for y
    for item in 0..no_rows{
        // to store the mean
        total = total + y_values[item];
        counter = counter + 1.0;

        // to get the sum_of_squares_regression
        x = x_values[item];
        y = slope*x + intercept;
        sum_of_squares_regression = sum_of_squares_regression + (y - y_values[item]).powf(2.0);
        
    }
    mean = total/counter;
    
    // get the sum of squares (SST)
    let mut sum_of_squares:f32 =0.0;
    for item in 0..no_rows{
        sum_of_squares = sum_of_squares + (y_values[item]-mean).powf(2.0);
    }

    // R^2 = 1 - SSR/SST
    return 1.0- sum_of_squares_regression/sum_of_squares;
}