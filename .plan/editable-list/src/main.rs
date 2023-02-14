use kabto::{
    html::HTML, component::Component, tag::*, css::{style_for, CSS},
    js::{String, Array, Optional, HTMLElement}
};

struct EditbaleList {
    title:         String,
    add_item_text: String,
    list_items:    Array<String>,
}
impl Component for EditableList {
    fn render(self) -> impl HTML {
        div-[
            style-[
                style_for("li, div > div")
                    .display(|d| d.Flex)
                    .align_items(|a| a.Center)
                    .justify_content(|j| j.SpaceBetween),
                style_for(".icon")
                    .back_ground_color("#fff")
                    .border(|b| b.None)
                    .cursor(|c| c.Pointer)
                    .float(|f| f.Right)
                    .font_size(1.8.rem()),

                // or

                style_for("li, div > div")
                    .display("flex")
                    .align_items("center")
                    .justify_content("space-between"),
                style_for(".icon")
                    .back_ground_color("#fff")
                    .border("none")
                    .cursor("pointer")
                    .float("right")
                    .font_size("1.8rem"),

                // or

                style_for("li, div > div", |s| [
                    s.display.flex(),
                    s.align_items.center(),
                    s.justify_content.space_between(),
                ]),
                style_for(".icon", |s| [
                    s.back_ground_color.code("#fff"),
                    s.border.none(),
                    s.cursor.pointer(),
                    s.float.right(),
                    s.font_size.rem(1.8)
                ]),

                // or

                "
                    li, div > div {
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                    }
                ",
                "
                    .icon {
                        back-ground-color: #fff;
                        border: none;
                        cursor: pointer;
                        float: right;
                        font-size: 1.8rem;
                    }
                ",

                // or

                style_for("li, div > div", CSS {
                    display: "flex",
                    align_items: "center",
                    justify_content: "space-between",
                    ..CSS::default()
                }),
                style_for(".icon", CSS {
                    back_ground_color: "#fff",
                    border: "none",
                    cursor: "pointer",
                    float: "right",
                    font_size: "1.8rem",
                    ..CSS::default()
                })
            ],
            h3-[self.title],
            ul.class("item-list")-[
                self.list_items._for(|item /*: String*/|
                    li-[
                        item,
                        button.class("editable-list-remove-item icon")
                        .on_click(self.remove_list_item)-[
                            "&ominus;"
                        ]
                    ]
                )
            ],
            div-[
                label-[self.add_item_text],
                input.class("add-new-list-item-input").input_type("text"),
                button.class("editable-list-add-item icon")
                .on_click(self.add_list_item)-[
                    "&oplus;"
                ]
            ]
        ]
    }
}
impl EditableList {
    fn add_list_item(&self) {
        let new_item_input = self.query_selector<input>(".add-new-list-item-input");
        let input_value: Optional<String> = new_item_input.value();

        input_value.if_contains(|text: String| {
            let childrem_len = self.item_list().children().len();

            self.item_list().append_child(
                li-[
                    text,
                    button.class("editable-list-remove-item icon")-[
                        "&ominus;"
                    ]
                ]
            );

            self.handle_remove_item_listeners([button]);
            input.clear_value();
        });
    }

    fn remove_list_item(&self, e: ClickEvent) {
        e.target.parent_node.remove()
    }
}

fn main() {
    struct App; impl Component for App {
        fn render(self) -> impl HTML {
            html.lang("en")-[
                head-[
                    meta.charset("utf-8"),
                    title-["Editable List"],
                ],
                body-[
                    EditableList{
                        title:         String::from("TODO"),
                        add_item_text: String::from("Add new list item:"),
                        list_items:    Array::from([
                            "First item on the list",
                            "Second item on the list",
                            "Third item on the list",
                            "Fourth item on the list",
                            "Fifth item on the list",
                            "This will not appear",
                        ])
                    }
                ]
            ]
        }
    }

    kabto::dev(App) // perform dev-build and serve
}
