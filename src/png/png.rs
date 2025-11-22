
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
    shift: u32,
    file: Vec<String>
}

impl PngParser {
    pub fn new() -> PngParser {
        PngParser{
            mode: WorkMode::Help,
            data: Vec::new(),
            chunk_type: String::new(),
            shift: 0,
            file: Vec::new()
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
    fn print_help(self) {
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
    pub fn run(self) {
        if self.mode == WorkMode::Help {
            self.print_help();
            return;
        }
    }
}