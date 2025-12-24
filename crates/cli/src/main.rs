use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Parser, Debug)]
#[command(
    name = "hayabusa",
    version,
    about = "CLI for the Hayabusa Solana runtime framework"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new Hayabusa workspace
    New {
        /// Workspace name (also used as the program crate name)
        name: String,

        /// Directory to create the workspace in (default: ./<name>)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Overwrite existing directory (DANGEROUS)
        #[arg(long, default_value_t = false)]
        force: bool,
    },

    /// Build the workspace program (aliases cargo build-sbf) and print .so size
    Build {
        /// Program name (defaults to workspace dir name)
        #[arg(long)]
        program: Option<String>,

        /// Path to workspace root (default: current directory)
        #[arg(long)]
        workspace: Option<PathBuf>,
    },

    /// Run tests (cargo test)
    Test {
        /// Path to workspace root (default: current directory)
        #[arg(long)]
        workspace: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::New { name, path, force } => cmd_new(&name, path.as_deref(), force),
        Commands::Build { program, workspace } => {
            cmd_build(program.as_deref(), workspace.as_deref())
        }
        Commands::Test { workspace } => cmd_test(workspace.as_deref()),
    }
}

fn cmd_new(name: &str, path: Option<&Path>, force: bool) -> Result<()> {
    validate_crate_name(name)?;

    let root = path
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from(name));

    if root.exists() {
        if !force {
            bail!(
                "Destination already exists: {} (use --force to overwrite)",
                root.display()
            );
        }
        // Very blunt overwrite: remove whole directory
        fs::remove_dir_all(&root)
            .with_context(|| format!("Failed to remove {}", root.display()))?;
    }

    // Create workspace structure
    let programs_dir = root.join("programs");
    let tests_dir = root.join("tests");
    let program_crate_dir = programs_dir.join(name);
    let test_crate_name = format!("{name}-tests");
    let test_crate_dir = tests_dir.join(&test_crate_name);

    fs::create_dir_all(program_crate_dir.join("src"))
        .with_context(|| "Failed to create program crate directories")?;
    fs::create_dir_all(test_crate_dir.join("src"))
        .with_context(|| "Failed to create tests crate directories")?;

    // Root workspace Cargo.toml
    write_file(
        &root.join("Cargo.toml"),
        &workspace_cargo_toml(name, &test_crate_name),
    )?;

    // Optional root .gitignore (handy)
    write_file(&root.join(".gitignore"), GITIGNORE)?;

    // Program crate Cargo.toml + lib.rs
    write_file(
        &program_crate_dir.join("Cargo.toml"),
        &program_cargo_toml(name),
    )?;
    write_file(&program_crate_dir.join("src/lib.rs"), PROGRAM_LIB_RS)?;

    // Tests crate Cargo.toml + lib.rs
    write_file(
        &test_crate_dir.join("Cargo.toml"),
        &tests_cargo_toml(&test_crate_name),
    )?;
    write_file(&test_crate_dir.join("src/lib.rs"), TESTS_LIB_RS)?;

    println!("Created workspace at {}", root.display());
    println!("  Program: programs/{}/", name);
    println!("  Tests:   tests/{}/", test_crate_name);

    Ok(())
}

fn cmd_build(program: Option<&str>, workspace: Option<&Path>) -> Result<()> {
    let ws = workspace.unwrap_or_else(|| Path::new("."));
    ensure_workspace_root(ws)?;

    let program_name = match program {
        Some(p) => p.to_string(),
        None => infer_workspace_dir_name(ws)?,
    }
    .replace("-", "_");

    // cargo build-sbf
    let status = Command::new("cargo")
        .arg("build-sbf")
        .current_dir(ws)
        .status()
        .context("Failed to spawn cargo build-sbf")?;

    if !status.success() {
        bail!("cargo build-sbf failed");
    }

    // Print binary size
    let so_path = ws
        .join("target")
        .join("deploy")
        .join(format!("{program_name}.so"));

    let md = fs::metadata(&so_path)
        .with_context(|| format!("Could not stat output .so: {}", so_path.display()))?;

    let size = md.len();
    println!("Built {}", so_path.display());
    println!("Binary size: {} bytes ({})", size, human_bytes(size));

    Ok(())
}

