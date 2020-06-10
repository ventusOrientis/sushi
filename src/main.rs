use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    loop{
        print!("sushi:");
        stdout().flush();

        let mut user_eingabe = String::new();
        stdin().read_line(&mut user_eingabe).unwrap();

        //new-line symbol aus stdin entfernen
        let befehl = user_eingabe.trim();

        let mut child = Command::new("cmd")
            .args(&["/C", befehl])
            .spawn()
            .unwrap();

        child.wait();
    }
}