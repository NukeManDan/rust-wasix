use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Read, Seek, Write};
use std::path::PathBuf;

/// This is printed to the file before the rest of the contents.
const PRELUDE: &str = r#"// This file is autogenerated.
//
// To add bindings, edit windows_sys.lst then use `./x run generate-windows-sys` to
// regenerate the bindings.
//
// ignore-tidy-filelength
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let mut path: PathBuf =
        env::args_os().nth(1).expect("a path to the rust repository is required").into();
    path.push("library/std/src/sys/windows/c");
    env::set_current_dir(&path)?;

    let info = windows_bindgen::bindgen(["--etc", "windows_sys.lst"])?;
    println!("{info}");

    // add some gunk to the output file.
    let mut f = fs::File::options().read(true).write(true).open("windows_sys.rs")?;
    let mut bindings = String::new();
    f.read_to_string(&mut bindings)?;
    f.seek(io::SeekFrom::Start(0))?;
    f.write_all(PRELUDE.as_bytes())?;
    f.write_all(bindings.as_bytes())?;

    Ok(())
}
