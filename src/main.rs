use std::io::{BufReader, BufRead};
use std::io;
use serde::Deserialize;
use std::io::Stdin;
use chrono::{DateTime, Utc};

#[derive(Debug,Deserialize)]
#[allow(dead_code,non_snake_case)]
struct Record {
    Time: DateTime<Utc>,
    Temp: f32,
    Humidity: f32,
}

fn convert_to_records(contents: Vec<String>) -> Vec<Record> {
    let mut list_of_records: Vec<Record> = vec!();

    // convert to char* and then add a header to this CSV file
    let mut lines: Vec<&str> = contents.iter().map(String::as_str).collect();
    lines.insert(0,"Time,Temp,Humidity");

    let mut clean_lines = String::from("");

    for l in lines {

        // clean the line of certain labels...
        //
        // * "Time:"
        // * "Temp:"
        // * "Humidity:"
        //
        let line_sans_labels = l.replace("Time: ","")
                                       .replace("Temp: ", "")
                                       .replace("Humidity: ", "");

        let current_line_elements: Vec<&str> = line_sans_labels.split(',').collect();

        let mut clean_line = String::from("");

        for element in current_line_elements {
            clean_line = [
                clean_line,
                element.trim().to_string(),
                ",".to_string(),
            ].concat();
        }

        clean_lines = [
            clean_lines,
            "\n".to_string(),
            clean_line.trim_end_matches(',').to_string(),
        ].concat();
    }

    let mut rdr = csv::Reader::from_reader(clean_lines.as_bytes());

    let mut num_of_errors = 0;
    for res in rdr.deserialize::<Record>() {
        let record: Record = match res {
            Ok(r) => r,
            Err(_) => {
                num_of_errors+=1;
                continue
            },
        };
        list_of_records.push(record);
    }

    if num_of_errors > 0 {
        println!("Number of broken lines that were skipped: {}", num_of_errors);
    }

    list_of_records
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader: BufReader<Stdin> = BufReader::new(stdin);
    let mut contents: Vec<String> = Vec::new();

    let lines = reader.lines();
    for line in lines {
        if line.is_err() {
            break;
        }
        contents.push(line.unwrap());
    }

    let records = convert_to_records(contents);

    //println!("Records:\n{:#?}",records);
    let mut previous_temperature = 0.0;
    let mut difference: Vec<f32> = Vec::new();
    let mut times_changes = 0;
    for r in records {
        let diff = r.Temp - previous_temperature;

        if difference.len() > 0 {
            let last = difference.len()-1;

            if (diff > 0.0 && difference[last] < 0.0) || (diff < 0.0 && difference[last] > 0.0) {
                times_changes += 1;
            }
        }

        difference.push(diff);

        if times_changes == 2 {
            println!("{}", r.Time);
            break;
        }

        previous_temperature = r.Temp;
    }

    Ok(())
}