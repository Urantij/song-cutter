use regex::Regex;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let args: Vec<_> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        return Err("первый аргумент".into());
    }

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

    let file_name = file_path
        .file_name()
        .unwrap()
        .to_str()
        .ok_or_else(|| "плохое имя")?;
    let temp_dir_name = format!("{}_dir_temp", file_name);

    let temp_dir_path = file_path.with_file_name(temp_dir_name);

    fs::create_dir(&temp_dir_path)?;

    spawn_strochkas_files(&temp_dir_path, &file_content, &mut selected)?;

    {
        let temp_file_path = file_path.with_file_name(format!("{}_temp", file_name));
        fs::rename(&file_path, &temp_file_path)?;

        fs::rename(&temp_dir_path, &file_path)?;
        fs::remove_file(temp_file_path)?;
    }

    Ok(())
}

fn spawn_strochkas_files(
    dir_path: &PathBuf,
    file_content: &str,
    strochkas: &mut Vec<&str>,
) -> Result<(), Box<dyn Error>> {
    let first = strochkas.pop().unwrap();
    let first_file_name = format_strochka_to_file_name(first);
    let first_file_path = dir_path.join(first_file_name);

    fs::write(&first_file_path, &file_content)?;

    while let Some(strochka) = strochkas.pop() {
        let strochka_file_name = format_strochka_to_file_name(strochka);
        let strochka_file_path = dir_path.join(strochka_file_name);

        fs::hard_link(&first_file_path, strochka_file_path)?;
    }

    Ok(())
}

fn select_strochkas(content: &str) -> Result<Vec<&str>, Box<dyn Error>> {
    let options: Vec<&str> = content.split("\n").collect();

    for (index, item) in options.iter().enumerate() {
        println!("[{}] {}", index + 1, item);
    }

    println!("Через пробел введи номера элементов, которые нужно добавить как строки.");
    println!("Просто ентер, если нужно всё.");
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let selected: Vec<usize> = Regex::new(r"(\D|^)(?<num>\d+)")
        .unwrap()
        .captures_iter(&line)
        .map(|m| m["num"].parse::<usize>().unwrap() - 1usize)
        .collect();

    if selected.len() == 0 {
        return Ok(options);
    }

    let selected: Vec<_> = options
        .iter()
        .enumerate()
        .filter(|(index, _)| selected.contains(index))
        .map(|(_, item)| *item)
        .collect();

    Ok(selected)
}

fn format_strochka_to_file_name(strochka: &str) -> String {
    // к сож dmenu с -i аргументом не работает с кириллицей. увы.
    strochka.to_lowercase()
}
