use exif::{Field, Reader, Value};
use image::GenericImageView;
use inquire::{InquireError, Select};
use prettytable::{format, row, Table};
use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
    process::exit,
};

fn main() -> Result<(), InquireError> {
    let ascii_art = r#"
 ___ ____   ____ 
|_ _|  _ \ / ___|
 | || |_) | |    
 | ||  __/| |___ 
|___|_|    \____|
"#;

    println!("{}", ascii_art);
    println!("Image Processing CLI (IPC) written in Rust 🦀");
    println!();

    // Interactive selection for main options
    let main_options = vec!["Image processing", "Get image information data"];
    let user_choice_main = Select::new("Choose an option:", main_options.clone()).prompt()?;

    if user_choice_main == main_options[0] {
        println!("Image processing");
    }

    if user_choice_main == main_options[1] {
        let (img, file) = get_image();

        // Interactive selection for information options
        let info_options = vec![
            "Show Dimensions and File Size",
            "Show EXIF Data",
            "Show Both",
        ];
        let user_choice =
            Select::new("Choose information you want:", info_options.clone()).prompt()?;

        let dimensions = img.dimensions();
        let file_size = fs::metadata(&file)
            .expect("Could not get file metadata")
            .len() as f64
            / 1_048_576.0; // Convert bytes to MB

        // Print image and file details in table format
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(row!["Property", "Value"]);
        table.add_row(row!["File", file]);
        table.add_row(row![
            "Dimensions",
            format!("{}x{}", dimensions.0, dimensions.1)
        ]);
        table.add_row(row!["File Size", format!("{:.2} MB", file_size)]);
        println!();

        if user_choice == info_options[0] || user_choice == info_options[2] {
            println!("Image metadata");
            table.printstd();
        }

        if user_choice == info_options[1] || user_choice == info_options[2] {
            // Extract and display EXIF data
            let exif_data = extract_exif_data(&file);
            if let Some(exif_table) = exif_data {
                println!();
                println!("Image EXIF data");
                exif_table.printstd();
            } else {
                println!("No EXIF data found or could not be read.");
            }
        }
    }

    Ok(())
}

/// Get image file from user input
fn get_image() -> (image::DynamicImage, String) {
    let mut file = String::new();
    println!("Enter image name or path: "); // Prompt stays on the same line
    std::io::stdin()
        .read_line(&mut file)
        .expect("Read user input failed");
    file = file.trim().to_string();

    if !Path::new(&file).exists() {
        println!("Could not found the file");
        exit(1);
    }
    let path: &Path = Path::new(&file);
    let image = image::open(path).expect("Could not read image");

    // Return image and file name
    (image, file)
}

/// Extracts EXIF data from the image file and returns it in a table format
fn extract_exif_data(file_path: &str) -> Option<Table> {
    let file = File::open(file_path).expect("Could not open file");
    let exif_reader = Reader::new();
    let exif = exif_reader
        .read_from_container(&mut BufReader::new(file))
        .ok()?;

    let mut exif_table = Table::new();
    exif_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    exif_table.set_titles(row!["Tag", "Value"]);

    for field in exif.fields() {
        let value = format_exif_value(field);
        exif_table.add_row(row![field.tag.to_string(), value]);
    }

    Some(exif_table)
}

/// Formats the EXIF value for display
fn format_exif_value(field: &Field) -> String {
    match &field.value {
        Value::Ascii(ref v) => v
            .iter()
            .map(|s| std::str::from_utf8(s).unwrap_or("").to_string())
            .collect::<Vec<String>>()
            .join(", "),
        Value::Short(ref v) => v
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        Value::Long(ref v) => v
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        Value::Rational(ref v) => v
            .iter()
            .map(|r| format!("{}/{}", r.num, r.denom))
            .collect::<Vec<String>>()
            .join(", "),
        _ => String::from("Unsupported EXIF value type"),
    }
}
