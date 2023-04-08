use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process::{exit, Command};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct VersionData {
    folder: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ConanPackageConfig {
    versions: HashMap<String, VersionData>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::from("recipes"))]
    recipes_dir: String,
}

#[derive(Debug)]
struct ConanArgs {
    path: std::path::PathBuf,
    version: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let conan_args = WalkDir::new(args.recipes_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .flat_map(|entry| -> Result<Vec<ConanArgs>, Box<dyn Error>> {
            let f_name = entry.file_name().to_string_lossy();
            if f_name == "config.yml" {
                let file = File::open(entry.path())?;
                let reader = BufReader::new(file);

                let config: ConanPackageConfig = serde_yaml::from_reader(reader)?;

                let base_path = entry.path().parent().unwrap();
                return Ok(config
                    .versions
                    .iter()
                    .map(|(version, data)| ConanArgs {
                        path: base_path.join(data.folder.clone()),
                        version: version.clone(),
                    })
                    .collect::<Vec<ConanArgs>>());
            }
            Ok(vec![])
        })
        .flatten()
        .collect::<Vec<_>>();

    for args in conan_args {
        println!(
            "-----------------------------\nconan export {} --version {}",
            args.path.display(),
            args.version
        );
        let output = Command::new("conan")
            .arg("export")
            .arg(args.path)
            .arg("--version")
            .arg(args.version)
            .output()
            .expect("Failed to execute process");

        print!("{}", String::from_utf8_lossy(&output.stdout));
        eprint!("{}", String::from_utf8_lossy(&output.stderr));

        if !output.status.success() {
            let code = output.status.code().unwrap();
            eprintln!("Exited with error code: {}", code);
            exit(code);
        }
    }

    Ok(())
}
