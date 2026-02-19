use anchor_size_core::{collect_all_structs, find_account_structs, size_of_type};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use chrono::Utc;

use clap::Parser;
use serde::Serialize;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about = "Anchor account size calculator")]
struct Args {
    path: String,
    #[arg(long)]
    all: bool,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out_dir: Option<String>,
}

#[derive(Serialize)]
struct AccountReport {
    name: String,
    exact_size: usize,
    recommended_size: usize,
    rent_sol: f64,
}



fn ask_number(question: &str) -> usize {
    print!("{}", question);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().expect("Enter a number")
}

fn size_of_dynamic_type(
    ty: &syn::Type,
    registry: &std::collections::HashMap<String, syn::ItemStruct>,
) -> usize {
    match ty {
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();
            let ident = segment.ident.to_string();

            match ident.as_str() {
                "String" => {
                    let max_len = ask_number("  Enter max string length: ");
                    4 + max_len
                }

                "Vec" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        let inner_ty = args.args.first().unwrap();
                        let inner_ty: syn::Type =
                            syn::parse_str(&quote::quote!(#inner_ty).to_string()).unwrap();

                        let max_items = ask_number("  Enter max items: ");

                        println!("  For Vec inner type:");
                        let inner_size = size_of_full_type_with_registry(&inner_ty, registry);

                        4 + max_items * inner_size
                    } else {
                        0
                    }
                }

                "Option" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        let inner_ty = args.args.first().unwrap();
                        let inner_ty: syn::Type =
                            syn::parse_str(&quote::quote!(#inner_ty).to_string()).unwrap();

                        println!("  For Option inner type:");
                        let inner_size = size_of_full_type_with_registry(&inner_ty, registry);

                        1 + inner_size
                    } else {
                        0
                    }
                }
                _ => 0,
            }
        }
        _ => 0,
    }
}

fn size_of_struct(
    struct_name: &str,
    registry: &HashMap<String, syn::ItemStruct>,
) -> usize {
    let strct = registry.get(struct_name).expect("Struct not found");

    let mut total = 0;

    for field in &strct.fields {
        let ty = &field.ty;
        total += size_of_full_type_with_registry(ty, registry);
    }

    total
}

fn size_of_full_type_with_registry(
    ty: &syn::Type,
    registry: &HashMap<String, syn::ItemStruct>,
) -> usize {
    if let Some(size) = size_of_type(ty) {
        return size;
    }

    // Проверяем: это struct?
    if let syn::Type::Path(type_path) = ty {
        let ident = type_path.path.segments.last().unwrap().ident.to_string();

        if registry.contains_key(&ident) {
            println!("    ↳ Calculating nested struct {}", ident);
            return size_of_struct(&ident, registry);
        }
    }

   size_of_dynamic_type(ty, registry)
}

fn rent_in_sol(data_len: usize) -> f64 {
    let rent = solana_program::rent::Rent::default();
    let lamports = rent.minimum_balance(data_len);
    lamports as f64 / solana_program::native_token::LAMPORTS_PER_SOL as f64
}

fn find_rust_files(path: &str) -> Vec<PathBuf> {
    let root = Path::new(path);

    let anchor_toml = root.join("Anchor.toml");

    let scan_path = if anchor_toml.exists() {
        println!("Detected Anchor workspace");
        root.join("programs")
    } else {
        root.to_path_buf()
    };

    WalkDir::new(scan_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_path_buf())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("rs"))
        .filter(|p| p.to_string_lossy().contains("/src/"))
        .collect()
}

fn analyze_file(
    file_path: &std::path::Path,
    json: bool,
) -> anyhow::Result<Vec<AccountReport>> {
    let code = std::fs::read_to_string(file_path)?;
    let registry = collect_all_structs(&code)?;
    let accounts = find_account_structs(&code)?;

    let mut reports = vec![];

    for strct in accounts {
        let mut total_size = 0usize;

        println!("\nAccount: {}\n", strct.ident);

        for field in &strct.fields {
            let name = field.ident.as_ref().unwrap().to_string();
            let ty = &field.ty;

            let size = size_of_full_type_with_registry(ty, &registry);
            println!("  {} → {} bytes", name, size);
            total_size += size;
        }

        let exact_size = total_size + 8;
        let mut recommended = ((exact_size as f64) * 1.15).ceil() as usize;
        recommended = (recommended + 7) / 8 * 8;
        let rent = rent_in_sol(recommended);

        if !json {
            println!("\nExact size: {} bytes", exact_size);
            println!("Recommended size: {} bytes", recommended);
            println!("Rent (mainnet): {:.6} SOL", rent);
        }

        reports.push(AccountReport {
            name: strct.ident.to_string(),
            exact_size,
            recommended_size: recommended,
            rent_sol: rent,
        });
    }

    Ok(reports)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut all_reports = vec![];

    if args.all {
        let files = find_rust_files(&args.path);
        for file in files {
            let mut reports = analyze_file(&file, args.json)?;
            all_reports.append(&mut reports);
        }
    } else {
        let path = std::path::Path::new(&args.path);
        let mut reports = analyze_file(path, args.json)?;
        all_reports.append(&mut reports);
    }

    if args.json {
        let json = serde_json::to_string_pretty(&all_reports)?;

        if let Some(dir) = args.out_dir {
            std::fs::create_dir_all(&dir)?;

            let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S");
            let path = format!("{}/anchor-size-{}.json", dir, timestamp);

            std::fs::write(&path, json)?;
            println!("Report saved to {}", path);
        } else {
            println!("{}", json);
        }
    }

    Ok(())
}
