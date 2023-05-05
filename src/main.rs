use core::panic;
use std::fs;

use egui::Ui;

use toml_edit::{Document, Table};
fn sudo_main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Config_Ui",
        options,
        Box::new(|_cc| Box::new(ConfigUi::default())),
    )
    .unwrap();
}
use std::collections::BTreeMap;
struct ConfigUi {
    file: Document,
    buffer: Document,
    manual_mutation: bool,
    options: BTreeMap<String, ()>,
}

impl ConfigUi {}

fn get_options(doc: &Document) -> BTreeMap<String, ()> {
    let mut options = BTreeMap::new();
    for (_key, item) in doc.iter() {
        recur_all_things_push_string(item, &mut options);
    }
    options
}

impl Default for ConfigUi {
    fn default() -> Self {
        let file = filer();
        let buffer = file.clone();
        let manual_mutation = true;
        let options = get_options(&file);
        Self {
            file,
            buffer,
            manual_mutation,
            options,
        }
    }
}
fn spacer(ui: &mut Ui, amount: i32) {
    for _ in 0..amount {
        ui.label("");
    }
}

impl eframe::App for ConfigUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //true_loop(&mut self.file, ui);
            for (keyu, (key, value)) in self.file.iter().enumerate() {
                println!("happen");
                ui.label(key);
                if value.is_table() {
                    recur("".to_string(), value.as_table().unwrap(), ui);
                } else {
                    let text = format!("{}\t\t type: {}", value.to_string(), value.type_name());
                    ui.label(text);
                }
                println!("keyu: {}", keyu);
            }

            spacer(ui, 5);

            // let mut string = String::default();
            // self.options.iter().for_each(|(k, _)| {
            //     string = format!("{}{}, ", string, k);
            //     //ui.label(format!("{:?}", self.options));
            // });
            // ui.label(string);
            recur_mut2(self, "".to_string(), ui);
        });
    }
}

fn fake_iter(reff: &mut Document, ui: &mut Ui) {
    reff.iter().for_each(|(key, thing)| {
        ui.label(format!("{}", key));
        ui.label(format!("{}", thing.to_string()));
        if thing.is_table() {
            //for thingy in thing.as_table().iter() {
            //    ui.label(format!("Execute: |{}|", thingy));
            //}
            let mut keys = Vec::new();
            thing.as_table().unwrap().iter().for_each(|(key, _value)| {
                keys.push(key);
            });

            if thing.to_string().contains("serde") {
                ui.label(format!(
                    "ELEVATE: <{}\nkeys amount...{:?}",
                    thing["MOre"], keys,
                ));
            }
        }
        //spacer(ui, 2);
    });
}

fn true_loop(file: &mut Document, ui: &mut Ui) {
    file.iter().for_each(|(key, value)| {
        let text = format!("{} = \"{}\"", key, value);
        ui.label(text);
        spacer(ui, 3);
    });
}

use toml_edit::{InlineTable, Value};

fn recur_by_value(value: &Value, options: &mut BTreeMap<String, ()>) {
    match value {
        Value::String(string) => {
            options.insert(string.to_string(), ());
        }
        Value::Integer(_int) => {
            // options.insert(int.to_string(), ());
        }
        Value::Float(_float) => {
            // options.insert(float.to_string(), ());
        }
        Value::Boolean(_) => {
            //
        }
        Value::Datetime(_datetime) => {
            //
        }
        Value::Array(array) => {
            for v in array.iter() {
                //recur_all_things_push_string(v, options);
                recur_by_value(v, options);
            }
        }
        Value::InlineTable(inline_table) => {
            for (_k, v) in inline_table.iter() {
                //recur_all_things_push_string(v, options);
                recur_by_value(v, options);
            }
        }
    }
}

// a character length of 5 should be (TIMES 6) at least
// every 5 characters after that it should go down by X%

