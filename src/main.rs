use clap::{arg, Command};
use std::rc::Rc;

fn cli() -> Command {
    Command::new("volumectl")
        .arg_required_else_help(true)
        .arg(arg!(-g --get "Get current volume"))
        .arg(arg!(-i --inc <VALUE> "Increase volume by this value"))
        .arg(arg!(-d --dec <VALUE> "Decrease volume by this value"))
        .arg(arg!(-t --"toggle-mute" "Toggle mute"))
}

fn get_current_volume() -> Rc<str> {
    let mut amixer = std::process::Command::new("amixer");
    let amixer_with_args = amixer.args(["sget", "'Master'"]);
    let output = amixer_with_args.output().unwrap().stdout;
    if let Ok(output) = std::str::from_utf8(&output) {
        output
            .split('[')
            .nth(1)
            .unwrap()
            .split(']')
            .next()
            .unwrap()
            .trim_matches('%')
            .into()
    } else {
        Rc::from("")
    }
}

fn set_volume(level: u8) {
    if level > 100 {
        return;
    }

    let mut amixer = std::process::Command::new("amixer");
    amixer
        .args(["sset", "'Master'", &format!("{}%", level)])
        .output()
        .unwrap();
}

fn increase_volume(level: u8) {
    let curr_volume: u8 = get_current_volume().parse().unwrap_or(0);
    if curr_volume + level > 100 {
        set_volume(100);
    } else {
        set_volume(curr_volume + level);
    }
}

fn decrease_volume(level: u8) {
    let curr_volume: u8 = get_current_volume().parse().unwrap_or(0);
    if level > curr_volume {
        set_volume(0);
    } else {
        set_volume(curr_volume - level);
    }
}

fn toggle_mute() {
    let mut amixer = std::process::Command::new("amixer");
    let current_volume: u8 = get_current_volume().parse().unwrap();
    if current_volume != 0 {
        std::fs::write(
            "/home/blanktiger/.config/prev_vol",
            format!("{}", current_volume),
        )
        .unwrap();
        set_volume(0);
    } else {
        let prev_vol: u8 =
            std::str::from_utf8(&std::fs::read("/home/blanktiger/.config/prev_vol").unwrap())
                .unwrap()
                .parse()
                .unwrap_or(1);
        dbg!(prev_vol);
        set_volume(prev_vol);
    }
    amixer.args([""]);
}

fn main() {
    let matches = cli().get_matches();

    if let Some(get) = matches.get_one::<bool>("get") {
        if *get {
            println!("{}", get_current_volume());
        }
    }

    if let Some(toggle) = matches.get_one::<bool>("toggle-mute") {
        if *toggle {
            toggle_mute();
        }
    }

    if let Some(inc) = matches.get_one::<String>("inc") {
        let inc: u8 = inc.parse().unwrap();
        increase_volume(inc);
    }

    if let Some(dec) = matches.get_one::<String>("dec") {
        let dec: u8 = dec.parse().unwrap();
        decrease_volume(dec);
    }
}
