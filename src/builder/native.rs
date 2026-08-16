use crate::ir::TypeIr;
use crate::tokens::NativeType;

/// Map a native PHP type hint to TypeIR.
pub fn build_native_type(ty: &NativeType) -> TypeIr {
    match ty {
        NativeType::String => TypeIr::String,
        NativeType::Int => TypeIr::Int,
        NativeType::Float => TypeIr::Float,
        NativeType::Bool => TypeIr::Bool,
        NativeType::Mixed | NativeType::Null => TypeIr::Unknown,
        NativeType::Array => TypeIr::Array {
            item: Some(Box::new(TypeIr::Unknown)),
        },
        NativeType::Class(fqcn) => TypeIr::class_ref(fqcn),
        NativeType::Nullable(inner) => wrap_nullable(build_native_type(inner)),
        NativeType::Union(types) => {
            let mut has_null = false;
            let mut mapped = Vec::new();
            for member in types {
                if matches!(member, NativeType::Null) {
                    has_null = true;
                    continue;
                }
                mapped.push(build_native_type(member));
            }
            let inner = match mapped.len() {
                0 => TypeIr::Unknown,
                1 => mapped.remove(0),
                _ => TypeIr::Union { types: mapped },
            };
            if has_null {
                wrap_nullable(inner)
            } else {
                inner
            }
        }
    }
}

pub fn native_allows_null(ty: &NativeType) -> bool {
    match ty {
        NativeType::Null | NativeType::Nullable(_) => true,
        NativeType::Union(types) => types.iter().any(native_allows_null),
        _ => false,
    }
}

fn wrap_nullable(ty: TypeIr) -> TypeIr {
    if ty.is_nullable() {
        ty
    } else {
        TypeIr::Nullable {
            inner: Box::new(ty),
        }
    }
}
