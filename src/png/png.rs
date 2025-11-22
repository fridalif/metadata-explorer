use std::{collections::HashMap, fmt, fs::File, io::{Read, Seek, SeekFrom, Write}, ptr::null};
use crc::{Crc, CRC_32_ISO_HDLC};

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

const PNG_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);


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

    fn read_chunk(&mut self) -> ([u8; 4], [u8; 4], Vec<u8>, [u8; 4], &u32) {
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
        self.chunk_counter.insert(type_str.clone(), self.chunk_counter.get(&type_str).unwrap_or(&0) + 1);
        let count = self.chunk_counter.get(&type_str).unwrap();
        return (len_bytes, type_bytes, data, crc32_bytes, &count);
         
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
        loop {
            let (len_bytes, type_bytes, data, crc32_bytes, count) = self.read_chunk();
            let type_str = String::from_utf8_lossy(&type_bytes).to_string();
            println!("----------------------------");
            println!("Type: {}", type_str);
            println!("Position: {}", count); 
            println!("Length: {}", ((len_bytes[0] as u32) << 24) | ((len_bytes[1] as u32) << 16) | ((len_bytes[2] as u32) << 8) | (len_bytes[3] as u32));
            println!("CRC32: {}", ((crc32_bytes[0] as u32) << 24) | ((crc32_bytes[1] as u32) << 16) | ((crc32_bytes[2] as u32) << 8) | (crc32_bytes[3] as u32));
            if type_str != "IDAT" {
                println!("Data as string: {}", String::from_utf8_lossy(&data));
                println!("Data as bytes: {:?}", data);
            }
            let current_pos = self.file_descriptor.as_ref().unwrap().stream_position().unwrap();
            if current_pos >= self.file_size || self.file_descriptor.is_none() {
                break;
            }
            if self.chunk_type == "IEND" {
                break;
            }
        }

        let current_pos = self.file_descriptor.as_ref().unwrap().stream_position().unwrap();
        if self.file_size != current_pos {
            println!("----------------------------");
            println!("Found bytes at the end of file");
            let mut data: Vec<u8> = vec![0; (self.file_size - current_pos) as usize];
            self.file_descriptor.as_ref().unwrap().read_exact(&mut data).unwrap();
            println!("Data as string: {}", String::from_utf8_lossy(&data));
            println!("Data as bytes: {:?}", data);
        }
    }

    fn crc32(&self, data: &[u8]) -> [u8; 4] {
        let crc = PNG_CRC.checksum(data);
        let crc_bytes = crc.to_be_bytes();
        return crc_bytes;
    }

    fn make_inserting_bytes(&mut self) -> Vec<u8> {
        let mut len_bytes = [0u8, 4];
        let data_len = self.data.len() as u32;
        len_bytes[0] = (data_len as u32 >> 24) as u8;
        len_bytes[1] = (data_len as u32 >> 16) as u8;
        len_bytes[2] = (data_len as u32 >> 8) as u8;
        len_bytes[3] = (data_len as u32) as u8;

        let mut type_bytes = [0u8, 4];
        type_bytes[0] = self.chunk_type.as_bytes()[0];
        type_bytes[1] = self.chunk_type.as_bytes()[1];
        type_bytes[2] = self.chunk_type.as_bytes()[2];
        type_bytes[3] = self.chunk_type.as_bytes()[3];

        let mut temp_data: Vec<u8> = Vec::new();
        temp_data.extend_from_slice(&type_bytes);
        temp_data.extend_from_slice(&self.data);
        let crc32_bytes = self.crc32(&temp_data);
        let mut result_data: Vec<u8> = Vec::new();
        result_data.extend_from_slice(&len_bytes);
        result_data.extend_from_slice(&type_bytes);
        result_data.extend_from_slice(&self.data);
        result_data.extend_from_slice(&crc32_bytes);
        return result_data;
            
    }

    fn update_png(&mut self) {
        match self.make_backup() {
            Ok(()) => {println!("Backup created")},
            Err(()) => {
                println!("Failed to create backup. Exiting...");
                return;
            }
        }
        let chunk_type = self.chunk_type.clone();
        let position = self.position;
        let mut found = false; 
        loop {
            let (len_bytes, type_bytes, data, crc32_bytes, count) = self.read_chunk();
            let type_str = String::from_utf8_lossy(&type_bytes).to_string();
            if type_str == chunk_type && position == *count && !found {
                found = true;
                let payload = self.make_inserting_bytes();
                self.file_descriptor.as_ref().unwrap().write(&payload).unwrap();
                continue;
            }
            self.file_descriptor.as_ref().unwrap().write(&len_bytes).unwrap();
            self.file_descriptor.as_ref().unwrap().write(&type_bytes).unwrap();
            self.file_descriptor.as_ref().unwrap().write(&data).unwrap();
            self.file_descriptor.as_ref().unwrap().write(&crc32_bytes).unwrap();
            let current_pos = self.file_descriptor.as_ref().unwrap().stream_position().unwrap();
            if current_pos >= self.file_size || self.file_descriptor.is_none() {
                break;
            }
        }
        if found {
            println!("Chunk updated");
        } else {
            println!("Chunk not found");
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
        let chunk_type = self.chunk_type.clone();
        let position = self.position;
        let mut found = false; 
        loop {
            let (len_bytes, type_bytes, data, crc32_bytes, count) = self.read_chunk();
            let type_str = String::from_utf8_lossy(&type_bytes).to_string();
            if type_str == chunk_type && position == *count && !found {
                found = true;
                continue;
            }
            self.file_descriptor.as_ref().unwrap().write(&len_bytes).unwrap();
            self.file_descriptor.as_ref().unwrap().write(&type_bytes).unwrap();
            self.file_descriptor.as_ref().unwrap().write(&data).unwrap();
            self.file_descriptor.as_ref().unwrap().write(&crc32_bytes).unwrap();
            let current_pos = self.file_descriptor.as_ref().unwrap().stream_position().unwrap();
            if current_pos >= self.file_size || self.file_descriptor.is_none() {
                break;
            }
        }
        if found {
            println!("Chunk deleted");
        } else {
            println!("Chunk not found");
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