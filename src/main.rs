use std::fs;

use toml::Table;

fn sudo_main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Config_Ui",
        options,
        Box::new(|_cc| Box::new(ConfigUi::default())),
    )
    .unwrap();
}
#[derive(Default)]
struct ConfigUi {}
impl ConfigUi {
    fn default() -> Self {
        Self {}
    }
}

impl 

fn main() {
    let file_as_string = fs::read_to_string("Cargo.toml").unwrap();

    let mut toml_value = file_as_string.as_str().parse::<Table>().unwrap();

    println!("File as a string: \n\n{}", file_as_string);

    println!("toml value: \n\n{:?}", toml_value);

    println!("\n\n\n");
    println!("Format Pretty:");
    toml_value.iter().for_each(|(key, toms_val)| {
        println!("\n \tkey: {} || value: {:?}", key, toms_val);
    });

    println!("\n\n\n");
    println!("Recur Format");
    recur2("".to_string(), &mut toml_value);

    // println!("\n\n\n");
    // println!("Format Pretty 2!:");
    // toml_value.iter().for_each(|(key, value)| {
    //     if value.is_table() {
    //         value.as_table().unwrap().iter().for_each(|(key, value)| {
    //             println!("Im NESTED");
    //             println!("Key/Value: |{} & {:?}|", key, value);
    //         });
    //     } else {
    //         println!("Key/Value: |{} & {:?}|\n", key, value);
    //     }
    // });

    let file = fs::read_to_string("Cargo.toml").unwrap();
    //let mut tfile = file.as_str().parse::
}

// value can be either a map or a string
// reem through each item in the list
// if its a map, start a new reem
// else print the string
use toml::map::Map;
fn recur2(tab_level: String, table: &mut Map<String, toml::Value>) {
    for (key, value) in table {
        if value.is_table() {
            println!("{}nest: |{}|\n", tab_level, key);
            let tab_level = format!("{}{}", tab_level, "\t");
            recur2(tab_level, value.as_table_mut().unwrap());
        } else {
            println!("{}Key: |{}| || Value: |{}|", tab_level, key, value);
        }
    }
}

// three
// two
// one

// one
// two
// three

// one
// two
