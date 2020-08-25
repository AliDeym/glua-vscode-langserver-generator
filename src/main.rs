use std::io::prelude::*;
use std::fs;
use serde_json::{Result, Value};

use std::collections::{HashMap};

use glib::*;

const REGEX_FOLDER: &str = "data";
const DOCS: &str = "data/locale/en-US/libs/@lua";
const LIBS: &str = "data/libs/@lua";


fn create_ok_dir(dir: &str) {
    if fs::create_dir_all(dir).is_ok() {
        println!("Created path '{}'.", dir);
    }
}

fn gen_func(lib: &mut GLib, f_obj: &serde_json::map::Map<String, Value>, class_signature: Option<String>) {
    let realms = f_obj.get("realms").unwrap().as_array().unwrap();

        let mut flag = 0;

        for realm in realms {
            let r = realm.as_str().unwrap();

            if r.to_lowercase() == "client" {
                flag |= 2;
            }
            if r.to_lowercase() == "server" {
                flag |= 4;
            }
        }

        let mut descr = String::new();
        let name = String::from(f_obj.get("name").unwrap().as_str().unwrap());
        let mut r_type = String::new();
        let mut m_args = Vec::new();

        if let Some(sign) = class_signature {
            m_args.push(GParam {
                data: GData {
                    name: String::new(),
                    descr: String::new(),
                    t_type: sign
                }
            });
        }

        if let Some(v) = f_obj.get("description") {
            descr.push_str(&parse_description(v.as_str().unwrap()));
        }
        
        
        if let Some(v) = f_obj.get("returnValues") {
            let rtn_obj = v.as_array().unwrap().first().unwrap().as_object().unwrap();

            r_type.push_str(rtn_obj.get("type").unwrap_or(&Value::Null).as_str().unwrap_or("nil"));

            // Adding return info to description is very useful.
            if let Some(return_descr) = rtn_obj.get("description") {
                descr.push_str(&format!("{0}{0}**Returns:** ", NEWLINE_CHAR));
                descr.push_str(&parse_description(return_descr.as_str().unwrap_or("")));
            }
        }

        // Define the scope of method.
        descr.push_str(&format!("{0}{0}**Scope:** ", NEWLINE_CHAR));
        
        if flag & 2 == 2 && flag & 4 == 4 {
            descr.push_str("Shared");
        } else if flag & 4 == 4 {
            descr.push_str("Server");
        } else {
            descr.push_str("Client");
        }

        if let Some(v) = f_obj.get("arguments") {
            let args = v.as_array().unwrap();

            for aarg in args {
                let arg = aarg.as_object().unwrap();

                let g_args = GParam {
                    data: GData {
                        name: String::from(arg["name"].as_str().unwrap()),
                        t_type: String::from(arg["type"].as_str().unwrap()),
                        descr: parse_description(arg.get("description").unwrap_or(&serde_json::Value::Null).as_str().unwrap_or(""))
                    }
                };

                m_args.push(g_args);
            }
        }

        let g_func = GFunc {
            data: GData {
                name,
                descr,
                t_type: r_type
            },
            params: m_args
        };

        lib.funcs.push(g_func);
}

