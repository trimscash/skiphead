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
        let start_index = index + pick_offset.min(buf.len() - index);
        result.extend_from_slice(&buf[start_index..end_index]);
        index += skip_num;
    }

    result
}

/// Parse the header of the file skipped by n bytes and display the file type.{n}
/// skiphead can search for file types by combining parameters.{n}
/// Forensic app.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Number of skips. Must be greater than 0.
    #[arg(short='s',visible_short_alias='n',visible_aliases=["skip"], value_delimiter = ' ', num_args = 1.., default_values_t = [1,2,3],)]
    skip_nums: Vec<usize>,

    /// Length to pick up from that location. Must be greater than 0.
    #[arg(short = 'l', value_delimiter = ' ',visible_aliases=["pick_length"], num_args = 1.., default_values_t = [1])]
    pick_length: Vec<usize>,

    /// Offset to start picking within that range. Must be greater than or equal to 0.
    #[arg(short = 'o', value_delimiter = ' ', visible_aliases=["pick_offset"],num_args = 1.., default_values_t = [0])]
    pick_offset: Vec<usize>,

    /// Offset to start parsing the entire file.
    #[arg(short = 'f', visible_aliases=["file_offset"],default_value_t = 0)]
    file_offset: usize,

    /// Combinate param mode. default mode is one on one.
    #[arg(short = 'c', long, action)]
    combinate: bool,

    /// Whether to output the file.
    #[arg(short = 'x', visible_short_alias='e', visible_aliases = ["output","export","output-file"], long, action)]
    export_file: bool,

    /// Only non bin file.
    #[arg(short = 'z', long, action)]
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

    // validate params
    let red = color::Fg(color::Red);
    let reset = color::Fg(color::Reset);

    if !args.skip_nums.iter().all(|&e| e > 0) {
        println!("{}Number of skips must be greater than 0.{}\n", red, reset);
        println!("  -h, --help");
        println!("          Print help");
        return;
    }
    if !args.pick_length.iter().all(|&e| e > 0) {
        println!(
            "{}Length to pick up must be greater than 0.{}\n",
            red, reset
        );
        println!("  -h, --help");
        println!("          Print help");
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

    let mut exists_non_bin = false;

    if args.combinate {
        for skip_num in args.skip_nums.iter() {
            for pick_length in args.pick_length.iter() {
                for pick_offset in args.pick_offset.iter() {
                    match do_skip(
                        *skip_num,
                        *pick_offset,
                        *pick_length,
                        args.file_offset,
                        &file_buf,
                        &args,
                    ) {
                        Ok(is_non_bin) => {
                            exists_non_bin = is_non_bin || exists_non_bin;
                        }
                        Err(_) => return,
                    };
                }
            }
        }
    } else {
        let mut skip_nums: Vec<usize> = args.skip_nums.clone();
        let mut pick_lengths: Vec<usize> = args.pick_length.clone();
        let mut pick_offsets: Vec<usize> = args.pick_offset.clone();

        let lens: Vec<usize> = Vec::from([skip_nums.len(), pick_lengths.len(), pick_offsets.len()]);
        let max_length = *lens.iter().max().unwrap();

        skip_nums.resize(max_length, 0);
        pick_lengths.resize(max_length, 0);
        pick_offsets.resize(max_length, 0);

        for ((skip_num, pick_offset), pick_length) in skip_nums
            .iter()
            .zip(pick_offsets.iter())
            .zip(pick_lengths.iter())
        {
            match do_skip(
                *skip_num,
                *pick_offset,
                *pick_length,
                args.file_offset,
                &file_buf,
                &args,
            ) {
                Ok(is_non_bin) => {
                    exists_non_bin = is_non_bin || exists_non_bin;
                }
                Err(_) => return,
            };
        }
    }

    if exists_non_bin == false && args.only {
        println!("Non-binary files was not detected.");
    }
}

// return whether file type is non-bin
fn do_skip(
    skip_num: usize,
    pick_offset: usize,
    pick_length: usize,
    file_offset: usize,
    file_buf: &Vec<u8>,
    args: &Args,
) -> Result<bool, String> {
    let skip_picked_data: Vec<u8> =
        skiped_and_picked_file_buf(skip_num, pick_offset, pick_length, file_offset, &file_buf);
    let fmt: FileFormat = FileFormat::from_bytes(&skip_picked_data);

    #[warn(unused_assignments)]
    let mut is_non_bin: bool = false;

    if matches!(fmt, FileFormat::ArbitraryBinaryData) {
        is_non_bin = false;

        if args.only {
            return Ok(is_non_bin);
        }
    } else {
        is_non_bin = true;
    }

    print_result(skip_num, pick_offset, pick_length, file_offset, &fmt);

    if args.print {
        println!("{:x?}", &skip_picked_data[0..32]);
    }
    println!();

    if args.export_file {
        match output_file(
            skip_num,
            pick_offset,
            pick_length,
            file_offset,
            &args.output_directory,
            &skip_picked_data,
        ) {
            Ok(_) => return Ok(is_non_bin),
            Err(_) => return Err("Error when outputting to file".to_string()),
        };
    }

    return Ok(is_non_bin);
}
