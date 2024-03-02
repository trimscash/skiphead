use clap::{CommandFactory, Parser};
use file_format::FileFormat;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::iter;
use termion;
use termion::color;

fn print_result(
    skip_num: usize,
    pick_offset: usize,
    pick_length: usize,
    file_offset: usize,
    result_fmt: &FileFormat,
) {
    let (col, _) = termion::terminal_size().unwrap();
    let bar: String = iter::repeat("-").take(col as usize).collect();
    let green = color::Fg(color::Green);
    let reset = color::Fg(color::Reset);

    println!("{}", &bar);
    println!(
        "# skip = {}, pick_offset = {}, pick_length = {}, file_offset = {}",
        skip_num, pick_offset, pick_length, file_offset
    );
    println!("{}", &bar);

    if !matches!(result_fmt, FileFormat::ArbitraryBinaryData) {
        print!("{}", green);
    }

    print!("{} ", result_fmt.name());
    if let Some(s) = result_fmt.short_name() {
        println!("({})", s);
    } else {
        println!();
    }

    print!("{}", reset);
}

fn output_file(
    skip_num: usize,
    pick_offset: usize,
    pick_length: usize,
    file_offset: usize,
    output_directory: &String,
    buf: &Vec<u8>,
) -> Result<(), std::io::Error> {
    let dirname = &output_directory;
    let filename = format!(
        "{}/skip_{}_pick_offset_{}_pick_length_{}_file_offset_{}",
        dirname, skip_num, pick_offset, pick_length, file_offset
    );
    let mut file = match File::create(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("Can't create {} file..", &filename);
            return Err(e);
        }
    };
    match file.write_all(buf.as_slice()) {
        Ok(_) => return Ok(()),
        Err(e) => {
            println!("Can't write to {} ..", &filename);
            return Err(e);
        }
    };
}

fn skiped_and_picked_file_buf(
    skip_num: usize,
    pick_offset: usize,
    pick_length: usize,
    file_offset: usize,
    buf: &Vec<u8>,
) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut index = file_offset;
    while index < buf.len() {
        let end_index = index + (pick_length + pick_offset).min(buf.len() - index);
        result.extend_from_slice(&buf[index + pick_offset..end_index]);
        index += skip_num;
    }

    result
}

/// Parse the header of the file skipped by n bytes and display the file type.
/// Forensic app.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Number of skips. Must be greater than 0.
    #[arg(short='n', value_delimiter = ' ', num_args = 1.., default_values_t = [1,2,3],)]
    skip_nums: Vec<usize>,

    /// Length to pick up from that location. Must be greater than 0.
    #[arg(short = 'l', default_value_t = 1)]
    pick_length: usize,

    /// Offset to start picking within that range. Must be greater than or equal to 0.
    #[arg(short = 'o', default_value_t = 0)]
    pick_offset: usize,

    /// Offset to start parsing the entire file.
    #[arg(short = 'f', default_value_t = 0)]
    file_offset: usize,

    /// Whether to output the file.
    #[arg(short = 'x', visible_aliases = ["output","export","output-file"], long, action)]
    export_file: bool,

    /// Only non bin file.
    #[arg(short = 'e', long, action)]
    only: bool,

    /// Print head of buffer.
    #[arg(short, long, action)]
    print: bool,

    /// Output directory path.
    #[arg(long, default_value = "./skiphead_out")]
    output_directory: String,

    file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut cmd = Args::command();

    let mut file = match File::open(&args.file) {
        Ok(f) => f,
        Err(_) => {
            println!("Can't open file");
            return;
        }
    };

    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).expect("Can't read");

    if args.skip_nums.contains(&0) {
        println!("Number of skips must be greater than 0.");
        _ = cmd.print_help();
        return;
    }

    if args.export_file {
        if !fs::metadata(&args.output_directory).is_ok() {
            let _ = match fs::create_dir(&args.output_directory) {
                Ok(f) => f,
                Err(_) => {
                    println!("Can't create {} directory..", &args.output_directory);
                    return;
                }
            };
        }
    }

    let mut sorted_skip_nums: Vec<usize> = args.skip_nums.clone();
    sorted_skip_nums.sort();

    let mut exists_non_bin = false;

    for skip in sorted_skip_nums.iter() {
        let skip_picked_data: Vec<u8> = skiped_and_picked_file_buf(
            *skip,
            args.pick_offset,
            args.pick_length,
            args.file_offset,
            &file_buf,
        );
        let fmt: FileFormat = FileFormat::from_bytes(&skip_picked_data);

        if matches!(fmt, FileFormat::ArbitraryBinaryData) {
            if args.only {
                continue;
            }
        } else {
            exists_non_bin = true;
        }

        print_result(
            *skip,
            args.pick_offset,
            args.pick_length,
            args.file_offset,
            &fmt,
        );

        if args.print {
            println!("{:x?}", &skip_picked_data[0..32]);
        }
        println!();

        if args.export_file {
            match output_file(
                *skip,
                args.pick_offset,
                args.pick_length,
                args.file_offset,
                &args.output_directory,
                &skip_picked_data,
            ) {
                Ok(_) => {}
                Err(_) => return,
            };
        }
    }

    if exists_non_bin == false && args.only {
        println!("Non-binary files was not detected.");
    }
}
