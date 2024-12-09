use song_cutter::{get_file_name, select_strochkas, spawn_strochkas_files};
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let args: Vec<_> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        return Err("первый аргумент".into());
    }

    let lower_case = std::env::var("LOWERCASE").is_ok();

    let file_path = args.first().unwrap();
    let file_path = Path::new(file_path);

    if fs::exists(file_path)? == false {
        return Err("файла нет".into());
    }

    let file_content = fs::read_to_string(file_path)?;

    let mut selected = select_strochkas(&file_content)?;

    if selected.len() == 0 {
        return Err("Че ты написал".into());
    }

    let file_name = get_file_name(file_path)?;

    let temp_dir_name = format!("{}_dir_temp", file_name);
    let temp_dir_path = file_path.with_file_name(temp_dir_name);

    fs::create_dir(&temp_dir_path)?;

    spawn_strochkas_files(&temp_dir_path, &file_content, &mut selected, lower_case)?;

    {
        let temp_file_path = file_path.with_file_name(format!("{}_temp", file_name));
        fs::rename(&file_path, &temp_file_path)?;

        fs::rename(&temp_dir_path, &file_path)?;
        fs::remove_file(temp_file_path)?;
    }

    Ok(())
}
