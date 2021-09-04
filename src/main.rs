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
            id: Some(attributes::Id {
                name: "main-content",
            }),
            children: vec![
                elements::H1::style_less(vec![htmldsl::text("Quinten Palmer's Web Page".into())]).into_element(),

                elements::H2::style_less(vec![htmldsl::text("Introduction".into())]).into_element(),

                elements::P::style_less(vec![htmldsl::text("My name is Quinten Palmer.".into())])
                    .into_element(),

                elements::P::style_less(vec![htmldsl::text(
                    "This is my homepage; it's a work in progress.".into(),
                )])
                .into_element(),

                elements::H2::style_less(vec![htmldsl::text("Interests".into())]).into_element(),

                elements::P::style_less(vec![htmldsl::text(
                    "I like to build software, listen to music, and play games.".into(),
                )])
                .into_element(),

                elements::H3::style_less(vec![htmldsl::text("Software".into())]).into_element(),

                elements::P::style_less(vec![
                    htmldsl::text("My favorite workflow for building software is with the ".into()),
                    link("https://www.rust-lang.org", "Rust").into_element(),
                    htmldsl::text(" programming language, using ".into()),
                    link("https://git-scm.com", "git").into_element(),
                    htmldsl::text(" as a version control system, and with ".into()),
                    link("https://neovim.io", "Neovim").into_element(),
                    htmldsl::text(" as a text editor.".into()),
                ])
                .into_element(),

                elements::P::style_less(vec![
                    htmldsl::text("You can find software projects that I've worked on ".into()),
                    link("https://github.com/quintenpalmer", "GitHub").into_element(),
                    htmldsl::text(" and on ".into()),
                    link("https://gitlab.com/quintenpalmer", "GitLab").into_element(),
                    htmldsl::text(".".into()),
                ])
                .into_element(),

                elements::H3::style_less(vec![htmldsl::text("Music".into())]).into_element(),

                elements::P::style_less(vec![
                    htmldsl::text(
                        "I am building a music collection, with which I use the media server ".into(),
                    ),
                    link("https://jellyfin.org", "Jellyfin").into_element(),
                    htmldsl::text(
                        ", hosted on a ".into(),
                    ),
                    link("https://www.raspberrypi.org/products/raspberry-pi-4-model-b/", "Raspberry Pi 4 Model B").into_element(),
                    htmldsl::text(
                        ". I buy my music from ".into(),
                    ),
                    link("https://bandcamp.com", "Bandcamp").into_element(),
                    htmldsl::text(" and ".into()),
                    link("https://www.qobuz.com/us-en/shop", "Qobuz").into_element(),
                    htmldsl::text(".".into()),
                ])
                .into_element(),

                elements::H3::style_less(vec![htmldsl::text("Games".into())]).into_element(),

                elements::H4::style_less(vec![htmldsl::text("Magic: The Gathering".into())]).into_element(),

                elements::P::style_less(vec![
                    htmldsl::text(
                        "I am obsessed with Magic: The Gathering and like to brew new decks using ".into(),
                    ),
                    link("https://scryfall.com", "Scryfall").into_element(),
                    htmldsl::text(" and purchase cards from ".into()),
                    link("https://www.tcgplayer.com", "TCGplayer").into_element(),
                    htmldsl::text(" and ".into()),
                    link("https://www.cardkingdom.com", "Card Kingdom").into_element(),
                    htmldsl::text(".".into()),
                ])
                .into_element(),

                elements::H4::style_less(vec![htmldsl::text("Board Games".into())]).into_element(),

                elements::P::style_less(vec![
                    htmldsl::text(
                        "I enjoy playing board games as well, especially: ".into(),
                    ),
                    link("https://www.riograndegames.com/games/dominion/", "Dominion").into_element(),
                    htmldsl::text(", ".into()),
                    link("https://www.daysofwonder.com/online/en/splendor/", "Splendor").into_element(),
                    htmldsl::text(", and ".into()),
                    link("https://www.zmangames.com/en/games/carcassone/", "Carcassonne").into_element(),
                    htmldsl::text(".".into()),
                ]).into_element(),

                elements::H4::style_less(vec![htmldsl::text("Video Games".into())]).into_element(),

                elements::P::style_less(vec![
                    htmldsl::text(
                        "I grew up on, and still love, video games. Some notable call-outs of my favorite games are: ".into(),
                    ),
                    link("https://en.wikipedia.org/wiki/Celeste_(video_game)", "Celeste").into_element(),
                    htmldsl::text(", ".into()),
                    link("https://en.wikipedia.org/wiki/The_Legend_of_Zelda:_Majora%27s_Mask", "The Legend of Zelda: Majora's Mask").into_element(),
                    htmldsl::text(", ".into()),
                    link("https://en.wikipedia.org/wiki/The_Legend_of_Zelda:_The_Wind_Waker", "The Legend of Zelda: The Wind Waker").into_element(),
                    htmldsl::text(", ".into()),
                    link("https://www.factorio.com/", "Factorio").into_element(),
                    htmldsl::text(", ".into()),
                    link("https://elderscrolls.bethesda.net/en/skyrim/", "The Elder Scrolls V: Skyrim").into_element(),
                    htmldsl::text(", and ".into()),
                    link("https://www.rocketleague.com/", "Rocket League").into_element(),
                    htmldsl::text(".".into()),
                ]).into_element(),
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