fn text_sizer(len: i32) -> f32 {
    // Never wanna look at this again lolololol
    if len < 5 {
        // This is a minimun length so the bar isnt to small
        return 30.0;
    }

    // We multiple by the strength, it slowly decends by a flat amount
    let strength = 8;

    // power is the amount it(strength) decends because of;
    let power: i32 = 5;

    // we divide by every set of 5 characters
    let current_power = len / power;

    // The reducer reduces how long the text bar by a flat amount
    let reducer: f32 = 0.2;

    // We multiply the reducer's size for every int/f32 of power
    let new_reducer = reducer * current_power as f32;

    // Controll the strength by finally reducing it with the reducer
    let current_strength: f32 = strength as f32 - new_reducer;

    // Here is the final length of the bar based on a flat scaling of how many characters there are in the line
    // It scales by every (power) amount of characters [currently at the time of writing, its 5 chars]
    len as f32 * current_strength
}
use toml_edit::Formatted;
fn recur_by_value_mut(value: &mut Value, options: &BTreeMap<String, ()>, ui: &mut Ui) {
    match value {
        Value::String(string) => {
            let mut temp = string.to_string().replace("\"", "");
            let len = temp.len();
            //ui.add(egui::TextEdit::singleline(&mut temp));
            let x = egui::TextEdit::singleline(&mut temp);
            //let newf: f32 = (len * 8) as f32;
            let newf = text_sizer(len as i32);
            let x = egui::TextEdit::desired_width(x, newf);
            ui.horizontal(|ui| {
                ui.add(x);
                ui.label(len.to_string());
            });

            *value = Value::String(Formatted::new(temp));
        }
        Value::Integer(_int) => {
            // options.insert(int.to_string(), ());
        }
        Value::Float(_float) => {
            // options.insert(float.to_string(), ());
        }
        Value::Boolean(_) => {
            //
        }
        Value::Datetime(_datetime) => {
            //
        }
        Value::Array(array) => {
            ui.horizontal(|ui| {
                for v in array.iter_mut() {
                    //recur_all_things_push_string(v, options);

                    recur_by_value_mut(v, options, ui);
                }
            });
        }
        Value::InlineTable(inline_table) => {
            ui.horizontal(|ui| {
                for (k, v) in inline_table.iter_mut() {
                    //recur_all_things_push_string(v, options);
                    ui.label(k.to_string());
                    recur_by_value_mut(v, options, ui);
                }
            });
        }
    }
}

fn recur_all_things(item: &mut Item, options: &mut BTreeMap<String, ()>, ui: &mut Ui) {
    match item {
        Item::None => {}
        Item::Value(value) => {
            recur_by_value_mut(value, options, ui);
        }
        Item::Table(table) => {
            for (k, v) in table.iter_mut() {
                ui.vertical(|ui| {
                    ui.label(k.to_string());
                    ui.horizontal(|ui| {
                        recur_all_things(v, options, ui);
                    });
                });
            }
        }
        Item::ArrayOfTables(array_of_tables) => {
            // Not procced yet...
            for v in array_of_tables.iter_mut() {
                ui.horizontal(|ui| {
                    for (k, v2) in v.iter_mut() {
                        ui.label(k.to_string());
                        recur_all_things(v2, options, ui);
                    }
                });
            }
        }
    }
}

fn recur_by_table(table: &Table, options: &mut BTreeMap<String, ()>) {
    for (_k, item) in table.iter() {
        recur_all_things_push_string(item, options);
    }
}
use toml_edit::Item;
fn recur_all_things_push_string(item: &Item, options: &mut BTreeMap<String, ()>) {
    match item {
        Item::None => {}
        Item::Value(value) => {
            recur_by_value(&value, options);
        }
        Item::Table(table) => {
            recur_by_table(&table, options);
        }
        Item::ArrayOfTables(array_of_tables) => {
            for v in array_of_tables.iter() {
                recur_by_table(v, options);
            }
        }
    }
}

// Value::String(..) => "string",
// Value::Integer(..) => "integer",
// Value::Float(..) => "float",
// Value::Boolean(..) => "boolean",
// Value::Datetime(..) => "datetime",
// Value::Array(..) => "array",
// Value::InlineTable(..) => "inline table",

