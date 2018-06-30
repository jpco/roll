extern crate rollerator;

use std::env;
use std::str;
use std::io::{self, Read, Write};
use std::collections::VecDeque;
use std::process;

use rollerator::Rollable;

fn by_args(a: env::Args) {
    println!(
        "{}",
        a.skip(1).map(|s| s.rolled()).collect::<Vec<_>>().join(" ")
    )
}

fn exit_err(s: &'static str, e: io::Error) -> ! {
    eprintln!("{}: {}", s, e);
    process::exit(e.raw_os_error().unwrap_or(0));
}

const BUF_SIZE: usize = 4096;

// FIXME: there are a couple ways NLL will collapse lines in this function
fn by_stdin() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut rbuf = [0; BUF_SIZE];
    let mut input = VecDeque::new();
    input.reserve(BUF_SIZE);
    loop {
        match stdin.read(&mut rbuf) {
            Err(e) => exit_err("reading stdin", e),
            Ok(0) => break,
            Ok(n) => {
                input.extend(rbuf[..n].iter());
                let mut cont = true;
                while cont {
                    cont = false;
                    if let Err(e) = match str::from_utf8(&input.iter().cloned().collect::<Vec<_>>())
                    {
                        Ok(s) => {
                            input.clear();
                            stdout.write(&s.rolled().bytes().collect::<Vec<_>>())
                        }
                        Err(e) => unsafe {
                            let x = input.drain(..e.valid_up_to()).collect::<Vec<_>>();
                            let rs = str::from_utf8_unchecked(&x).rolled();
                            let b: Vec<u8> = match e.error_len() {
                                None => rs.bytes().collect(),
                                Some(ni) => {
                                    cont = true;
                                    rs.bytes().chain(input.drain(..ni)).collect()
                                }
                            };

                            stdout.write(&b)
                        },
                    } {
                        exit_err("writing stdout", e);
                    }
                }
            }
        }
    }

    if input.len() > 0 {
        if let Err(e) = stdout.write(&input.drain(..).collect::<Vec<_>>()) {
            exit_err("writing stdout", e);
        }
    }
}

fn main() {
    let a = env::args();
    if a.len() > 1 {
        by_args(a);
    } else {
        by_stdin();
    }
}
