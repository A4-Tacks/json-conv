use std::io::{read_to_string, stdin};

use json_conv::{Spec, SpecDe, SpecSer};

fn parse_format(s: String) -> Spec {
    match s.to_lowercase().as_str() {
        "json" => Spec::Json,
        "5" | "json5" => Spec::Json5,
        "h" | "hjson" => Spec::HJson,
        _ => panic!("Invalid json spec format: {s:?}, expect like json, json5 etc"),
    }
}

fn main() {
    let options = getopts_macro::getopts_options! {
        -f, --from=FORMAT   "from format (default: json5)";
        -t, --to=FORMAT     "to format (default: json)";
        -c, --compact       "use compact output";
        -h, --help*         "show help message";
        -v, --version*      "show version message";
    };
    let matches = getopts_macro::simple_parse(
        &options,
        env!("CARGO_PKG_DESCRIPTION"),
        0,
        "[FILE]",
    );
    let from = matches.opt_str("from").map_or(Spec::Json5, parse_format);
    let to = matches.opt_str("to").map_or(Spec::Json, parse_format);
    let compact = matches.opt_present("compact");

    if matches.opt_present("version") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if matches.free.len() > 1 {
        eprintln!("Extra position arguments: expected 0 or 1, found {}", matches.free.len());
        std::process::exit(2)
    }

    let input = if let Some(path) = matches.free.first() {
        fs_err::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(2)
        })
    } else {
        read_to_string(stdin().lock()).unwrap()
    };
    let value = match serde_json::Value::from_specjson(from, &input) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Can't parse input: {e}");
            std::process::exit(1)
        },
    };
    if compact {
        println!("{}", value.to_specjson(to).unwrap())
    } else {
        println!("{}", value.to_specjson_prettify(to).unwrap())
    }
}
