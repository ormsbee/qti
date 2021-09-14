/*
 * Models also needs to have something to verify the data relationships and hold
 * the Identifier lookups.
 */

type Identifier = String;
type Template = String;


#[derive(Debug, PartialEq)]
pub enum OrientationEnum {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq)]
pub enum ShowHideEnum {
    Show,
    Hide,
}

#[derive(Debug, PartialEq)]
pub struct SimpleChoice {
    pub identifier: Identifier,
    pub fixed: bool,
    pub template_identifier: Identifier,
    pub show_hide: ShowHideEnum,

    // This should be FlowStaticGroup, but this is a simplification for now.
    pub text: String,
}

#[derive(Debug, PartialEq)]
pub struct ChoiceInteraction {
    pub response_identifier: Identifier,
    pub shuffle: bool,
    pub max_choices: u32,
    pub min_choices: u32,
    pub orientation: OrientationEnum,
    pub choices: Vec<SimpleChoice>,
}
