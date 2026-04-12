use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    static BINDING: &str = "resolv.rs";
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let header = format!(
        "{}/resolv.h",
        std::env::var("GLIBC_INCLUDE").unwrap_or_else(|_| "/usr/include".into())
    );

    eprintln!("Generating binding {BINDING:?} from {header:?} ...\n");
    let bindings = bindgen::Builder::default()
        .header(&header)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file(out_dir.join(BINDING))?;

    eprintln!("Checking available libresolv adapters to the generated binding {BINDING:?} ...\n");

    let mut paths: Vec<PathBuf> = fs::read_dir("lib.d")?.map(|e| e.unwrap().path()).collect();
    paths.sort_unstable();
    let mut selected_path = None;
    for path in paths.iter().rev() {
        eprintln!("Trying {:?} ...\n", path.display());
        fs::copy(path, out_dir.join("lib.rs"))?;
        let mut cmd = Command::new("rustc");
        cmd.args(["--emit", "dep-info=/dev/null", "lib.rs"]);
        cmd.current_dir(&out_dir);
        let output = cmd.output()?;
        if output.status.success() {
            eprintln!("Success\n");
            selected_path = Some(path.clone());
            break;
        }
        let msg = str::from_utf8(output.stderr.as_slice())?;
        eprintln!("\"\"\"\n{msg}\"\"\"\n");
        fs::remove_file(out_dir.join("lib.rs"))?;
    }

    let selected_path = selected_path.expect("No available adapters compiled successfully");
    let adapter = fs::read_to_string(selected_path)?;
    let generated = adapter.replace(
        "mod resolv;",
        r#"#[allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_imports)]
mod resolv {
    include!(concat!(env!("OUT_DIR"), "/resolv.rs"));
}"#,
    );
    fs::write(out_dir.join("lib_generated.rs"), generated)?;

    println!("cargo:rustc-flags=-l resolv");

    Ok(())
}
