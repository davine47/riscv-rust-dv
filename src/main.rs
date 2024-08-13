use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct RawInstr {
    encoding: String,
    extension: Vec<String>,
    mask: String,
    r#match: String,
    variable_fields: Vec<String>,
}

struct InstrSets {
    raw_instrs: HashMap<String, RawInstr>, 
    //riscvIsaInstrSet: HashMap<String, HashMap<String, RawInstr>>,
}

impl InstrSets {

    fn new(s: &str) -> Self {
        let content = fs::read_to_string(s);

        InstrSets {
            raw_instrs: serde_yaml::from_str(content.unwrap().as_str()).expect("Failed to parse YAML"),
            //riscvIsaInstrSet: 
        }
    }

    fn dump_all_raw_instrs(&self) {
        for (key, instr) in &self.raw_instrs{
            println!("Instruction: {}", key);
            println!("Encoding: {}", instr.encoding);
            println!("Extension: {:?}", instr.extension);
            println!("Mask: {}", instr.mask);
            println!("Match: {}", instr.r#match);
            println!("Variable Fields: {:?}", instr.variable_fields);
            println!();
        }
    }
}

fn main() {
    let instr_sets = InstrSets::new("src/isa/instr_dict.yaml");
    instr_sets.dump_all_raw_instrs();
    println!("Helloi, world!");
}
