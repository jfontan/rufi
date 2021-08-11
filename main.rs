use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("ARGS: {:?}", args);

    let search_default = String::from(".");
    let path_default = String::from(".");

    let search = args.get(1).unwrap_or(&search_default);
    let path = args.get(2).unwrap_or(&path_default);

    let re = Regex::new(search).unwrap();

    let mut paths: Vec<String> = Vec::new();
    paths.push(String::from(path));

    loop {
        if let Some(last) = paths.pop() {
            let dirs = process(&last, &re);
            paths.extend(dirs)
        } else {
            break;
        }
    }
}

fn process(path: &str, re: &Regex) -> Vec<String> {
    let mut dirs: Vec<String> = Vec::new();

    let res = std::fs::read_dir(path);
    if let Err(err) = res {
        println!("ERROR: {}, {}", path, err);
        return dirs;
    }

    for d in res.unwrap() {
        if let Err(err) = d {
            println!("ERROR: {}", err);
            continue;
        }

        let f = d.unwrap();
        let p = f.path().to_str().unwrap().to_string();

        if re.is_match(&p) {
            println!("{}", p);
        }

        if let Err(err) = f.file_type() {
            println!("ERROR: {}", err);
            continue;
        }

        let t = f.file_type().unwrap();

        if t.is_dir() {
            dirs.push(p);
        }
    }

    dirs
}
