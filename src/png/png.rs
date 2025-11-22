
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
        for flag in flags {
            if flag == "--help" {
                self.mode = WorkMode::Help;
                continue;
            }
            if flag == "--read" && self.mode == WorkMode::Help {
                self.mode = WorkMode::Read;
                continue;
            }
            if flag == "--write" && self.mode == WorkMode::Help {
                self.mode = WorkMode::Write;
                continue;
            }
            if flag == "--update" && self.mode == WorkMode::Help {
                self.mode = WorkMode::Update;
                continue;
            }
            if flag == "--delete" && self.mode == WorkMode::Help {
                self.mode = WorkMode::Delete;
                continue;
            }
        }
    }
    pub fn run(self) {}
}