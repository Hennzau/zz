use std::path::PathBuf;

use clap::{Parser, Subcommand};
use eyre::Result;
use uv::python::PythonVersion;

#[derive(Parser)]
#[command(
    author = "Enzo Le Van <dev@enzo-le-van.fr>",
    version = env!("CARGO_PKG_VERSION"),
    about = "Python 3.12.6 easy mirror"
)]
struct Cli {
    #[arg()]
    python_args: Vec<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Install {
        package: Option<String>,

        #[arg(short, long, value_name = "FILE")]
        requirement: Option<String>,

        #[arg(short = 'e', long = "editable")]
        editable: bool,
    },

    Dir,
}

async fn get_python_bin() -> Result<PathBuf> {
    let mut path = std::env::current_dir()?;

    let venv = loop {
        let venv = path.join(".venv");
        if venv.exists() {
            break Some(venv);
        }

        if !path.pop() {
            break None;
        }
    };

    if let Some(venv) = venv {
        #[cfg(target_os = "windows")]
        let bin = venv.join("Scripts").join("python.exe");
        #[cfg(not(target_os = "windows"))]
        let bin = venv.join("bin").join("python");

        return Ok(bin);
    }

    let uv = uv::UV::new()?;
    let bin = uv
        .python_dir(PythonVersion::from_str("3.12.6").ok())
        .await?;

    return Ok(bin);
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Init => {
                let mut uv = uv::UV::new()?;
                uv.install_bin().await?;

                uv.install_python(PythonVersion::from_str("3.12.6")?)
                    .await?;

                println!("UV installed and Python 3.12.6 installed.");
            }
            Commands::Install {
                package,
                requirement,
                editable,
            } => {
                let mut path = std::env::current_dir()?;

                let venv = loop {
                    let venv = path.join(".venv");
                    if venv.exists() {
                        break Some(venv);
                    }

                    if !path.pop() {
                        break None;
                    }
                };

                if venv.is_none() {
                    let uv = uv::UV::new()?;
                    uv.create_venv(PythonVersion::from_str("3.12.6")?).await?;
                }

                if let Some(pkg) = package {
                    if editable {
                        let uv = uv::UV::new()?;
                        uv.pip_install(&["-e".to_string(), pkg]).await?;
                    } else {
                        let uv = uv::UV::new()?;
                        uv.pip_install(&[pkg]).await?;
                    }
                } else if let Some(req_file) = requirement {
                    let uv = uv::UV::new()?;
                    uv.pip_install(&["-r".to_string(), req_file]).await?;
                } else {
                    eyre::bail!("No package or requirement file specified");
                }
            }
            Commands::Dir => {
                let bin = get_python_bin().await?;
                println!("{}", bin.display());
            }
        }
    } else if !cli.python_args.is_empty() {
        let python_bin = get_python_bin().await?;

        let status = tokio::process::Command::new(python_bin)
            .args(cli.python_args)
            .status()
            .await?;

        if !status.success() {
            eyre::bail!("Python command failed");
        }
    } else {
        Cli::parse_from(&["epython", "--help"]);
    }

    Ok(())
}
