use std::num::ParseIntError;

use crate::model::{ChoiceInteraction, OrientationEnum, ShowHideEnum, SimpleChoice};
use xmltree::{Element, XMLNode};

fn parse_uint_with_default(maybe_int_str: Option<&String>, default: u32) -> u32 {
    if let Some(int_str) = maybe_int_str {
        // version 1 that returned a Result<u32, ParseIntError>
        // return int_str.parse::<u32>();

        // v2 (until we get real errors), just return default if it parses incorrectly.
        match int_str.parse::<u32>() {
            Ok(number) => return number,
            Err(_) => return default,
        }
    }
    return default;
}

fn parse_simple_choice(el: &Element) -> SimpleChoice {
    let identifier = match el.attributes.get("identifier") {
        Some(id_str) => id_str,
        None => "generic_id",
    };
    let mut text_els: Vec<String> = Vec::new();
    for text_el in el.children.iter() {
        match text_el {
            XMLNode::Text(text) => text_els.push(text.to_string()),
            _ => ()
        }
    }

    return SimpleChoice {
        identifier: identifier.to_string(),
        fixed: false,
        template_identifier: "bar".to_string(),
        show_hide: ShowHideEnum::Show,
        text: text_els.join(" "),
    };
}

pub fn parse_choice_interaction(el: &Element) -> ChoiceInteraction {
    let max_choices = parse_uint_with_default(el.attributes.get("max-choices"), 0);
    let min_choices = parse_uint_with_default(el.attributes.get("min-choices"), 0);
    let mut choice_els: Vec<&Element> = Vec::new();
    for choice_el in el.children.iter() {
        match choice_el {
            XMLNode::Element(element) => choice_els.push(element),
            _ => (),
        }
    }
    let choices = choice_els.iter().map(|ch_el| parse_simple_choice(ch_el)).collect();

    return ChoiceInteraction {
        response_identifier: "stub_response_id".to_string(),
        shuffle: false,
        max_choices: max_choices,
        min_choices: min_choices,
        orientation: OrientationEnum::Vertical,
        choices: choices,
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{ChoiceInteraction, OrientationEnum, SimpleChoice};
    use xmltree::Element;

    #[test]
    fn basic_choice_interaction() {
        const CHOICE_INTERACTION_EXAMPLE: &'static str = r#"<qti-choice-interaction
                    max-choices="1" min-choices="1" response-identifier="RESPONSE">
                <qti-simple-choice identifier="A">Epinephrine</qti-simple-choice>
                <qti-simple-choice identifier="B">Glucagon</qti-simple-choice>
                <qti-simple-choice identifier="C">Insulin</qti-simple-choice>
                <qti-simple-choice identifier="D">Oxytocin</qti-simple-choice>
            </qti-choice-interaction>"#;

        let root = Element::parse(CHOICE_INTERACTION_EXAMPLE.as_bytes()).unwrap();

        println!("{:#?}", root);

        let choice_interaction = super::parse_choice_interaction(&root);

        println!("{:#?}", choice_interaction);

        assert_eq!(1, 2);
    }
}
