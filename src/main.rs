use std::io::prelude::*;
use std::fs;
use serde_json::{Result, Value};

use std::collections::{HashMap};

use glib::*;

const DOCS: &str = "data/locale/en-US/libs/@lua";
const LIBS: &str = "data/libs/@lua";


fn create_ok_dir(dir: &str) {
    if fs::create_dir_all(dir).is_ok() {
        println!("Created path '{}'.", dir);
    }
}

fn gen_lib(filename: &str) {
    println!("Generating '{}'...", filename);

    let content = fs::read_to_string(filename).expect(&format!("File '{}' does not exist, quitting.", filename));

    let json: Value = serde_json::from_str(&content).expect(&format!("File '{}' may be corrupt. exiting.", filename));

    let arr = json.as_array().unwrap();

    for val in arr {
        let obj = val.as_object().unwrap();

        
        let name = String::from(obj.get("name").unwrap().as_str().unwrap());
        let descr = String::from(obj.get("description").unwrap().as_str().unwrap());

        let mut lib = GLib {
            data: GData {
                name,
                descr,
                t_type: String::new()
            },
            funcs: Vec::new()
        };

        let funcs = obj.get("functions").unwrap().as_array().unwrap();

        for func in funcs {
            let f_obj = func.as_object().unwrap();

            let realms = f_obj.get("realms").unwrap().as_array().unwrap();

            let mut flag = 0;

            for realm in realms {
                let r = realm.as_str().unwrap();

                if r == "client" {
                    flag |= 2;
                }
                if r == "server" {
                    flag |= 4;
                }
            }

            let mut descr = String::new();
            let name = String::from(f_obj.get("name").unwrap().as_str().unwrap());
            let mut r_type = String::new();
            let mut m_args = Vec::new();

            if flag & 2 == 2 && flag & 4 == 4 {
                descr.push_str("SH|");
            } else if flag & 4 == 4 {
                descr.push_str("S|");
            } else {
                descr.push_str("C|");
            }

            if let Some(v) = f_obj.get("description") {
                descr.push_str(v.as_str().unwrap());
            }

            if let Some(v) = f_obj.get("returnValues") {
                r_type.push_str(v.as_array().unwrap().first().unwrap().as_object().unwrap().get("type").unwrap().as_str().unwrap());
            }

            if let Some(v) = f_obj.get("arguments") {
                let args = v.as_array().unwrap();

                for aarg in args {
                    let arg = aarg.as_object().unwrap();

                    let g_args = GParam {
                        data: GData {
                            name: String::from(arg["name"].as_str().unwrap()),
                            t_type: String::from(arg["type"].as_str().unwrap()),
                            descr: String::from(arg.get("description").unwrap_or(&serde_json::Value::Null).as_str().unwrap_or(""))
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

        let l_output = glib::Generable::generate(&lib);

        let mut f = fs::File::create(&format!("{}/{}.lni", LIBS, lib.data.name)).unwrap();

        f.write_all(l_output.as_bytes()).unwrap();
    }

    println!("Generating '{}' done.\n", filename);
}

fn gen_class(val: &Value) {

}

fn main() {
    create_ok_dir(LIBS);
    create_ok_dir(DOCS);


    println!("\n\nGenerating...\n\n");

    gen_lib("input/libraries.json");


}
