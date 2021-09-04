use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::style_sheet;
use htmldsl::styles;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

fn link<'a>(url: &'static str, display_text: &'static str) -> elements::A<'a> {
    elements::A {
        href: attributes::Href {
            value: units::SourceValue::new(url.into()),
        },
        children: vec![htmldsl::text(display_text.into())],
        styles: attributes::StyleAttr {
            values: vec![&styles::Color {
                color_value: "#70a0ff",
            }],
        },
    }
}

fn main() {
    let html = elements::Html::style_less(
        Some(elements::Head::new(
            vec![elements::Meta::style_less(Some(attributes::Charset {
                value: units::CharsetValue::Utf8,
            }))],
            vec![elements::Style {
                style_sheet: style_sheet::StyleSheet {
                    assignments: vec![
                        style_sheet::StyleAssignment {
                            names: vec!["#main-content".into()],
                            styles: vec![
                                &styles::Margin::TopRightBottonLeft(
                                    units::NumberOrAuto::Number(units::Number::Length(0, units::Length::Pixel)),
                                    units::NumberOrAuto::Auto,
                                    units::NumberOrAuto::Number(units::Number::Length(0, units::Length::Pixel)),
                                    units::NumberOrAuto::Auto,
                                ),
                                &styles::Padding::TopRightBottonLeft(
                                    units::Number::Length(2, units::Length::Pixel),
                                    units::Number::Length(15, units::Length::Pixel),
                                    units::Number::Length(2, units::Length::Pixel),
                                    units::Number::Length(15, units::Length::Pixel),
                                ),
                                &styles::Width {
                                    value: units::NumberOrAuto::Number(units::Number::Length(
                                        800,
                                        units::Length::Pixel,
                                    )),
                                },
                                &styles::Height {
                                    value: units::NumberOrAuto::Number(units::Number::Percentage(
                                        100,
                                    )),
                                },
                                &styles::BackgroundColor {
                                    color_value: "#333333",
                                },
                            ],
                        },
                        style_sheet::StyleAssignment {
                            names: vec!["body".into()],
                            styles: vec![
                                &styles::Margin::TopRightBottonLeft(
                                    units::NumberOrAuto::Number(units::Number::Length(0, units::Length::Pixel)),
                                    units::NumberOrAuto::Auto,
                                    units::NumberOrAuto::Number(units::Number::Length(0, units::Length::Pixel)),
                                    units::NumberOrAuto::Auto,
                                ),
                                &styles::BackgroundColor {
                                    color_value: "#222222",
                                },
                                &styles::Color  {
                                    color_value: "#ccddcc",
                                },
                                &styles::FontFamily { name: "sans-serif" },
                            ],
                        },
                    ],
                },
            }],
        )),
        Some(elements::Body::style_less(vec![elements::Div {
            id: None,
            children: vec![
                elements::H1::style_less(vec![htmldsl::text("Quinten Palmer's Web Page".into())]).into_element(),
            ],
            styles: attributes::StyleAttr { values: Vec::new() },
        }
        .into_element()])),
        attributes::Lang {
            tag: units::LanguageTag::En,
            sub_tag: units::LanguageSubTag::Us,
        },
    );

    println!("{}", htmldsl::render_simple_html_page(true, html));
}
