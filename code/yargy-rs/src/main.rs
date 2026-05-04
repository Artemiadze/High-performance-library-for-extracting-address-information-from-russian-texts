mod rules;

use crate::rules::build_address_rules;
use yargy_core::parser::Parser;
use yargy_core::RuleRegistry;

fn main() {
    let _ = yargy_core::load("code/yargy-rs/dicts");
    let (registry, addr_id) = build_address_rules();
    // Parser требует `&'r RuleRegistry<'r>`; для `RuleRegistry<'static>` нужна ссылка с тем же `'r`.
    let registry: &'static RuleRegistry<'static> = Box::leak(Box::new(registry));
    let parser = Parser::new(registry, addr_id);

    let text = "Россия, обл. Курская, р-н Золотухинский, рп Золотухино, ул. Куйбышева, дом 42";
    for m in parser.findall(text) {
        if let Some(fact) = m.fact(registry) {
            println!("{}", fact);
        }
    }
}