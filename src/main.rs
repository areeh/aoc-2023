#![feature(test)]

mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::{env, fs};

use anyhow::Result;
use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Utc};
use curl::easy::Easy;

const TOKEN_ENV: &str = "TOKEN";

fn aoc_now() -> DateTime<FixedOffset> {
    FixedOffset::west_opt(18_000)
        .unwrap()
        .from_utc_datetime(&Utc::now().naive_utc())
}

fn latest_aoc_year_day() -> (i32, u32) {
    let now = aoc_now();
    if now.month() != 12 {
        println!("not AoC yet, returning last day for last year");
        (now.year() - 1, 25u32)
    } else if now.day() > 25 {
        (now.year(), 25u32)
    } else {
        (now.year(), now.day())
    }
}

fn make_day(year: i32, day: u32) -> Result<()> {
    let mut day_dir = PathBuf::from("./src/");
    day_dir.push(format!("day{day}"));

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    match fs::create_dir(&day_dir) {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => (),
            other_error => {
                panic!("Problem creating directory: {other_error:?}")
            }
        },
    }

    let mut input_path = day_dir.clone();
    input_path.push("input.txt");

    if !input_path.exists() {
        let mut file = File::create(&input_path)?;

        let mut easy = Easy::new();
        let token = env::var(TOKEN_ENV)?;
        easy.useragent("https://github.com/areeh")?;
        easy.cookie(&format!("session={token}")).unwrap();
        easy.url(&url).unwrap();
        easy.write_function(move |data| {
            file.write_all(data).unwrap();
            Ok(data.len())
        })
        .unwrap();
        easy.get(true).unwrap();
        easy.perform()
            .unwrap_or_else(|_| panic!("Encountered error when performing request to {:?}", &url));

        if easy.response_code().unwrap() != 200 {
            panic!(
                "Got response code {} with url {}",
                easy.response_code().unwrap(),
                url
            );
        };
    }

    let mut rs_path = day_dir.clone();
    rs_path.push("mod.rs");

    if !rs_path.exists() {
        let template = PathBuf::from("./src/template.rs");
        let _ = File::create(&rs_path)?;
        std::fs::copy(template, rs_path)?;
    }

    Ok(())
}

fn make_until_today() -> Result<()> {
    let (year, day) = latest_aoc_year_day();
    (1..day + 1).try_for_each(|x| make_day(year, x))
}

fn main() -> Result<()> {
    make_until_today()?;
    day1::main()?;
    day2::main()?;
    day3::main()?;
    day4::main()?;

    Ok(())
}
