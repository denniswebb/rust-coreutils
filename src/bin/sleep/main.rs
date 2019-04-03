use humantime;
use std::{env::args, path::Path, process::exit, thread, time::Duration};

fn main() {
    if args().len() <= 1 {
        usage();
        exit(0)
    }

    for arg in args().skip(1) {
        sleep(str_to_duration(&arg));
    }
}

fn sleep(duration: Duration) {
    if duration > Duration::from_millis(0) {
        dbg!(duration);
        thread::sleep(duration)
    }
}

fn str_to_duration(input: &str) -> Duration {
    match input.parse::<f64>() {
        Ok(seconds) => Duration::from_millis((seconds * 1000.0) as u64),
        Err(_) => humantime::parse_duration(input).unwrap_or_default(),
    }
}

fn usage() {
    println!(
"usage: {cmd} NUMBER[SUFFIX]...

examples: {cmd} 5
          {cmd} 1m 30s",
        cmd = Path::new(&(args().nth(0).unwrap()))
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    )
}

#[cfg(test)]
mod should {
    use super::*;
    use std::time::Duration;

    #[test]
    fn convert_cmplex_string_to_duration() {
        assert_eq!(Duration::from_secs((60*60*24) + (60*60*2) + (60*3) + 4), str_to_duration("1d2h3m4s"))
    }

    #[test]
    fn convert_junk_string_to_0_seconds() {
        assert_eq!(Duration::from_secs(0), str_to_duration("rustaceans"))
    }

    #[test]
    fn convert_number_to_seconds() {
        assert_eq!(Duration::from_secs(5), str_to_duration("5"))
    }

    #[test]
    fn convert_string_to_duration() {
        assert_eq!(Duration::from_secs(60), str_to_duration("1m"))
    }
}