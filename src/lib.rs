pub const NEWLINE_CHAR: &str = "\\r\\n";

/// Parse a description in config (.lni) format friendly.
pub fn parse_description(content: &str) -> String {
    content.replace("'", "\\'").replace("\n", NEWLINE_CHAR)
}

pub struct GData {
    pub name: String,
    pub t_type: String,
    pub descr: String,
}

pub trait Generable {
    fn generate(&self) -> String;
}

pub trait ClassGenerable: Generable {
    fn generate_globalheader(&self, gen: &mut String);
    fn generate_libheader(&self, buffer: &mut String);
    fn generate_classheader(&self, buffer: &mut String);
}

pub trait DocGenerable {
    fn generate(&self) -> String;
}

pub struct GParam {
    pub data: GData,
}

impl Generable for GParam {
    fn generate(&self) -> String {
        let mut gen = String::new();

        if !self.data.name.is_empty() {
            gen.push_str(&format!(
                "name = '{}'\ntype = '{}'\ndescription = '{}'",
                self.data.name, self.data.t_type, self.data.descr
            ));
        } else {
            gen.push_str(&format!("type = '{}'", self.data.t_type))
        }

        gen
    }
}

pub struct GFunc {
    pub data: GData,
    pub params: Vec<GParam>,
}

impl Generable for GFunc {
    fn generate(&self) -> String {
        let mut gen = String::new();

        gen.push_str(&format!(
            "[{}]\ndescription = '{}'\n",
            self.data.name, self.data.descr
        ));

        if self.params.len() > 0 {
            gen.push_str("[[.args]]\n");

            for (i, e) in self.params.iter().enumerate() {
                if i >= 1 {
                    gen.push_str("``````````\n");
                }

                gen.push_str(&e.generate());

                gen.push_str("\n");
            }
        }

        if !self.data.t_type.is_empty() {
            gen.push_str("[[.returns]]\n");
            gen.push_str(&format!("type = '{}'\n", self.data.t_type));
        }

        gen
    }
}

pub struct GLib {
    pub data: GData,
    pub funcs: Vec<GFunc>,
}

impl Generable for GLib {
    fn generate(&self) -> String {
        let mut gen = String::new();

        for e in self.funcs.iter() {
            gen.push_str(&e.generate());

            gen.push_str("\n");
        }

        gen
    }
}

impl ClassGenerable for GLib {
    fn generate_globalheader(&self, gen: &mut String) {
        gen.push_str("<default>\n");
        gen.push_str("type = 'function'\n\n");
        gen.push_str("[arg]\n");
        gen.push_str("type = 'table'\n\n");
    }

    fn generate_classheader(&self, gen: &mut String) {
        let formatted_name = format!("name = '{}'\n", self.data.name);

        gen.push_str("<default>\n");
        gen.push_str("type = 'function'\n\n");
        gen.push_str("parent = {\n");
        gen.push_str("\t1 = {\n");
        gen.push_str("\t\ttype = 'object',\n\t\t");
        gen.push_str(&formatted_name);
        gen.push_str("\t},\n");
        gen.push_str("}\n\n");
    }

    fn generate_libheader(&self, gen: &mut String) {
        let formatted_name = format!("name = '{}'\n", self.data.name);

        gen.push_str(&format!("[{}]\n", self.data.name));
        gen.push_str("type = 'table'\n");
        gen.push_str("[[.source]]\n");
        gen.push_str("type = 'global'\n");
        gen.push_str("``````````\n");
        gen.push_str("type = 'library'\n");
        gen.push_str(&formatted_name);
        gen.push_str("\n<default>\n");
        gen.push_str("type = 'function'\n");
        gen.push_str("parent = {\n");
        gen.push_str("\t1 = {\n");
        gen.push_str("\t\ttype = 'global',\n\t\t");
        gen.push_str(&formatted_name);
        gen.push_str("\t},\n");
        gen.push_str("\t2 = {\n");
        gen.push_str("\t\ttype = 'library',\n\t\t");
        gen.push_str(&formatted_name);
        gen.push_str("\t}\n");
        gen.push_str("}\n\n");
    }
}

// Unused.
impl DocGenerable for GLib {
    fn generate(&self) -> String {
        let mut gen = String::new();

        for e in self.funcs.iter() {
            gen.push_str(&format!(
                "[{}]\ndescription = '{}'\n\n",
                e.data.name, e.data.descr
            ));
        }

        gen
    }
}
