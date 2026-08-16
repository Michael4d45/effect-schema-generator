use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct RootIr {
    pub namespaces: BTreeMap<String, NamespaceIr>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NamespaceIr {
    pub namespace: String,
    pub name: String,
    pub uses: BTreeMap<String, String>,
    pub schemas: Vec<SchemaIr>,
    pub enums: Vec<EnumIr>,
}

impl NamespaceIr {
    pub fn new(namespace: impl Into<String>) -> Self {
        let namespace = namespace.into();
        let name = namespace
            .rsplit('\\')
            .next()
            .unwrap_or(&namespace)
            .to_string();
        Self {
            namespace,
            name,
            uses: BTreeMap::new(),
            schemas: Vec::new(),
            enums: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SchemaIr {
    pub name: String,
    pub uses: BTreeMap<String, String>,
    pub properties: Vec<PropertyIr>,
    pub class_attributes: Vec<AttributeIr>,
}

impl SchemaIr {
    pub fn new(name: impl Into<String>, properties: Vec<PropertyIr>) -> Self {
        Self {
            name: name.into(),
            uses: BTreeMap::new(),
            properties,
            class_attributes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EnumIr {
    pub name: String,
    pub cases: Vec<EnumCaseIr>,
    pub backed_type: String,
}

impl EnumIr {
    pub fn new(
        name: impl Into<String>,
        cases: Vec<EnumCaseIr>,
        backed_type: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            cases,
            backed_type: backed_type.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EnumCaseIr {
    pub name: String,
    pub value: Option<serde_json::Value>,
}

impl EnumCaseIr {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Some(serde_json::Value::String(value.into())),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PropertyIr {
    pub name: String,
    pub ty: TypeIr,
    pub nullable: bool,
    pub optional: bool,
    pub attributes: Vec<AttributeIr>,
    pub hidden: bool,
}

impl PropertyIr {
    pub fn new(name: impl Into<String>, ty: TypeIr) -> Self {
        Self {
            name: name.into(),
            ty,
            nullable: false,
            optional: false,
            attributes: Vec::new(),
            hidden: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AttributeIr {
    pub name: String,
    pub arguments: Vec<serde_json::Value>,
}

impl AttributeIr {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arguments: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum TypeIr {
    String,
    Int,
    Float,
    Bool,
    Unknown,
    Nullable {
        inner: Box<TypeIr>,
    },
    Union {
        types: Vec<TypeIr>,
    },
    Array {
        item: Option<Box<TypeIr>>,
    },
    Record {
        key: Box<TypeIr>,
        value: Box<TypeIr>,
    },
    Struct {
        properties: Vec<PropertyIr>,
    },
    Class {
        fqcn: String,
        namespace: String,
        alias: String,
        is_enum: bool,
        type_parameters: Vec<TypeIr>,
    },
}

impl TypeIr {
    pub fn class_ref(fqcn: impl Into<String>) -> Self {
        let fqcn = fqcn.into().trim_start_matches('\\').to_string();
        let mut parts: Vec<&str> = fqcn.split('\\').collect();
        let alias = parts.pop().unwrap_or("").to_string();
        let namespace = parts.join("\\");
        Self::Class {
            fqcn,
            namespace,
            alias,
            is_enum: false,
            type_parameters: Vec::new(),
        }
    }

    pub fn class_ref_params(fqcn: impl Into<String>, type_parameters: Vec<TypeIr>) -> Self {
        match Self::class_ref(fqcn) {
            Self::Class {
                fqcn,
                namespace,
                alias,
                is_enum,
                ..
            } => Self::Class {
                fqcn,
                namespace,
                alias,
                is_enum,
                type_parameters,
            },
            other => other,
        }
    }

    pub fn enum_ref(fqcn: impl Into<String>) -> Self {
        match Self::class_ref(fqcn) {
            Self::Class {
                fqcn,
                namespace,
                alias,
                type_parameters,
                ..
            } => Self::Class {
                fqcn,
                namespace,
                alias,
                is_enum: true,
                type_parameters,
            },
            other => other,
        }
    }

    pub fn is_nullable(&self) -> bool {
        matches!(self, Self::Nullable { .. })
    }

    /// Mark class-references whose FQCN appears in `enum_fqcns` as enums.
    pub fn mark_enums(&mut self, enum_fqcns: &std::collections::HashSet<String>) {
        match self {
            Self::Class {
                fqcn,
                is_enum,
                type_parameters,
                ..
            } => {
                let normalized = fqcn.trim_start_matches('\\');
                if enum_fqcns
                    .iter()
                    .any(|candidate| candidate.trim_start_matches('\\') == normalized)
                {
                    *is_enum = true;
                }
                for param in type_parameters {
                    param.mark_enums(enum_fqcns);
                }
            }
            Self::Array { item } => {
                if let Some(item) = item {
                    item.mark_enums(enum_fqcns);
                }
            }
            Self::Nullable { inner } => inner.mark_enums(enum_fqcns),
            Self::Record { key, value } => {
                key.mark_enums(enum_fqcns);
                value.mark_enums(enum_fqcns);
            }
            Self::Union { types } => {
                for ty in types {
                    ty.mark_enums(enum_fqcns);
                }
            }
            Self::Struct { properties } => {
                for property in properties {
                    property.ty.mark_enums(enum_fqcns);
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriterContext {
    Interface,
    EncodedInterface,
    Schema,
    Enum,
}
