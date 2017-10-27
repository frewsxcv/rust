use std::{env, error, fs, path};
use std::io::Write;

fn run<S, D>(src_path: S, dest_path: D) -> Result<(), Box<error::Error>>
where
    S: AsRef<path::Path>,
    D: AsRef<path::Path>,
{
    let mut buf = String::new();
    buf.push_str("# RFCS\n\n");

    fs::create_dir_all(&dest_path)?;

    let mut rfcs_file_names = src_path
        .as_ref()
        .read_dir()?
        .map(|d| d.unwrap().path().file_name().unwrap().to_owned())
        .collect::<Vec<_>>();

    rfcs_file_names.sort();

    for i in rfcs_file_names {
        buf.push_str(&format!(
            "- [{}]({})\n",
            i.to_str().unwrap().trim_right_matches(".md"),
            i.to_str().unwrap()
        ));
    }

    let mut file = fs::File::create(dest_path.as_ref().join("SUMMARY.md"))?;
    file.write_all(buf.as_bytes())?;

    for d in src_path.as_ref().read_dir()? {
        let d = d.unwrap();
        fs::copy(d.path(), dest_path.as_ref().join(d.file_name())).expect("could not copy");
    }

    Ok(())
}

fn main() {
    let src_path_str = env::args_os().skip(1).next().expect("source path required");
    let src_path = path::Path::new(&src_path_str);

    let dest_path_str = env::args_os().skip(2).next().expect(
        "destination path required",
    );
    let dest_path = path::Path::new(&dest_path_str).join("src");

    run(src_path, dest_path).unwrap();
}
