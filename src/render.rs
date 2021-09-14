use crate::model::{ChoiceInteraction, OrientationEnum, ShowHideEnum, SimpleChoice};

pub trait RenderHtml {
    fn html(&self) -> String;
}

impl RenderHtml for ChoiceInteraction {
    fn html(&self) -> String {
        let choices_html: Vec<String> = self.choices.iter().map(|choice|
            // todo: needs proper escaping
            format!(
                "<input type=\"radio\" name=\"placeholder_name\" value=\"{}\"><label>{}</label><br>",
                choice.identifier,
                choice.text,
            )
        ).collect();

        format!(
            "<fieldset id=\"{}\">{}</fieldset>",
            self.response_identifier,
            choices_html.join("\n"),
        )
    }
}
