use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("ARGS: {:?}", args);

    let search_default = String::from(".");
    let path_default = String::from(".");

    let search = args.get(1).unwrap_or(&search_default);
    let path = args.get(2).unwrap_or(&path_default);

    if true {
        find_parallel(path, search);
    } else {
        find_sequential(path, search);
    }
}

fn find_parallel(path: &String, search: &String) {
    let workers = 16;

    let (jobs_s, jobs_r) = crossbeam_channel::bounded::<String>(0);
    let (dirs_s, dirs_r) = crossbeam_channel::bounded::<Vec<String>>(0);

    let re = Regex::new(search).unwrap();

    for _i in 0..workers {
        let jr = jobs_r.clone();
        let ds = dirs_s.clone();
        let rec = re.clone();

        std::thread::spawn(move || loop {
            let job = jr.recv().unwrap();
            let dirs = process(&job, &rec);
            ds.send(dirs).unwrap();
        });
    }

    let mut paths: Vec<String> = Vec::new();
    let mut dir = String::from(path);
    let mut value = true;
    let mut active = 0;

    loop {
        if value {
            crossbeam_channel::select! {
                send(jobs_s, dir.clone()) -> res => {
                    if res.is_err() {
                        break;
                    }

                    res.unwrap();
                    active+=1;
                    if let Some(d) = paths.pop() {
                        dir = d;
                        value = true;
                    } else {
                        value = false;
                    }
                }
                recv(dirs_r) -> res => {
                    if res.is_err() {
                        break;
                    }

                    active-=1;
                    let dirs = res.unwrap();
                    paths.extend(dirs);
                }
            }
        } else {
            if active == 0 {
                break;
            }

            let res = dirs_r.recv();
            if res.is_err() {
                break;
            }

            let dirs = res.unwrap();
            active -= 1;
            paths.extend(dirs);

            if let Some(d) = paths.pop() {
                dir = d;
                value = true;
            } else {
                value = false;
            }
        }
    }
}

fn find_sequential(path: &String, search: &String) {
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