fn gen_globals(filename: &str) {
    println!("Generating '{}'...", filename);

    let content = fs::read_to_string(filename).expect(&format!("File '{}' does not exist, quitting.", filename));

    let json: Value = serde_json::from_str(&content).expect(&format!("File '{}' may be corrupt. exiting.", filename));

    let arr = json.as_array().unwrap();

    let mut glib = GLib {
        data: GData {
            name: String::from("Globals"),
            descr: String::from("Global functions"),
            t_type: String::new()
        },
        funcs: Vec::new()
    };


    
    for val in arr {
        let func = val.as_object().unwrap();

        gen_func(&mut glib, &func, None);
    }
    
    let mut l_output = String::new();
    glib.generate_globalheader(&mut l_output);

    l_output.push_str(&glib::Generable::generate(&glib));

    let mut f = fs::File::create(&format!("{}/{}.lni", LIBS, glib.data.name)).unwrap();

    f.write_all(l_output.as_bytes()).unwrap();

    println!("Generating '{}' done.\n", filename);

    // Generating the highlight files.
    let mut filename = String::from(filename.split("/").last().unwrap());
    filename.push_str(".regex");

    println!("Generating '{}'...", filename);
    
    let mut pre_str = String::from(r#"(?<![^.]\\.|:)\\b(false|nil|true|_ENV|_G|_VERSION"#);

    // consuming is safe, we no longer need glib.
    for val in glib.funcs {
        pre_str.push_str("|");
        pre_str.push_str(&val.data.name);
    }

    pre_str.push_str(r#"\\b|(?<![.])\\.{3}(?!\\.)"#);

    let mut f = fs::File::create(&format!("{}/{}", REGEX_FOLDER, filename)).unwrap();

    f.write_all(pre_str.as_bytes()).unwrap();

    println!("Generating '{}' done.\n", filename);
}

fn gen(filename: &str, h_func: fn(&GLib, &mut String), is_class: bool) {
    println!("Generating '{}'...", filename);

    let content = fs::read_to_string(filename).expect(&format!("File '{}' does not exist, quitting.", filename));

    let json: Value = serde_json::from_str(&content).expect(&format!("File '{}' may be corrupt. exiting.", filename));

    let arr = json.as_array().unwrap();

    let mut lib_list = Vec::<GLib>::new();

    for val in arr {
        let obj = val.as_object().unwrap();

        
        let name = String::from(obj.get("name").unwrap().as_str().unwrap());
        let descr = parse_description(obj.get("description").unwrap_or(&Value::Null).as_str().unwrap_or(""));

        let mut lib = GLib {
            data: GData {
                name,
                descr,
                t_type: String::new()
            },
            funcs: Vec::new()
        };

        if !obj.contains_key("functions") {
            continue; // Empty class or library found; skip.
        }

        let funcs = obj.get("functions").unwrap().as_array().unwrap();
        
        for func in funcs {
            let f_obj = func.as_object().unwrap();

            let class_sign: Option<String> = {
                if is_class {
                    Some(String::from(&lib.data.name))
                } else {
                    None
                }
            };

            gen_func(&mut lib, f_obj, class_sign);
        }

        let mut l_output = String::new();
        h_func(&lib, &mut l_output); // Dynamic function call.

        l_output.push_str(&glib::Generable::generate(&lib));

        let mut f = fs::File::create(&format!("{}/{}.lni", LIBS, lib.data.name)).unwrap();

        lib_list.push(lib);

        f.write_all(l_output.as_bytes()).unwrap();
    }

    println!("Generating '{}' done.\n", filename);
    if is_class { return; }

    // Generating the highlight files.
    let mut filename = String::from(filename.split("/").last().unwrap());
    filename.push_str(".regex");

    println!("Generating '{}'...", filename);
    
    let mut pre_str = String::from(r#"\\b("#);

    for (i, lib) in lib_list.into_iter().enumerate() {
        if i > 0 {
            pre_str.push_str("|");
        }

        pre_str.push_str(&lib.data.name);
        pre_str.push_str(r#"\\.("#);

        let mp: Vec<String> = lib.funcs.into_iter().map(|x| x.data.name).collect();
        
        pre_str.push_str(&mp.join("|"));

        pre_str.push_str(")");
    }

    pre_str.push_str(r#")\\b"#);

    let mut f = fs::File::create(&format!("{}/{}", REGEX_FOLDER, filename)).unwrap();

    f.write_all(pre_str.as_bytes()).unwrap();

    println!("Generating '{}' done.\n", filename);
}

fn gen_lib(filename: &str) {
    gen(filename, GLib::generate_libheader, false);
}

fn gen_class(filename: &str) {
    gen(filename, GLib::generate_classheader, true);
}

fn main() {
    create_ok_dir(LIBS);
    create_ok_dir(DOCS);


    println!("\n\nGenerating...\n\n");

    gen_lib("input/libraries.json");
    gen_class("input/classes.json");
    gen_globals("input/global-functions.json");


    println!("\n\nFinished!");
}
