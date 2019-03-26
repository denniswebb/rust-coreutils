use humantime;
use std::{env::args, path::Path, process::exit, thread, time::Duration};

fn main() {
    if args().len() <= 1 {
        usage();
        exit(0)
    }

    for arg in args().skip(1) {
        let duration = match arg.parse::<f64>() {
            Ok(seconds) => Duration::from_millis((seconds * 1000.0) as u64),
            Err(_) => humantime::parse_duration(&arg).unwrap_or_default(),
        };

        sleep(duration);
    }
}

fn sleep(duration: Duration) -> () {
    if duration > Duration::from_millis(0) {
        thread::sleep(duration)
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
