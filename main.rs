fn main() {
    let str = "hello".to_owned() + "world";
    println!("{}", str);

    let mut paths: Vec<String> = Vec::new();
    paths.push("/home/jfontan".to_owned());

    loop {
        if let Some(last) = paths.pop() {
            let dirs = process(&last);
            paths.extend(dirs)
        } else {
            break;
        }
    }
}

fn process(path: &str) -> Vec<String> {
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
        let pp = f.path();
        let p = pp.to_str().unwrap();
        println!("{}", p);

        if let Err(err) = f.file_type() {
            println!("ERROR: {}", err);
            continue;
        }

        let t = f.file_type().unwrap();

        if t.is_dir() {
            dirs.push(p.to_string());
        }
    }

    dirs
}
