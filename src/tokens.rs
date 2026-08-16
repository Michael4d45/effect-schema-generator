use std::collections::BTreeMap;

use crate::ir::AttributeIr;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassToken {
    pub namespace: String,
    pub fqcn: String,
    pub name: String,
    pub uses: BTreeMap<String, String>,
    pub public_properties: Vec<PublicPropertyToken>,
    pub attributes: Vec<AttributeIr>,
}

impl ClassToken {
    pub fn new(namespace: impl Into<String>, fqcn: impl Into<String>) -> Self {
        let fqcn = fqcn.into();
        let name = fqcn.rsplit('\\').next().unwrap_or(&fqcn).to_string();
        Self {
            namespace: namespace.into(),
            fqcn,
            name,
            uses: BTreeMap::new(),
            public_properties: Vec::new(),
            attributes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PublicPropertyToken {
    pub name: String,
    pub native_type: NativeType,
    pub phpdoc_type: Option<String>,
    pub attributes: Vec<AttributeIr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NativeType {
    String,
    Int,
    Float,
    Bool,
    Mixed,
    Array,
    Null,
    Class(String),
    Union(Vec<NativeType>),
    Nullable(Box<NativeType>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumToken {
    pub fqcn: String,
    pub namespace: String,
    pub name: String,
    pub backed_type: String,
    pub cases: Vec<crate::ir::EnumCaseIr>,
}

impl EnumToken {
    pub fn new(namespace: impl Into<String>, fqcn: impl Into<String>) -> Self {
        let fqcn = fqcn.into();
        let name = fqcn.rsplit('\\').next().unwrap_or(&fqcn).to_string();
        Self {
            namespace: namespace.into(),
            fqcn,
            name,
            backed_type: String::new(),
            cases: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Class(ClassToken),
    Enum(EnumToken),
}
