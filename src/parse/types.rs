use mago_names::ResolvedNames;
use mago_syntax::cst::{
    Access, ArrayElement, ClassLikeConstantSelector, Expression, Hint, Literal,
    ModifierSequenceExt, PartialArgument, Sequence, UnaryPrefixOperator,
};

use crate::ir::AttributeIr;
use crate::tokens::NativeType;

use super::names::{bytes_to_string, owned_name, resolve_identifier};

pub(crate) fn convert_hint(
    hint: &Hint<'_>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
    current_fqcn: &str,
) -> NativeType {
    match hint {
        Hint::Integer(_) => NativeType::Int,
        Hint::Float(_) => NativeType::Float,
        Hint::String(_) => NativeType::String,
        Hint::Bool(_) => NativeType::Bool,
        Hint::Array(_) => NativeType::Array,
        Hint::Mixed(_) => NativeType::Mixed,
        Hint::Null(_) => NativeType::Null,
        Hint::True(_) | Hint::False(_) => NativeType::Bool,
        Hint::Nullable(inner) => {
            let converted = convert_hint(inner.hint, resolved, namespace, current_fqcn);
            match converted {
                NativeType::Mixed | NativeType::Nullable(_) => converted,
                other => NativeType::Nullable(Box::new(other)),
            }
        }
        Hint::Parenthesized(inner) => convert_hint(inner.hint, resolved, namespace, current_fqcn),
        Hint::Union(_) => flatten_union(hint, resolved, namespace, current_fqcn),
        Hint::Identifier(ident) => {
            NativeType::Class(resolve_identifier(resolved, ident, namespace))
        }
        Hint::Self_(_) => NativeType::Class(current_fqcn.to_string()),
        Hint::Static(_) => NativeType::Class("static".to_string()),
        Hint::Parent(_) => NativeType::Class("parent".to_string()),
        Hint::Object(_) => NativeType::Class("object".to_string()),
        Hint::Iterable(_) => NativeType::Class("iterable".to_string()),
        Hint::Callable(_) => NativeType::Class("callable".to_string()),
        Hint::Void(_) => NativeType::Class("void".to_string()),
        Hint::Never(_) => NativeType::Class("never".to_string()),
        Hint::Intersection(inner) => convert_hint(inner.left, resolved, namespace, current_fqcn),
    }
}

fn flatten_union(
    hint: &Hint<'_>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
    current_fqcn: &str,
) -> NativeType {
    let mut members = Vec::new();
    let mut has_null = false;
    collect_union_members(
        hint,
        resolved,
        namespace,
        current_fqcn,
        &mut members,
        &mut has_null,
    );
    finish_union(members, has_null)
}

fn collect_union_members(
    hint: &Hint<'_>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
    current_fqcn: &str,
    members: &mut Vec<NativeType>,
    has_null: &mut bool,
) {
    match hint {
        Hint::Union(union) => {
            collect_union_members(
                union.left,
                resolved,
                namespace,
                current_fqcn,
                members,
                has_null,
            );
            collect_union_members(
                union.right,
                resolved,
                namespace,
                current_fqcn,
                members,
                has_null,
            );
        }
        Hint::Parenthesized(inner) => {
            collect_union_members(
                inner.hint,
                resolved,
                namespace,
                current_fqcn,
                members,
                has_null,
            );
        }
        Hint::Null(_) => *has_null = true,
        Hint::Nullable(inner) => {
            *has_null = true;
            collect_union_members(
                inner.hint,
                resolved,
                namespace,
                current_fqcn,
                members,
                has_null,
            );
        }
        Hint::Mixed(_) => members.push(NativeType::Mixed),
        other => match convert_hint(other, resolved, namespace, current_fqcn) {
            NativeType::Null => *has_null = true,
            NativeType::Nullable(inner) => {
                *has_null = true;
                members.push(*inner);
            }
            NativeType::Union(inner) => members.extend(inner),
            ty => members.push(ty),
        },
    }
}

fn finish_union(mut types: Vec<NativeType>, has_null: bool) -> NativeType {
    types.sort_by_key(type_sort_key);
    match types.len() {
        0 => NativeType::Mixed,
        1 => {
            let inner = types.pop().expect("len == 1");
            if has_null {
                NativeType::Nullable(Box::new(inner))
            } else {
                inner
            }
        }
        _ => {
            let union = NativeType::Union(types);
            if has_null {
                NativeType::Nullable(Box::new(union))
            } else {
                union
            }
        }
    }
}

fn type_sort_key(ty: &NativeType) -> String {
    match ty {
        NativeType::Int => "1".to_string(),
        NativeType::Float => "2".to_string(),
        NativeType::String => "3".to_string(),
        NativeType::Bool => "4".to_string(),
        NativeType::Class(name) => format!("6-{name}"),
        NativeType::Nullable(inner) => format!("8-{}", type_sort_key(inner)),
        NativeType::Union(inner) => format!(
            "7-{}",
            inner
                .iter()
                .map(type_sort_key)
                .collect::<Vec<_>>()
                .join("|")
        ),
        other => format!("9-{other:?}"),
    }
}

