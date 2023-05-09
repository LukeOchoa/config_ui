use egui::Ui;
use std::collections::BTreeMap;
use std::fs;
use toml_edit::{Decor, Value};
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

fn main() {
    sudo_main();
}
struct ConfigUi {
    file: Document,
    buffer: Document,
    manual_mutation: bool,
    options: BTreeMap<String, ()>,
    save: bool,
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
        let save = bool::default();
        Self {
            file,
            buffer,
            manual_mutation,
            options,
            save,
        }
    }
}

impl eframe::App for ConfigUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //true_loop(&mut self.file, ui);
            // for (keyu, (key, value)) in self.file.iter().enumerate() {
            //     println!("happen");
            //     ui.label(key);
            //     if value.is_table() {
            //         recur("".to_string(), value.as_table().unwrap(), ui);
            //     } else {
            //         let text = format!("{}\t\t type: {}", value.to_string(), value.type_name());
            //         ui.label(text);
            //     }
            //     println!("keyu: {}", keyu);
            // }

            // let mut string = String::default();
            // self.options.iter().for_each(|(k, _)| {
            //     string = format!("{}{}, ", string, k);
            //     //ui.label(format!("{:?}", self.options));
            // });
            // ui.label(string);
            if !self.save {
                if ui.button("Save").clicked() {
                    self.save = true;
                }
                recur_mut2(self, "".to_string(), ui);
            } else {
                ui.label("Are you sure you want to save");
                if ui.button("Yes").clicked() {
                    self.save = false;
                    panic!("DONE");
                }
                if ui.button("No").clicked() {
                    self.save = false;
                }
            }
        });
    }
}

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

fn to_bold(string: impl ToString) -> egui::RichText {
    let string = string.to_string();
    egui::RichText::strong(egui::RichText::new(string))
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
            // ui.horizontal(|ui| {
            ui.label("[");
            for v in array.iter_mut() {
                //recur_all_things_push_string(v, options);
                recur_by_value_mut(v, options, ui);
            }
            ui.label("]");
            // });
        }
        Value::InlineTable(inline_table) => {
            //ui.horizontal(|ui| {
            ui.label(" {");
            for (k, v) in inline_table.iter_mut() {
                //recur_all_things_push_string(v, options);
                ui.label(k.to_string());
                recur_by_value_mut(v, options, ui);
            }
            ui.label("}");
            // });
        }
    }
}

fn proper_spacing() {}

fn get_key() {}

fn count_new_lines_and_make_string(string: String) -> i32 {
    let mut count = i32::default();
    for ch in string.chars() {
        if ch == '\n' {
            count += 1;
        }
    }
    count
}

fn get_not_the_key(decor: &Decor) -> Option<&str> {
    decor.prefix()?.as_str()
}

fn to_underline(string: impl ToString) -> egui::RichText {
    let string = egui::RichText::new(string.to_string());
    egui::RichText::underline(string)
}