//fn recur_for_options(ptions: &mut BTreeMap<String, ()>) {
//    for (key, value) in table.iter() {
//        if value.is_table() {
//            recur_for_options(value.as_table().unwrap(), options);
//        } else {
//            //options.insert(value.to_owned(), ());
//            match value.type_name() {
//                "inline table" => {
//                    inline_table_values(&value.as_inline_table().unwrap())
//                        .iter()
//                        .for_each(|string| options.insert(string.to_owned(), ()).unwrap());
//                }
//                _ => {
//                    options.insert(value.to_string(), ());
//                }
//            }
//        }
//    }
//}

fn recur(tab_level: String, table: &Table, ui: &mut Ui) {
    for (keyu, (key, value)) in table.iter().enumerate() {
        if value.is_table() {
            let nest = format!("{}Nest: |{}|\n", tab_level, key);
            println!("if true : {}", nest);
            ui.label(nest);

            let tab_level = format!("{}{}", "\t", tab_level);
            let new_table = value.as_table().unwrap();
            recur(tab_level, new_table, ui);
        } else {
            let the_type = format!("\t\ttype: <{}>", value.type_name());
            let mut text = format!("{}|{} = {}{}", tab_level, key, value, the_type);
            if value.type_name() == "inline table" {
                let mut keys = Vec::new();
                value.as_inline_table().unwrap().iter().for_each(|(k, v)| {
                    let mut kmut = k.to_owned();
                    ui.horizontal(|ui| {
                        ui.label(format!("AUDIT: {}", k));
                        ui.add(egui::TextEdit::singleline(&mut kmut));
                    });
                    keys.push(k);
                });
                text.push_str(&format!("\t\t{:?}", keys));
            }
            ui.label(text);
        }
        println!("---------------------->keyu recur!|{}|{}|", keyu, key);
    }
}

fn recur_mut2(
    ConfigUi {
        file,
        buffer,
        manual_mutation,
        options,
    }: &mut ConfigUi,
    mut tab_level: String,
    ui: &mut Ui,
) {
    for (k, item) in buffer.iter_mut() {
        recur_all_things(item, options, ui);
    }
}

fn recur_mut(tab_level: String, table: &mut Table, buffer: &mut Table, ui: &mut Ui) {
    for (keydex, (key, magical_thing)) in buffer.iter_mut().enumerate() {
        // If its a table, reem through it with recursion (may have many nested tables)
        if magical_thing.is_table() {
            // Tab Level is for easier viewing when it is displayed in a ui.label. The more tabs the deeper the nest!
            let tab_level = format!("{}{}", tab_level, "\t");

            // get the next level of nested variables to pass to recur_mut()
            let table = table.get_mut(&key).unwrap().as_table_mut().unwrap();
            let magical_thing = magical_thing.as_table_mut().unwrap();

            // Go one level deeper!
            recur_mut(tab_level, table, magical_thing, ui);
        } else {
        }
    }
}

//fn recur_ui(tab_lvl: String, table: &mut Map<String, toml::Value>, ui: &mut Ui) {
//    for (key, value) in table {
//        if value.is_table() {
//            let line = format!("{}nest: |{}|\n", tab_lvl, key);
//            ui.label(line);
//
//            let tab_lvl = format!("{}{}", tab_lvl, "\t");
//            recur_ui(tab_lvl, value.as_table_mut().unwrap(), ui);
//        } else {
//            let mut text = format!("{}Key: |{}| || Value: |{}|", tab_lvl, key, value);
//            let text_helper = match value.type_str() {
//                "String" => ui.add(egui::TextEdit::singleline()),
//                "Array" => {}
//            };
//
//            text.push_str(&text_helper);
//            ui.label(text);
//        }
//    }
//}

//fn recur2(tab_level: String, table: &mut Map<String, toml::Value>) {
//    for (key, value) in table {
//        if value.is_table() {
//            println!("{}nest: |{}|\n", tab_level, key);
//            let tab_level = format!("{}{}", tab_level, "\t");
//            recur2(tab_level, value.as_table_mut().unwrap());
//        } else {
//            println!("{}Key: |{}| || Value: |{}|", tab_level, key, value);
//        }
//    }
//}

fn filer() -> Document {
    let toml = fs::read_to_string("Cargo.toml").unwrap();
    toml.parse::<Document>().unwrap()
}
fn main() {
    sudo_main();
}
