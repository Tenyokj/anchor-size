use syn::{Type, TypePath};

pub fn size_of_type(ty: &Type) -> Option<usize> {
    match ty {
        Type::Path(type_path) => size_of_path(type_path),
        _ => None,
    }
}

fn size_of_path(type_path: &TypePath) -> Option<usize> {
    let ident = type_path.path.segments.last()?.ident.to_string();

    match ident.as_str() {
        "u8" | "i8" | "bool" => Some(1),
        "u16" | "i16" => Some(2),
        "u32" | "i32" => Some(4),
        "u64" | "i64" => Some(8),
        "Pubkey" => Some(32),

        // динамические типы — пока не считаем
        "String" | "Vec" | "Option" => None,

        _ => None,
    }
}
