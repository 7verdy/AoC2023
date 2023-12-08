use anyhow::Result;
use reqwest::{self, Response};
use std::fs::File;
use std::io::copy;
use std::io::prelude::*;

pub fn dl_file_from_url(url_str: String) -> Result<()> {
    if File::open("input.txt").is_ok() {
        return Ok(());
    }
    let jar = reqwest::cookie::Jar::default();
    let url = reqwest::Url::parse(&url_str)?;

    let mut cookie_file = File::open("cookie.txt")?;
    let mut cookie = String::new();
    cookie_file.read_to_string(&mut cookie)?;
    jar.add_cookie_str(format!("session={}", cookie).as_str(), &url);

    let client = reqwest::blocking::Client::builder()
        .cookie_provider(jar.into())
        .build()?;
    let mut response = client.get(url).send()?;
    println!("{:?}", response);
    let mut file = File::create("input.txt")?;
    copy(&mut response, &mut file)?;
    Ok(())
}

pub fn upload_solution(year: u32, day: u32, part: u32, answer: String) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let jar = reqwest::cookie::Jar::default();
    let url = reqwest::Url::parse(&url)?;

    let mut cookie_file = File::open("cookie.txt")?;
    let mut cookie = String::new();
    cookie_file.read_to_string(&mut cookie)?;
    jar.add_cookie_str(format!("session={}", cookie).as_str(), &url);

    let client = reqwest::blocking::Client::builder()
        .cookie_provider(jar.into())
        .build()?;
    let response = client
        .post(url)
        .form(&[("level", part.to_string()), ("answer", answer)])
        .send()?;
    if response.text()?.contains("That's the right answer!") {
        println!("[*] Correct answer!");
    } else {
        println!("[!] Wrong answer!");
    }
    Ok(())
}
