use std::{collections::HashMap, fmt, fs::File, io::{Read, Seek, SeekFrom, Write}, ptr::null};

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
    offset: u32,
    position: u32,
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
            offset: 0,
            position: 1,
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
            if flags[i] == "--offset" {
                if i + 1 > vec_len {
                    self.mode = WorkMode::Help;
                    println!("No offset provided");
                    return;
                }
                self.offset = flags[i + 1].parse().unwrap();
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
        println!("\t\t(write)--offset <offset:number>(offset equals zero by default, means offset in list of chunks with equal type)");
        println!("\t\t(delete/update)--position <position:number>(equals one by default, position in list of chunks with equal type, starts from 1)");
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
        if self.mode == WorkMode::Delete || self.mode == WorkMode::Update {
            if self.position == 0 {
                println!("No position provided");
                self.print_help();
                return Err(());
            }
        }
        if self.mode == WorkMode::Write || self.mode == WorkMode::Update {
            if self.data.len() == 0 {
                println!("No data provided");
                self.print_help();
                return Err(());
            }    
        }
        
        Ok(())
    }


    fn make_backup(&mut self) -> Result<(), ()> {
        let current_pos = self.file_descriptor.as_ref().unwrap().stream_position().unwrap();
        self.file_descriptor.as_ref().unwrap().seek(SeekFrom::Start(0)).unwrap();
        File::create(self.file.clone()+"_backup.png").unwrap();
        std::fs::copy(self.file.clone(), self.file.clone()+"_backup.png").unwrap();
        self.file_descriptor = Some(File::open(self.file.clone()).unwrap());
        self.file_descriptor.as_ref().unwrap().seek(SeekFrom::Start(current_pos)).unwrap();
        Ok(())    
    }

    fn read_chunk(&mut self) -> bool {
        let mut len_bytes: [u8; 4] = [0; 4];
        self.file_descriptor.as_ref().unwrap().read(&mut len_bytes).unwrap();
        let len = ((len_bytes[0] as u32) << 24) | ((len_bytes[1] as u32) << 16) | ((len_bytes[2] as u32) << 8) | (len_bytes[3] as u32);
        let mut type_bytes: [u8; 4] = [0; 4];
        self.file_descriptor.as_ref().unwrap().read(&mut type_bytes).unwrap();
        let type_str = String::from_utf8_lossy(&type_bytes).to_string();
        let mut data: Vec<u8> = vec![0; len as usize];
        self.file_descriptor.as_ref().unwrap().read_exact(&mut data).unwrap();
        let mut crc32_bytes: [u8; 4] = [0; 4];
        self.file_descriptor.as_ref().unwrap().read(&mut crc32_bytes).unwrap();
        let crc32 = ((crc32_bytes[0] as u32) << 24) | ((crc32_bytes[1] as u32) << 16) | ((crc32_bytes[2] as u32) << 8) | (crc32_bytes[3] as u32);
        self.chunk_counter.insert(type_str.clone(), self.chunk_counter.get(&type_str).unwrap_or(&0) + 1);
        let count = self.chunk_counter.get(&type_str).unwrap();
        if self.mode == WorkMode::Read && type_str != "IDAT" {
            println!("Found Chunk {} {}", type_str, count);
            println!("\tlen: {}", len);
            println!("\tdata as string: {}", String::from_utf8_lossy(&data));
            println!("\tdata as bytes: {:?}", data);
            println!("\tcrc32: {}", crc32);
        } else if type_str == "IDAT" {
            println!("Found Chunk {} {}", type_str, count);
        }
        if type_str == "IEND" {
            return false;
        }
        true
         
    }
    fn write_png(&mut self) {
        match self.make_backup() {
            Ok(()) => {println!("Backup created")},
            Err(()) => {
                println!("Failed to create backup. Exiting...");
                return;
            }
        }
    }
    
    fn read_png(&mut self) {
        while self.read_chunk() {}
        let current_pos = self.file_descriptor.as_ref().unwrap().stream_position().unwrap();
        if self.file_size != current_pos {
            println!("Found bytes at the end of file");
            let mut data: Vec<u8> = vec![0; (self.file_size - current_pos) as usize];
            self.file_descriptor.as_ref().unwrap().read_exact(&mut data).unwrap();
            println!("\tdata as string: {}", String::from_utf8_lossy(&data));
            println!("\tdata as bytes: {:?}", data);
        }
    }

    fn update_png(&mut self) {
        match self.make_backup() {
            Ok(()) => {println!("Backup created")},
            Err(()) => {
                println!("Failed to create backup. Exiting...");
                return;
            }
        }
    }
    fn delete_png(&mut self) {
        match self.make_backup() {
            Ok(()) => {println!("Backup created")},
            Err(()) => {
                println!("Failed to create backup. Exiting...");
                return;
            }
        }
    }

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
                println!("\toffset: {}", self.offset);
                println!("\tposition: {}", self.position);
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