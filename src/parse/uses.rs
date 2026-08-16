use std::collections::BTreeMap;

use mago_syntax::cst::{
    Identifier, MaybeTypedUseItem, MixedUseItemList, Statement, Use, UseItem, UseItems,
};

use super::names::{bytes_to_string, join_fqcn, owned_name, short_name};

pub(crate) fn collect_use_map(
    statements: &mago_syntax::cst::Sequence<'_, Statement<'_>>,
) -> BTreeMap<String, String> {
    let mut uses = BTreeMap::new();
    walk_uses(statements, &mut uses);
    uses
}

fn walk_uses(
    statements: &mago_syntax::cst::Sequence<'_, Statement<'_>>,
    uses: &mut BTreeMap<String, String>,
) {
    for statement in statements {
        match statement {
            Statement::Namespace(namespace) => walk_uses(namespace.statements(), uses),
            Statement::Block(block) => walk_uses(&block.statements, uses),
            Statement::Use(use_stmt) => collect_use_statement(use_stmt, uses),
            _ => {}
        }
    }
}

fn collect_use_statement(use_stmt: &Use<'_>, uses: &mut BTreeMap<String, String>) {
    match &use_stmt.items {
        UseItems::Sequence(sequence) => {
            for item in sequence.items.iter() {
                insert_use_item(item, None, uses);
            }
        }
        UseItems::MixedList(list) => collect_mixed_list(list, uses),
        UseItems::TypedSequence(_) | UseItems::TypedList(_) => {}
    }
}

fn collect_mixed_list(list: &MixedUseItemList<'_>, uses: &mut BTreeMap<String, String>) {
    let prefix = ident_name(&list.namespace);
    for maybe in list.items.iter() {
        let MaybeTypedUseItem { r#type, item } = maybe;
        if r#type.is_some() {
            continue;
        }
        insert_use_item(item, Some(&prefix), uses);
    }
}

fn insert_use_item(item: &UseItem<'_>, prefix: Option<&str>, uses: &mut BTreeMap<String, String>) {
    let name = ident_name(&item.name);
    if name.is_empty() {
        return;
    }
    let fqcn = match prefix {
        Some(prefix) => join_fqcn(prefix, &name),
        None => name,
    };
    let alias = item
        .alias
        .as_ref()
        .and_then(|alias| owned_name(alias.identifier.value))
        .unwrap_or_else(|| short_name(&fqcn).to_string());
    uses.insert(alias, fqcn);
}

fn ident_name(ident: &Identifier<'_>) -> String {
    owned_name(ident.value()).unwrap_or_else(|| bytes_to_string(ident.value()))
}
