```rust
struct Meta {
    title:       String,
    description: String,
} impl Component for Meta {
    fn render(self) -> impl HTML {
        [
            meta.charset("utf-8"),
            meta.name("viewport").content("width=device-width,initial-scale=1"),
            meta.http_equiv("Content-Type").content("text/html; charset=utf-8"),
            meta.http_equiv("Content-Language").content("ja"),
            meta.name("description").content(self.description),
            link.rel("icon").href("/blog/favicon.ico").sizes("any"),
            link.rel("icon").href("/blog/icon.svg").icon_type("image/svg+xml")
        ]
    }
}

struct App {
    title:       String,
    description: String,
    series:      Option<String>,
    volume:      usize,
    category:    Vec<String>,
    written_at:  Option<Date>,
    updated_at:  Option<Date>,
}
impl Component for App {
    fn render(self) -> impl HTML {
        html.lang("en")-[
            head.prefix("og: http://ogp.me/ns#"),
            Meta {
                title:       self.title,
                description: self.description
            },
            body-[
                header-[
                    h1-["kanarus"]
                ],
                div.id("content")-[
                    article-[
                        nav-[(self.series)._then(|series|
                            SeriesNavHeader {
                                series_name: series,
                                volume:      self.volume
                            }
                        )],
                        h2-[(self.title)._then(|title| title)],
                        div.id("metadata")-[
                            div.id("tags")-[
                                (self.category)._then(|categories|
                                    categories._for(|c|
                                        span.class("tag")-[category]
                                    )
                                )
                            ],
                            div.id("dates")-[
                                (self.written_at)._then(|written_at|
                                    svg.class("icon").view_box("0 0 32 32")-[
                                        path.d("M27 0c2.761 0 5 2.239 5 5 0 1.126-0.372 2.164-1 3l-2 2-7-7 2-2c0.836-0.628 1.874-1 3-1zM2 23l-2 9 9-2 18.5-18.5-7-7-18.5 18.5zM22.362 11.362l-14 14-1.724-1.724 14-14 1.724 1.724z")
                                    ],
                                    span.style("margin-left: 0.25em;")-[written_at]
                                ),
                                (self.updated_at)._then(|updated_at|
                                    svg
                                    .class("icons")
                                    .view_box("0 0 32 32")
                                    .style("margin-left: 0.5em;")-[
                                        path.d("M32 12h-12l4.485-4.485c-2.267-2.266-5.28-3.515-8.485-3.515s-6.219 1.248-8.485 3.515c-2.266 2.267-3.515 5.28-3.515 8.485s1.248 6.219 3.515 8.485c2.267 2.266 5.28 3.515 8.485 3.515s6.219-1.248 8.485-3.515c0.189-0.189 0.371-0.384 0.546-0.583l3.010 2.634c-2.933 3.349-7.239 5.464-12.041 5.464-8.837 0-16-7.163-16-16s7.163-16 16-16c4.418 0 8.418 1.791 11.313 4.687l4.687-4.687v12z")
                                    ],
                                    span.style("margin-left: 0.25em;")-[updated_at]
                                )
                            ]
                        ],
                        p-[(is_series_title(self.title))._then(
                            self.description
                        )],
                        nav-[content.series._then(|series|
                            SeriesNaviFooter {
                                series_name: series,
                                volume:      self.volume
                            }
                        )]
                    ],

                    Sidebar
                ]
            ]
        ]
    }
}
```