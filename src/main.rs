use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::error::Error;
use std::process;

#[derive(Debug, Deserialize)]
struct InputRecord {
    ContentPath: String,
    AssetClass: String,
    ManagerFundName: String,
    ManagerName: String,
    Vendor: String,
    DocumentType: String,
    DocumentTitle: String,
    PeriodDate: String
}

#[derive(Debug, Serialize)]
struct OutputRecord {
    ContentPath: String,
    FileName: String,
    AssetClass: String,
    ManagerFundName: String,
    ManagerName: String,
    Vendor: String,
    DocumentType: String,
    DocumentTitle: String,
    PeriodDate: String
}

fn process_csv(read_path: &str, write_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(read_path)?;
    let mut writer = csv::Writer::from_path(write_path)?;

    for result in reader.deserialize() {
        let input_record: InputRecord = result?;

        let driveless_path: String = remove_drive_letter(input_record.ContentPath.clone());
        let filename: String = derive_filename_from_path(input_record.ContentPath.clone());

        let output_record = OutputRecord {
            ContentPath: driveless_path,
            FileName: filename,
            AssetClass: input_record.AssetClass,
            ManagerFundName: input_record.ManagerFundName,
            ManagerName: input_record.ManagerName,
            Vendor: input_record.Vendor,
            DocumentType: input_record.DocumentType,
            DocumentTitle: input_record.DocumentTitle,
            PeriodDate: input_record.PeriodDate,
        };

        writer.serialize(&output_record)?;
    }

    writer.flush()?;
    Ok(())
}

fn remove_drive_letter(path: String) -> String {
    let str_path = path.as_str();
    let mut split_path: Vec<&str> = str_path.split('\\').collect();
    split_path.pop();
    split_path.remove(0);
    let mut p = split_path.join("\\").to_string();
    p.insert(0, '\\');
    return p;
}

fn derive_filename_from_path(path: String) -> String {
    let str_path = path.as_str();
    let mut split_path: Vec<&str> = str_path.split('\\').collect();
    let f = split_path.pop().unwrap();
    return f.to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = &args[1];
    let output_file = &args[2];

    if let Err(err) = process_csv(input_file, output_file) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}