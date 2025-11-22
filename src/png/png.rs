use std::{collections::HashMap, fmt, fs::File, io::{Read, Seek, SeekFrom}, ptr::null};

#[derive(PartialEq)]
enum WorkMode {
    Read,
    Write,
    Update,
    Delete,
    Help
}

impl fmt::Display for WorkMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WorkMode::Read => write!(f, "Read"),
            WorkMode::Write => write!(f, "Write"),
            WorkMode::Update => write!(f, "Update"),
            WorkMode::Delete => write!(f, "Delete"),
            WorkMode::Help => write!(f, "Help"),
        }
    }
}



pub struct PngParser {
    mode: WorkMode,
    data: Vec<u8>,
    chunk_type: String,
    shift: u32,
    file:String,
    chunk_counter: HashMap<String, u32>,
    file_descriptor: Option<File>,
    file_size: u64
}

impl PngParser {
    pub fn new() -> PngParser {
        PngParser{
            mode: WorkMode::Help,
            data: Vec::new(),
            chunk_type: String::new(),
            shift: 0,
            file: String::new(),
            chunk_counter: HashMap::new(),
            file_descriptor: None,
            file_size: 0
        }
    }

    fn parse_data(&mut self, input: String) -> Result<(), ()> {
            let separated_input = input.split(",").collect::<Vec<&str>>();
            for i in 0..separated_input.len() {
                let num = separated_input[i].parse::<u8>();
                match num {
                    Ok(n) => self.data.push(n),
                    Err(_) => return Err(())
                }
            }
            Ok(())
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
                self.file = flags[i + 1].clone();
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
            if flags[i] == "--shift" {
                if i + 1 > vec_len {
                    self.mode = WorkMode::Help;
                    println!("No shift provided");
                    return;
                }
                self.shift = flags[i + 1].parse().unwrap();
                continue;
            }
            if flags[i] == "--data" {
                if i + 1 > vec_len {
                    self.mode = WorkMode::Help;
                    println!("No data provided");
                    return;
                }
                let result = self.parse_data(flags[i + 1].clone());
                match result {
                    Ok(_)=>{},
                    Err(_)=> {
                        self.mode = WorkMode::Help;
                        println!("Invalid data provided");
                        return;
                    }
                }
                continue;
            }
        }
    }
    fn print_help(&self) {
        println!("Usage: mde png <options>");
        println!("\tOptions:");
        println!("\t\t--read");
        println!("\t\t--write");
        println!("\t\t--update");
        println!("\t\t--delete");
        println!("\t\t(read/write/update/delete)--filename <filename:string>");
        println!("\t\t(write/update/delete)--chunk_type <chunk_type:string>");
        println!("\t\t(write/update/delete)--shift <shift:number>(shift equals zero by default, shift means position of operating chunk in equal chunk type list)");
        println!("\t\t(write/update)--data <bytes:string>(sequence of integers associated to bytes separated by ',')");
        println!("Example:");
        println!("mde png --write --chunk_type tEXt --data 1,23,44,32,2 --filename test.png --shift 1");
        println!();
        println!("For more information read README.md");
    }

    fn validate_params(&self) -> Result<(), ()> {
        if self.file == "" {
            println!("No filename provided");
            self.print_help();
            return Err(());
        }
        if self.mode == WorkMode::Read {
            return Ok(());
        }
        if self.chunk_type == "" {
            println!("No chunk type provided");
            self.print_help();
            return Err(());
        }
        if self.mode == WorkMode::Delete {
            return Ok(());
        }
        if self.data.len() == 0 {
            println!("No data provided");
            self.print_help();
            return Err(());
        }
        Ok(())
    }

    fn read_chunk(&self) -> bool {}
    fn write_png(&self) {}
    
    fn read_png(&self) {
        while self.read_chunk() {}
        if self.file_descriptor.
    }

    fn update_png(&self) {}
    fn delete_png(&self) {}

    pub fn run(&mut self) {
        if self.mode == WorkMode::Help {
            self.print_help();
            return;
        }
        match self.validate_params() {
            Ok(()) => {
                println!("Program started with params:");
                println!("\tmode: {}", self.mode.to_string());
                println!("\tfile: {}", self.file);
                println!("\tchunk_type: {}", self.chunk_type);
                println!("\tshift: {}", self.shift);
                println!("\tdata: {:?}", self.data);
            }
            Err(()) => {
                return;
            }
        }
        self.file_descriptor = Some(File::open(self.file.clone()).unwrap());
        if self.file_descriptor.is_none() {
            println!("Cant open file");
            return;
        }
        
        let current_pos = self.file_descriptor.as_ref().unwrap().stream_position().unwrap();
        self.file_size = self.file_descriptor.as_ref().unwrap().seek(SeekFrom::End(0)).unwrap();
        self.file_descriptor.as_ref().unwrap().seek(SeekFrom::Start(current_pos)).unwrap();
        
        let mut signature = [0u8; 8];
        self.file_descriptor.as_ref().unwrap().read_exact(&mut signature).unwrap();
        if signature != [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
            println!("File is not png");
            return;
        }
        match self.mode {
            WorkMode::Read => self.read_png(),
            WorkMode::Write => self.write_png(),
            WorkMode::Update => self.update_png(),
            WorkMode::Delete => self.delete_png(),
            WorkMode::Help => self.print_help(),
        }

    }
}