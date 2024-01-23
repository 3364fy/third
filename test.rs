fn start_simulate(inppaths: Vec<String>, selected_path: String) {
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use std::process::Command;
    use std::path::Path;
    use std::thread;

    thread::spawn(move || {
        print!("Selected path: {}", selected_path);
        println!("Input paths: {:?}", inppaths);
        let log_txt_path = Path::new(&selected_path).join("log.txt");
        let mut log_txt = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&log_txt_path)
            .expect("Failed to open log.txt");

        for (index, path) in inppaths.iter().enumerate() {
            println!("{}: {}", index, path);
            let path = Path::new(&path);
            let filename = path.file_stem().expect("Failed to get file name");
            let inpname = filename.to_str().expect("Failed to convert to string");

            let parent_dir = path.parent().expect("Failed to get parent directory");
            println!("Parent directory: {:?}", parent_dir);

            writeln!(log_txt, "{}开始模拟", path.display()).expect("Failed to write to log.txt");

            let log_ll_path = Path::new(&selected_path).join("log.ll");
            let mut log_ll = File::create(&log_ll_path).expect("Failed to create log.ll");
            writeln!(log_ll, "{}", index).expect("Failed to write to log.ll");

            let mut cmd = Command::new("cmd");
            cmd.current_dir(parent_dir);
            cmd.arg("/c");
            cmd.arg("call");
            cmd.arg("abq2022");
            cmd.arg(format!("job={}", inpname));
            cmd.arg("cpus=1");
            cmd.arg("int");
            cmd.output().expect("Failed to execute process");
            println!("Executing command: {:?}", cmd);

            writeln!(log_txt, "{}模拟完成", path.display()).expect("Failed to write to log.txt");
        }
    });
}

fn main() {
    let inppaths = vec![String::from("G:\\Model\\Abaqus\\project\\cross-layer.inp")];
    let selected_path = String::from("G:\\Model\\Abaqus\\project");
    start_simulate(inppaths, selected_path);
}
