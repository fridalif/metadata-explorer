
#[derive(PartialEq)]
enum WorkMode {
    Read,
    Write,
    Update,
    Delete,
    Help
}

pub struct PngParser {
    mode: WorkMode,
    data: Vec<u8>,
    chunk_type: String,
    position: u32,
    file: Vec<String>
}

impl PngParser {
    pub fn new() -> PngParser {
        PngParser{
            mode: WorkMode::Help,
            data: Vec::new(),
            chunk_type: String::new(),
            position: 0,
            file: Vec::new()
        }
    }
    pub fn parse(&mut self, flags: &Vec<String>) {
        let vec_len = flags.len();
        for i in 0..vec_len {
            if flags[i] == "--help" {
                self.mode = WorkMode::Help;
                return;
            }
            if flags[i] == "--read" && self.mode == WorkMode::Help {
                self.mode = WorkMode::Read;
                continue;
            }
            if flags[i] == "--write" && self.mode == WorkMode::Help {
                self.mode = WorkMode::Write;
                continue;
            }
            if flags[i] == "--update" && self.mode == WorkMode::Help {
                self.mode = WorkMode::Update;
                continue;
            }
            if flags[i] == "--delete" && self.mode == WorkMode::Help {
                self.mode = WorkMode::Delete;
                continue;
            }
            if flags[i] == "--filename" {
                if i + 1 > vec_len {
                    self.mode = WorkMode::Help;
                    println!("No filename provided");
                    return;
                }
                self.file.push(flags[i + 1].clone());
                continue;
            }
            if flags[i] == "--chunk_type" {
                if i + 1 > vec_len {
                    self.mode = WorkMode::Help;
                    println!("No chunk type provided");
                    return;
                }
                self.chunk_type = flags[i + 1].clone();
                continue;
            }
            if flags[i] == "--position" {
                if i + 1 > vec_len {
                    self.mode = WorkMode::Help;
                    println!("No position provided");
                    return;
                }
                self.position = flags[i + 1].parse().unwrap();
                continue;
            }
            if flags[i] == "--data" {
                if i + 1 > vec_len {
                    self.mode = WorkMode::Help;
                    println!("No data provided");
                    return;
                }
                self.data = flags[i + 1].clone().into_bytes();
                continue;
            }
        }
    }
    pub fn run(self) {}
}