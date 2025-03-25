use peg;
use std::time::Duration;
use std::io::{stdin, stdout, Write};

peg::parser!{
    grammar parser() for str {
        pub rule digit() -> u64
        = n:$(['0'..='9']) {? n.parse().or(Ok(0)) }

        rule optional_pair() -> u64
        = n:$(digit()*) {? n.parse().or(Ok(0)) }

        rule non_empty_pair() -> u64
        = n:$(digit()+) {? n.parse().or(Ok(0)) }

        pub rule only_hours() -> Duration
        = nh:non_empty_pair() [':'] nm:optional_pair() [':']? ns:optional_pair()?
         { Duration::from_secs((nh*3600)+(nm*60)+(ns.unwrap_or(0))) }

        pub rule only_minutes() -> Duration
        = nh:optional_pair() [':'] nm:non_empty_pair() [':']? ns:optional_pair()?
        { Duration::from_secs((nh*3600)+(nm*60)+(ns.unwrap_or(0))) }

        pub rule only_seconds() -> Duration
        = nh:optional_pair() [':'] nm:optional_pair() [':'] ns:non_empty_pair()
         { Duration::from_secs((nh*3600)+(nm*60)+(ns)) }

        pub rule time_val() -> Duration
        = only_hours() / only_minutes() / only_seconds()

        pub rule whitespace() = quiet!{[' ' | '\t']+}
        pub rule parse() -> Duration
        = whitespace()? a:time_val() whitespace()? b:$(['+'|'-']) whitespace()? c:time_val() {
            match b {
                "+" => a + c,
                "-" => a - c,
                &_ => todo!()
        } }
    }
}

pub fn get_input(prompt: &str) -> String{
    let _ = stdout().write(format!("{}",prompt).as_bytes());
    let _ = stdout().flush();
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => {},
    }
    input.trim().to_string()
}

#[test]
fn test1() {
    assert_eq!(parser::parse("36:40:13 - ::13"), Ok(Duration::from_secs(36*3600 + 40 * 60)));
}

pub fn timecalc() -> Result<(),()> {
    loop {
        let a = get_input(">>> ");
        let b = parser::parse(&a);
        if let Ok(c) = b {
            let mut k = c.as_secs();
            let days = match k/86400 {
                0 => "",
                n => { k %= 86400;
                    &format!("{n} d")}
            };
            let hours = match k/3600 {
                0 => "",
                n => {k %= 3600; &format!("{n} h")}
            };
            let mins = match k/60 {
                0 => "",
                n => {k %= 60;&format!("{n} m")}
            };
            let secs = if k>0 {&format!("{k} s")} else {""};
            println!("{} {} {} {}", days, hours, mins, secs);
        }
    }
    Ok(())
}