fn recur_all_things(item: &mut Item, options: &mut BTreeMap<String, ()>, ui: &mut Ui) {
    match item {
        Item::None => {}
        Item::Value(value) => {
            recur_by_value_mut(value, options, ui);
        }
        Item::Table(table) => {
            for (k, v) in table.iter_mut() {
                // ui.horizontal(|ui| {
                //
                // let key = k.get(); //k.to_string();

                // Remove the actual key from the string
                let key = k.to_string();

                // Whatever space there is before the key, comments, etc...; GET. THAT.
                let not_the_key = get_not_the_key(k.decor());

                // Count how many spaces there should be
                let count = count_new_lines_and_make_string(key);
                let newkey = k.get();
                if v.is_str() || v.is_inline_table() {
                    //spacer(ui, count);
                    spacer(ui, count);
                    if let Some(string) = not_the_key {
                        //if string != "" || !string.contains('\n') || string != " " {
                        if string != "" && string != "\n" {
                            //let magic = if string == "" { "yes" } else { "nope" };
                            //let string =
                            //    format!("{}{}<{}>{}", string, "it happened", string, magic);
                            ui.label(string);
                        }
                    }
                    ui.horizontal(|ui| {
                        ui.label(newkey);
                        ui.label("=");
                        recur_all_things(v, options, ui);
                    });

                    //if count > 0 {
                    //    spacer(ui, count);
                    //    ui.horizontal(|ui| {
                    //        ui.label(format!("<{}>", newkey));
                    //    });
                    //} else {
                    //    ui.horizontal(|ui| {
                    //        // k.to_string().replace("\n", "")
                    //        ui.label(newkey);
                    //        // ui.label(format!("test -> {}", k.to_string()));
                    //        recur_all_things(v, options, ui);
                    //    });
                    //}
                } else {
                    ui.label(to_underline(k.to_string()));
                    recur_all_things(v, options, ui);
                }
                // });
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

fn to_green(string: impl ToString) -> egui::RichText {
    let string = string.to_string();
    egui::RichText::new(string).color(egui::Color32::GREEN)
}
fn recur_mut2(
    ConfigUi {
        file,
        buffer,
        manual_mutation,
        options,
        save,
    }: &mut ConfigUi,
    mut tab_level: String,
    ui: &mut Ui,
) {
    for (k, item) in buffer.iter_mut() {
        ui.label(to_green(format!("<|Table Start")));
        let rich_text = egui::RichText::new(k.to_string()).color(egui::Color32::RED);
        ui.label(rich_text);
        recur_all_things(item, options, ui);
        ui.label(to_green("Table End|>"));
        ui.label("");
    }
}

fn filer() -> Document {
    let toml = fs::read_to_string("Cargo.toml").unwrap();
    toml.parse::<Document>().unwrap()
}

// fn fake_iter(reff: &mut Document, ui: &mut Ui) {
//     reff.iter().for_each(|(key, thing)| {
//         ui.label(format!("{}", key));
//         ui.label(format!("{}", thing.to_string()));
//         if thing.is_table() {
//             //for thingy in thing.as_table().iter() {
//             //    ui.label(format!("Execute: |{}|", thingy));
//             //}
//             let mut keys = Vec::new();
//             thing.as_table().unwrap().iter().for_each(|(key, _value)| {
//                 keys.push(key);
//             });
//
//             if thing.to_string().contains("serde") {
//                 ui.label(format!(
//                     "ELEVATE: <{}\nkeys amount...{:?}",
//                     thing["MOre"], keys,
//                 ));
//             }
//         }
//         //spacer(ui, 2);
//     });
// }
//
// fn true_loop(file: &mut Document, ui: &mut Ui) {
//     file.iter().for_each(|(key, value)| {
//         let text = format!("{} = \"{}\"", key, value);
//         ui.label(text);
//         spacer(ui, 3);
//     });
// }

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

fn spacer(ui: &mut Ui, amount: i32) {
    for _ in 0..amount {
        ui.label("");
    }
}

// fn recur(tab_level: String, table: &Table, ui: &mut Ui) {
//     for (keyu, (key, value)) in table.iter().enumerate() {
//         if value.is_table() {
//             let nest = format!("{}Nest: |{}|\n", tab_level, key);
//             println!("if true : {}", nest);
//             ui.label(nest);
//
//             let tab_level = format!("{}{}", "\t", tab_level);
//             let new_table = value.as_table().unwrap();
//             recur(tab_level, new_table, ui);
//         } else {
//             let the_type = format!("\t\ttype: <{}>", value.type_name());
//             let mut text = format!("{}|{} = {}{}", tab_level, key, value, the_type);
//             if value.type_name() == "inline table" {
//                 let mut keys = Vec::new();
//                 value.as_inline_table().unwrap().iter().for_each(|(k, v)| {
//                     let mut kmut = k.to_owned();
//                     ui.horizontal(|ui| {
//                         ui.label(format!("AUDIT: {}", k));
//                         ui.add(egui::TextEdit::singleline(&mut kmut));
//                     });
//                     keys.push(k);
//                 });
//                 text.push_str(&format!("\t\t{:?}", keys));
//             }
//             ui.label(text);
//         }
//         println!("---------------------->keyu recur!|{}|{}|", keyu, key);
//     }
// }

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