fn cmd_test(workspace: Option<&Path>) -> Result<()> {
    let ws = workspace.unwrap_or_else(|| Path::new("."));
    ensure_workspace_root(ws)?;

    let status = Command::new("cargo")
        .arg("test")
        .current_dir(ws)
        .status()
        .context("Failed to spawn cargo test")?;

    if !status.success() {
        bail!("cargo test failed");
    }

    Ok(())
}

fn ensure_workspace_root(path: &Path) -> Result<()> {
    let cargo_toml = path.join("Cargo.toml");
    if !cargo_toml.exists() {
        bail!("No Cargo.toml found at workspace root: {}", path.display());
    }
    Ok(())
}

fn infer_workspace_dir_name(ws: &Path) -> Result<String> {
    ws.canonicalize()
        .ok()
        .and_then(|p| p.file_name().map(|s| s.to_string_lossy().to_string()))
        .ok_or_else(|| anyhow!("Could not infer workspace dir name; pass --program"))
}

fn validate_crate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("Name cannot be empty");
    }
    let ok = name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_');
    if !ok {
        bail!("Invalid name '{name}'. Use lowercase letters, digits, '-', '_'.");
    }
    Ok(())
}

fn write_file(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    let mut f = fs::File::create(path)
        .with_context(|| format!("Failed to create file {}", path.display()))?;
    f.write_all(contents.as_bytes())
        .with_context(|| format!("Failed to write file {}", path.display()))?;
    Ok(())
}

fn human_bytes(n: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut v = n as f64;
    let mut i = 0usize;
    while v >= 1024.0 && i + 1 < UNITS.len() {
        v /= 1024.0;
        i += 1;
    }
    if i == 0 {
        format!("{n} {}", UNITS[i])
    } else {
        format!("{:.2} {}", v, UNITS[i])
    }
}

fn workspace_cargo_toml(program_name: &str, test_crate_name: &str) -> String {
    format!(
        r#"[workspace]
resolver = "2"
members = [
  "programs/{program_name}",
  "tests/{test_crate_name}",
]

[workspace.package]
version = "0.1.0"
name = "{program_name}"
"#
    )
}

fn program_cargo_toml(program_name: &str) -> String {
    format!(
        r#"[package]
name = "{program_name}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
hayabusa = "0.1.0"
"#
    )
}

fn tests_cargo_toml(test_crate_name: &str) -> String {
    format!(
        r#"[package]
name = "{test_crate_name}"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
hayabusa = "0.1.0
litesvm = "0.6.1"
solana-sdk = "2.2.1"
"#
    )
}

const PROGRAM_LIB_RS: &str = r#"#![no_std]
#![allow(dead_code, unexpected_cfgs)]

use bytemuck::{Pod, Zeroable};
use hayabusa::prelude::*;

declare_id!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint {
    use super::*;

    program_entrypoint!(program_entrypoint);
    no_allocator!();
    nostd_panic_handler!();

    pub fn program_entrypoint(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> Result<()> {
        dispatch!(
            program_id,
            instruction_data,
            accounts,
            UpdateCounterInstruction => update_counter(amount),
            InitializeCounterInstruction => initialize_counter(),
        );
    }
}

#[instruction] // generates UpdateCounterInstruction { amount: u64 } + Discriminator
fn update_counter<'a>(ctx: Ctx<'a, UpdateCounter<'a>>, amount: u64) -> Result<()> {
    let mut counter = ctx.counter.try_deserialize_mut()?;

    counter.count += amount;

    Ok(())
}

pub struct UpdateCounter<'a> {
    pub user: Signer<'a>,
    pub counter: Mut<'a, ZcAccount<'a, CounterAccount>>,
}

// Intentionally kept manual, you get to see what the FromAccountInfos proc macro is doing
impl<'a> FromAccountInfos<'a> for UpdateCounter<'a> {
    #[inline(always)]
    fn try_from_account_infos(account_infos: &mut AccountIter<'a>) -> Result<Self> {
        let user = Signer::try_from_account_info(account_infos.next()?)?;
        let counter = Mut::try_from_account_info(account_infos.next()?)?;

        Ok(UpdateCounter {
            user,
            counter,
        })
    }
}

