use csv;
use log::debug;
use std::fs;

pub fn write_csv(data_vec: &Vec<String>, output_file: &str) {
    // if output file path is not valid, create the folder
    if output_file.contains("/") {
        let output_folder = output_file
            .split("/")
            .collect::<Vec<&str>>()
            [0..output_file.split("/").collect::<Vec<&str>>().len() - 1].join("/");
        // check if folder exists
        if !fs::metadata(&output_folder).is_ok() {
            debug!("Creating folder: {}", output_folder);
            fs::create_dir_all(&output_folder).unwrap();
        }
    }
    let mut wtr = csv::Writer::from_path(output_file).unwrap();
    for data in data_vec {
        wtr.write_record(data.split(",").collect::<Vec<&str>>()).unwrap();
    }

    wtr.flush().unwrap();
}
