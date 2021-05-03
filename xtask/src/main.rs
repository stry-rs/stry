mod ark;

use {
    chrono::Utc,
    std::{
        fs::{self, File},
        io::{Result, Write as _},
        path::PathBuf,
    },
};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("codegen") => match args.next().as_deref() {
            Some("proto") => {}
            None => {}
            Some(_) => {}
        },
        Some("new") => match args.next().as_deref() {
            Some("migration") => {
                let name = args
                    .next()
                    .expect("there must be a <NAME> argument to `xtask new migration`");
                let folder = args.next().unwrap_or_else(|| "./migrations".to_string());

                new_migration(&name, &folder)?;
            }
            None => {}
            Some(_) => {}
        },
        Some("--help") | None => {
            eprintln!("Tasks:");
            eprintln!();
            eprintln!("codegen");
            eprintln!("    proto        <FILE>");
            eprintln!("new");
            eprintln!("    migration    <NAME> <FOLDER>");
        }
        Some(cmd) => {
            eprintln!("unknown command: `{}`", cmd);
        }
    }

    Ok(())
}

fn new_migration(name: &str, folder: &str) -> std::io::Result<()> {
    fs::create_dir_all(folder)?;

    let dt = Utc::now();
    let file_prefix = dt.format("%Y%m%d%H%M%S").to_string();

    let mut path = PathBuf::new();

    path.push(folder);
    path.push(&format!("{}_{}.sql", file_prefix, name.replace(' ', "_")));

    println!("Creating {}", path.display());

    let mut file = File::create(&path)?;

    file.write_all("-- Add migration script here\n".as_bytes())?;

    Ok(())
}
