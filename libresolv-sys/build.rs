use std::error::Error;
use std::fs::{self, File};
use std::io::Write; // File.write_all()
use std::path::PathBuf;
use std::process::Command;
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    static BINDING: &str = "resolv.rs";
    let header = format!("{}/resolv.h", std::env::var("GLIBC_INCLUDE").unwrap_or("/usr/include".into()));
    let mut cmd;
    let mut output;

    eprintln!("Generating binding {:?} from {:?} ...\n", BINDING, header);
    cmd = Command::new("bindgen");
    cmd.args(["--with-derive-default", &header]);
    output = cmd.output()?;
    if !output.status.success() {
        let msg = str::from_utf8(output.stderr.as_slice())?;
        eprintln!("\"\"\"\n{}\"\"\"\n", msg);
        panic!("{:?}", cmd);
    }

    let lints = fs::read("resolv.lints")?;
    let mut f = File::create(BINDING)?;
    f.write_all(lints.as_slice())?;
    f.write_all(output.stdout.as_slice())?;

    eprintln!(
        "Checking available libresolv adapters to the generated binding {:?} ...\n",
        BINDING
    );

    let mut paths: Vec<PathBuf> = fs::read_dir("lib.d")?.map(|e| e.unwrap().path()).collect();
    paths.sort_unstable();
    for path in paths.iter().rev() {
        eprintln!("Trying {:?} ...\n", path);
        fs::copy(path, "lib.rs")?;
        cmd = Command::new("rustc");
        cmd.args(["--emit", "dep-info=/dev/null", "lib.rs"]);
        output = cmd.output()?;
        if output.status.success() {
            eprintln!("Success\n");
            break;
        } else {
            let msg = str::from_utf8(output.stderr.as_slice())?;
            eprintln!("\"\"\"\n{}\"\"\"\n", msg);
            fs::remove_file("lib.rs")?;
        }
    }

    if !output.status.success() {
        panic!("None of the available adapters compiled successfully!");
    }

    println!("cargo:rustc-flags=-l resolv");

    Ok(())
}
