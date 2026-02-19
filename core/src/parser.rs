use anyhow::Result;
use syn::{Item, ItemStruct, Fields, Type};
use std::collections::HashMap;

pub fn find_account_structs(code: &str) -> Result<Vec<ItemStruct>> {
    let file = syn::parse_file(code)?;
    let mut accounts = vec![];

    for item in file.items {
        if let Item::Struct(item_struct) = item {
            if has_account_attr(&item_struct) {
                accounts.push(item_struct);
            }
        }
    }

    Ok(accounts)
}

fn has_account_attr(item_struct: &ItemStruct) -> bool {
    item_struct.attrs.iter().any(|attr| {
        attr.path().is_ident("account")
    })
}

pub fn extract_fields(strct: &ItemStruct) -> Vec<(String, String)> {
    let mut result = vec![];

    match &strct.fields {
        Fields::Named(fields_named) => {
            for field in &fields_named.named {
                let name = field.ident.as_ref().unwrap().to_string();
                let ty = type_to_string(&field.ty);
                result.push((name, ty));
            }
        }
        _ => {}
    }

    result
}

fn type_to_string(ty: &Type) -> String {
    quote::quote!(#ty).to_string()
}

pub fn collect_all_structs(code: &str) -> Result<HashMap<String, ItemStruct>> {
    let file = syn::parse_file(code)?;
    let mut map = HashMap::new();

    for item in file.items {
        if let Item::Struct(item_struct) = item {
            let name = item_struct.ident.to_string();
            map.insert(name, item_struct);
        }
    }

    Ok(map)
}