#[instruction]
fn initialize_counter<'a>(ctx: Ctx<'a, InitializeCounter<'a>>) -> Result<()> {
    // account is zeroed on init
    let _ = ctx.counter.try_initialize(
        InitAccounts::new(
            &crate::ID,
            ctx.user.to_account_info(),
            ctx.system_program.to_account_info(),
        ),
        None,
    )?;

    Ok(())
}

#[derive(FromAccountInfos)]
pub struct InitializeCounter<'a> {
    pub user: Mut<'a, Signer<'a>>,
    pub counter: Mut<'a, ZcAccount<'a, CounterAccount>>,
    pub system_program: Program<'a, System>,
}

#[account]
#[derive(OwnerProgram)]
pub struct CounterAccount {
    pub count: u64,
}
"#;

const TESTS_LIB_RS: &str = r#"#![allow(unused)]

use hayabusa::prelude::Discriminator;
use litesvm::LiteSVM;
use solana_sdk::{
    account::Account, instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::Keypair, signer::Signer, system_program, transaction::Transaction, pubkey,
};
use spl_token::{state::{Account as TokenAccount, Mint}, solana_program::program_pack::Pack};

#[test]
fn integration() {
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../target/deploy/counter_program.so");

    let program_id = pubkey!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

    svm.add_program(program_id, program_bytes);

    let keypair = Keypair::new();
    let user = keypair.pubkey();

    svm.airdrop(&user, 1_000_000_000_000).unwrap();

    let counter_account_data = pack_zc_account(CounterAccount { counter: 0 });
    let counter_account_pk = Pubkey::new_unique();
    let counter_account = Account {
        lamports: svm.minimum_balance_for_rent_exemption(counter_account_data.len()),
        data: counter_account_data,
        owner: program_id,
        executable: false,
        rent_epoch: 0,
    };

    svm.set_account(counter_account_pk, counter_account).unwrap();

    let ix_data = {
        const UPDATE_COUNTER_DISCRIMINATOR: [u8; 8] = [231, 120, 160, 18, 72, 164, 104, 62];
        let mut data = UPDATE_COUNTER_DISCRIMINATOR.to_vec();
        data.extend_from_slice(&1u64.to_le_bytes());
        data
    };

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(user, true),
            AccountMeta::new(counter_account_pk, false),
        ],
        data: ix_data,
    };

    let tx = Transaction::new_signed_with_payer(&[ix], Some(&user), &[&keypair], svm.latest_blockhash());

    let res = svm.send_transaction(tx);

    println!("Transaction result: {:#?}", res);

}

#[test]
fn integration2() {
    let mut svm = LiteSVM::new();

    let program_bytes = include_bytes!("../../target/deploy/counter_program.so");

    let program_id = pubkey!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

    svm.add_program(program_id, program_bytes);

    let keypair = Keypair::new();
    let user = keypair.pubkey();

    let target_keypair = Keypair::new();
    let target = target_keypair.pubkey();

    svm.airdrop(&user, 1_000_000_000_000).unwrap();

    let ix_data = {
        const INITIALIZE_COUNTER_DISCRIMINATOR: [u8; 8] = [184, 155, 169, 181, 122, 145, 244, 45];
        let data = INITIALIZE_COUNTER_DISCRIMINATOR.to_vec();
        data
    };

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(target, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: ix_data,
    };

    let tx = Transaction::new_signed_with_payer(&[ix], Some(&user), &[&keypair, &target_keypair], svm.latest_blockhash());

    let res = svm.send_transaction(tx);

    println!("Transaction result: {:#?}", res);

}

fn pack_zc_account<T: bytemuck::NoUninit + Discriminator>(account: T) -> Vec<u8> {
    let mut data = T::DISCRIMINATOR.to_vec();
    data.extend_from_slice(bytemuck::bytes_of(&account));
    data
}

#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Discriminator)]
#[repr(C)]
struct CounterAccount {
    counter: u64,
}
"#;

const GITIGNORE: &str = r#"/target
**/target
.DS_Store
"#;