pub(crate) fn convert_attributes(
    lists: &Sequence<'_, mago_syntax::cst::AttributeList<'_>>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
) -> Vec<AttributeIr> {
    let mut attributes = Vec::new();
    for list in lists {
        for attribute in list.attributes.iter() {
            let name = resolve_identifier(resolved, &attribute.name, namespace);
            let arguments = attribute
                .argument_list
                .as_ref()
                .map(|args| {
                    args.arguments
                        .iter()
                        .filter_map(|arg| partial_argument_json(arg, resolved, namespace))
                        .collect()
                })
                .unwrap_or_default();
            attributes.push(AttributeIr { name, arguments });
        }
    }
    attributes
}

fn partial_argument_json(
    arg: &PartialArgument<'_>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
) -> Option<serde_json::Value> {
    match arg {
        PartialArgument::Positional(positional) => {
            Some(expr_to_json(positional.value, resolved, namespace))
        }
        PartialArgument::Named(named) => Some(expr_to_json(named.value, resolved, namespace)),
        _ => None,
    }
}

pub(crate) fn expr_to_json(
    expr: &Expression<'_>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
) -> serde_json::Value {
    match expr {
        Expression::Literal(literal) => literal_to_json(literal),
        Expression::Array(array) => array_elements_to_json(&array.elements, resolved, namespace),
        Expression::LegacyArray(array) => {
            array_elements_to_json(&array.elements, resolved, namespace)
        }
        Expression::UnaryPrefix(unary) => match unary.operator {
            UnaryPrefixOperator::Negation(_) => {
                match expr_to_json(unary.operand, resolved, namespace) {
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            serde_json::json!(-i)
                        } else if let Some(f) = n.as_f64() {
                            serde_json::json!(-f)
                        } else {
                            serde_json::Value::Null
                        }
                    }
                    other => other,
                }
            }
            UnaryPrefixOperator::Plus(_) => expr_to_json(unary.operand, resolved, namespace),
            _ => serde_json::Value::Null,
        },
        Expression::Access(Access::ClassConstant(access)) => {
            if is_class_keyword(&access.constant) {
                class_name_from_expr(access.class, resolved, namespace)
                    .map(serde_json::Value::String)
                    .unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        Expression::Identifier(ident) => {
            serde_json::Value::String(resolve_identifier(resolved, ident, namespace))
        }
        Expression::Parenthesized(inner) => expr_to_json(inner.expression, resolved, namespace),
        _ => serde_json::Value::Null,
    }
}

fn is_class_keyword(selector: &ClassLikeConstantSelector<'_>) -> bool {
    match selector {
        ClassLikeConstantSelector::Identifier(ident) => ident.value.eq_ignore_ascii_case(b"class"),
        _ => false,
    }
}

fn class_name_from_expr(
    expr: &Expression<'_>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
) -> Option<String> {
    match expr {
        Expression::Identifier(ident) => Some(resolve_identifier(resolved, ident, namespace)),
        Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_) => None,
        _ => None,
    }
}

fn literal_to_json(literal: &Literal<'_>) -> serde_json::Value {
    match literal {
        Literal::String(string) => {
            let bytes = string.value.unwrap_or(string.raw);
            serde_json::Value::String(decode_php_string(bytes, string.raw))
        }
        Literal::Integer(integer) => {
            integer
                .value
                .map(|n| serde_json::json!(n))
                .unwrap_or_else(|| {
                    owned_name(integer.raw)
                        .and_then(|s| s.parse::<i64>().ok())
                        .map(|n| serde_json::json!(n))
                        .unwrap_or(serde_json::Value::Null)
                })
        }
        Literal::Float(float) => serde_json::json!(float.value.into_inner()),
        Literal::True(_) => serde_json::Value::Bool(true),
        Literal::False(_) => serde_json::Value::Bool(false),
        Literal::Null(_) => serde_json::Value::Null,
    }
}

fn decode_php_string(value: &[u8], raw: &[u8]) -> String {
    if let Some(s) = owned_name(value) {
        return s;
    }
    let raw_s = String::from_utf8_lossy(raw);
    let trimmed = raw_s.trim();
    if (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        || (trimmed.starts_with('"') && trimmed.ends_with('"'))
    {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

fn array_elements_to_json(
    elements: &mago_syntax::cst::TokenSeparatedSequence<'_, ArrayElement<'_>>,
    resolved: &ResolvedNames<'_>,
    namespace: Option<&str>,
) -> serde_json::Value {
    let mut values = Vec::new();
    let mut object = serde_json::Map::new();
    let mut is_object = false;
    for element in elements.iter() {
        match element {
            ArrayElement::Value(value) => {
                values.push(expr_to_json(value.value, resolved, namespace));
            }
            ArrayElement::KeyValue(kv) => {
                is_object = true;
                let key = match expr_to_json(kv.key, resolved, namespace) {
                    serde_json::Value::String(s) => s,
                    serde_json::Value::Number(n) => n.to_string(),
                    other => other.to_string(),
                };
                object.insert(key, expr_to_json(kv.value, resolved, namespace));
            }
            _ => {}
        }
    }
    if is_object {
        serde_json::Value::Object(object)
    } else {
        serde_json::Value::Array(values)
    }
}

pub(crate) fn variable_name(name: &[u8]) -> String {
    bytes_to_string(name).trim_start_matches('$').to_string()
}

pub(crate) fn is_public_non_static(
    modifiers: &Sequence<'_, mago_syntax::cst::Modifier<'_>>,
) -> bool {
    !modifiers.contains_static()
        && (modifiers.contains_public() || !modifiers.contains_visibility())
}
