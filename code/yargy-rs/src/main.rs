mod rules;

use crate::rules::build_address_rules;
use renert::parser::Parser;
use renert::RuleRegistry;

fn main() {
    let _ = renert::load("code/yargy-rs/dicts");
    let (registry, addr_id) = build_address_rules();
    // Parser требует `&'r RuleRegistry<'r>`; для `RuleRegistry<'static>` нужна ссылка с тем же `'r`.
    let registry: &'static RuleRegistry<'static> = Box::leak(Box::new(registry));
    let parser = Parser::new(registry, addr_id);

    let text = "Россия, Курская область, р-н Золотухинский, рп Золотухино, ул. Куйбышева, дом 42";
    for m in parser.findall(text) {
        if let Some(fact) = m.fact(registry) {
            println!("{}", fact);
        }
    }
}