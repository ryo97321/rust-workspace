use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use sha1::{Digest, Sha1};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name="mygit", version, about="Tiny git-like (baby steps)")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Init,
    HashObject {
        path: PathBuf,
    },
    CatFile {
        hash: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Init => init_repo(".")?,
        Cmd::HashObject { path } => {
            let hash = hash_object(&path)?;
            println!("{hash}");
        }
        Cmd::CatFile { hash } => {
            cat_file(&hash)?;
        }
    }
    Ok(())
}

fn mygit_dir(repo_root: impl AsRef<Path>) -> PathBuf {
    repo_root.as_ref().join(".mygit")
}

fn objects_dir(repo_root: impl AsRef<Path>) -> PathBuf {
    mygit_dir(repo_root).join("objects")
}

fn init_repo(repo_root: impl AsRef<Path>) -> Result<()> {
    let root = repo_root.as_ref();
    fs::create_dir_all(objects_dir(root))?;
    fs::create_dir_all(mygit_dir(root).join("refs"))?;
    let head = mygit_dir(root).join("refs").join("HEAD");
    if !head.exists() {
        fs::write(&head, b"")?;
    }
    println!("Initialized empty mygit repository in {}", mygit_dir(root).display());
    Ok(())
}

fn ensure_repo() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    if mygit_dir(&cwd).is_dir() {
        Ok(cwd)
    } else {
        Err(anyhow!("Not a mygit repository: run `mygit init first"))
    }
}

fn sha1_hex(bytes: &[u8]) -> String {
    let mut h = Sha1::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

fn hash_object(path: &Path) -> Result<String> {
    let root = ensure_repo()?;
    let data = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
     // obejct bytes = "blob\0" + payload
     let mut obj = Vec::with_capacity(5 + data.len());
     obj.extend_from_slice(b"blob\0");
     obj.extend_from_slice(&data);

     let hash = sha1_hex(&obj);
     let obj_path = objects_dir(&root).join(&hash);
     if !obj_path.exists() {
         fs::write(&obj_path, &obj)
             .with_context(|| format!("failed to write {}", obj_path.display()))?;
     }
     Ok(hash)
}

fn cat_file(hash: &str) -> Result<()> {
    let root = ensure_repo()?;
    let obj_path = objects_dir(&root).join(&hash);
    let obj = fs::read(&obj_path).with_context(|| format!("failed to read {}", obj_path.display()))?;

    let nul = obj.iter().position(|b| *b == 0).ok_or_else(|| anyhow!("invalid object"))?;
    let kind = std::str::from_utf8(&obj[..nul]).context("invalid utf8 in kind")?;

    let payload = &obj[nul + 1..];
    match kind {
        "blob" => {
            use std::io::Write;
            std::io::stdout().write_all(payload)?;
        }
        other => return Err(anyhow!("unsupported kind: {other}")),
    }
    Ok(())
}

