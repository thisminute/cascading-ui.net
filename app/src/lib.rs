extern crate cascading_ui;
use cascading_ui::cui;

cui! {
    title: "CUI -- Cascading UI";

    // ── Theme variables ──
    let $bg: "white";
    let $fg: "#1a1a2e";
    let $tile: "#f7f8fa";
    let $code_bg: "#1a1a2e";
    let $accent: "#5865f2";

    // ── Page routing ──
    let $show_home: "block";
    let $show_tutorial: "none";

    // ── Lesson routing ──
    let $show_cover: "block";
    let $show_l1: "none";
    let $show_l2: "none";
    let $show_l3: "none";
    let $show_l4: "none";
    let $show_l5: "none";

    // ── Compiled-result panels ──
    let $compiled_1: "none";
    let $compiled_2: "none";
    let $compiled_3: "none";
    let $compiled_4: "none";
    let $compiled_5: "none";

    // ── Per-lesson expand/collapse toggle class pairs ──
    // Each pair toggles a specific $compiled_N variable via apply.

    .l1_show { text: "▶ Show compiled HTML + CSS"; cursor: "pointer"; color: $accent; background: "transparent"; border: "none"; padding: "8px 0"; font-weight: "500"; font-size: "0.9rem"; ?click { $compiled_1: "block"; apply: .l1_hide; } }
    .l1_hide { text: "▼ Hide compiled HTML + CSS"; cursor: "pointer"; color: $accent; background: "transparent"; border: "none"; padding: "8px 0"; font-weight: "500"; font-size: "0.9rem"; ?click { $compiled_1: "none"; apply: .l1_show; } }

    .l2_show { text: "▶ Show compiled HTML + CSS"; cursor: "pointer"; color: $accent; background: "transparent"; border: "none"; padding: "8px 0"; font-weight: "500"; font-size: "0.9rem"; ?click { $compiled_2: "block"; apply: .l2_hide; } }
    .l2_hide { text: "▼ Hide compiled HTML + CSS"; cursor: "pointer"; color: $accent; background: "transparent"; border: "none"; padding: "8px 0"; font-weight: "500"; font-size: "0.9rem"; ?click { $compiled_2: "none"; apply: .l2_show; } }

    .l3_show { text: "▶ Show compiled HTML + CSS"; cursor: "pointer"; color: $accent; background: "transparent"; border: "none"; padding: "8px 0"; font-weight: "500"; font-size: "0.9rem"; ?click { $compiled_3: "block"; apply: .l3_hide; } }
    .l3_hide { text: "▼ Hide compiled HTML + CSS"; cursor: "pointer"; color: $accent; background: "transparent"; border: "none"; padding: "8px 0"; font-weight: "500"; font-size: "0.9rem"; ?click { $compiled_3: "none"; apply: .l3_show; } }

    .l4_show { text: "▶ Show compiled HTML + CSS"; cursor: "pointer"; color: $accent; background: "transparent"; border: "none"; padding: "8px 0"; font-weight: "500"; font-size: "0.9rem"; ?click { $compiled_4: "block"; apply: .l4_hide; } }
    .l4_hide { text: "▼ Hide compiled HTML + CSS"; cursor: "pointer"; color: $accent; background: "transparent"; border: "none"; padding: "8px 0"; font-weight: "500"; font-size: "0.9rem"; ?click { $compiled_4: "none"; apply: .l4_show; } }

    .l5_show { text: "▶ Show compiled HTML + CSS"; cursor: "pointer"; color: $accent; background: "transparent"; border: "none"; padding: "8px 0"; font-weight: "500"; font-size: "0.9rem"; ?click { $compiled_5: "block"; apply: .l5_hide; } }
    .l5_hide { text: "▼ Hide compiled HTML + CSS"; cursor: "pointer"; color: $accent; background: "transparent"; border: "none"; padding: "8px 0"; font-weight: "500"; font-size: "0.9rem"; ?click { $compiled_5: "none"; apply: .l5_show; } }

    // ── Theme toggle ──
    .light_mode_button {
        text: "Dark";
        cursor: "pointer";
        padding: "6px 16px";
        border-radius: "6px";
        font-size: "0.85rem";
        background: $tile;
        color: $fg;
        ?click {
            $bg: "#0f0f23";
            $fg: "#e2e8f0";
            $tile: "#1a1a2e";
            $code_bg: "#131325";
            apply: .dark_mode_button;
        }
    }
    .dark_mode_button {
        text: "Light";
        cursor: "pointer";
        padding: "6px 16px";
        border-radius: "6px";
        font-size: "0.85rem";
        background: $tile;
        color: $fg;
        ?click {
            $bg: "white";
            $fg: "#1a1a2e";
            $tile: "#f7f8fa";
            $code_bg: "#1a1a2e";
            apply: .light_mode_button;
        }
    }

    // ── Sidebar menu item classes ──
    .menu_item {
        padding: "8px 16px";
        cursor: "pointer";
        border-radius: "4px";
        font-size: "0.85rem";
        margin-bottom: "4px";
        color: $fg;
    }

    page {
        background: $bg;
        color: $fg;
        min-height: "100vh";
        font-family: "system-ui, -apple-system, sans-serif";
        line-height: "1.7";

        // ── Fixed theme toggle ──
        toggle_wrap {
            position: "fixed";
            top: "16px";
            right: "16px";
            z-index: "100";

            light_mode_button {}
        }

        // ══════════════════════════════════════════════
        //  HOME PAGE
        // ══════════════════════════════════════════════
        home_page {
            display: $show_home;
            max-width: "720px";
            margin: "0 auto";
            padding: "0 24px 80px";

            hero {
                padding: "72px 0 0";
                margin-bottom: "56px";
                text-align: "center";

                hero_title {
                    text: "CUI";
                    font-size: "3rem";
                    font-weight: "800";
                    letter-spacing: "-0.04em";
                    margin-bottom: "8px";
                }
                hero_subtitle {
                    text: "A web language based on CSS syntax that compiles to HTML + Wasm. No JavaScript.";
                    font-size: "1.15rem";
                    font-weight: "400";
                    opacity: "0.6";
                    margin-bottom: "32px";
                }
                hero_example {
                    text: ".button {\n    background: \"blue\";\n    color: \"white\";\n    cursor: \"pointer\";\n}\n\npage {\n    button {\n        text: \"click me\";\n        ?click {\n            text: \"clicked!\";\n        }\n    }\n}";
                    background: $code_bg;
                    color: "#e2e8f0";
                    padding: "24px 28px";
                    border-radius: "10px 10px 0 0";
                    font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                    font-size: "0.85rem";
                    text-align: "left";
                    overflow-x: "auto";
                    line-height: "1.6";
                    white-space: "pre";
                }
                hero_demo {
                    background: $tile;
                    padding: "20px 28px";
                    border-radius: "0 0 10px 10px";
                    border-top: "1px solid #333";
                    text-align: "left";

                    hero_demo_label {
                        text: "Live output:";
                        font-size: "0.75rem";
                        text-transform: "uppercase";
                        letter-spacing: "0.05em";
                        opacity: "0.5";
                        margin-bottom: "12px";
                    }

                    .hero_button {
                        background: "blue";
                        color: "white";
                        cursor: "pointer";
                    }

                    hero_button {
                        let $hero_text: "click me";
                        text: $hero_text;
                        padding: "8px 16px";
                        border-radius: "4px";
                        border: "none";
                        font-size: "0.9rem";
                        display: "inline-block";

                        ?click {
                            $hero_text: "clicked!";
                        }
                    }
                }
            }

            section_what {
                margin-bottom: "56px";

                what_title {
                    text: "What is CUI?";
                    font-size: "1.35rem";
                    font-weight: "700";
                    letter-spacing: "-0.01em";
                    margin-bottom: "20px";
                }
                what_body {
                    text: "CUI is a compiled language where structure, style, and behavior live in one CSS-like syntax. Classes define how things look. Instances create them. Listeners handle events. The compiler figures out the rest -- what's static gets baked into HTML, what's dynamic gets compiled to WebAssembly.";
                    line-height: "1.6";
                    color: $fg;
                    margin-bottom: "16px";
                }
            }

            section_why {
                margin-bottom: "56px";

                why_title {
                    text: "Why CUI?";
                    font-size: "1.35rem";
                    font-weight: "700";
                    letter-spacing: "-0.01em";
                    margin-bottom: "20px";
                }

                .why_point {
                    margin-bottom: "24px";
                    background: $tile;
                    padding: "20px 24px";
                    border-radius: "8px";
                }
                .why_point_title {
                    font-weight: "700";
                    font-size: "1rem";
                    margin-bottom: "4px";
                }
                .why_point_body {
                    font-size: "0.95rem";
                    opacity: "0.7";
                    line-height: "1.6";
                    color: $fg;
                }

                why_point {
                    why_point_title { text: "Zero JavaScript"; }
                    why_point_body { text: "CUI compiles to WebAssembly. No bundler, no transpiler, no node_modules. Event handlers and reactivity run as native Wasm."; }
                }
                why_point {
                    why_point_title { text: "Three-layer compilation"; }
                    why_point_body { text: "The compiler detects what's fully static (baked into HTML), what needs one-time setup (wired up at page load), and what's truly reactive. Static pages have zero runtime cost."; }
                }
                why_point {
                    why_point_title { text: "CSS semantics you already know"; }
                    why_point_body { text: "Classes cascade. Properties inherit. If you know CSS, you know the mental model. CUI extends it to structure and behavior."; }
                }
                why_point {
                    why_point_title { text: "Three block types, that's it"; }
                    why_point_body { text: "Instances create elements. Classes define their appearance. Listeners handle events. Everything else is a property. No components, no hooks, no lifecycle methods."; }
                }
            }

            section_try {
                margin-bottom: "56px";

                try_title {
                    text: "Try it";
                    font-size: "1.35rem";
                    font-weight: "700";
                    letter-spacing: "-0.01em";
                    margin-bottom: "20px";
                }
                try_body {
                    text: "This page is built with CUI. The button below is a live Wasm element:";
                    line-height: "1.6";
                    color: $fg;
                    margin-bottom: "16px";
                }

                try_demo {
                    background: $tile;
                    border-radius: "10px";
                    padding: "32px 24px";
                    text-align: "center";
                    margin: "20px 0";

                    demo_button {
                        let $label: "Click me";
                        text: $label;
                        background: $accent;
                        color: "#e2e8f0";
                        padding: "10px 20px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "500";
                        font-size: "0.95rem";
                        border: "none";
                        display: "inline-block";

                        ?click {
                            $label: "Clicked! This ran through Wasm.";
                        }
                    }
                }
            }

            section_tutorial {
                margin-bottom: "56px";

                tutorial_title {
                    text: "Interactive Tutorial";
                    font-size: "1.35rem";
                    font-weight: "700";
                    letter-spacing: "-0.01em";
                    margin-bottom: "20px";
                }
                tutorial_body {
                    text: "Learn CUI step by step. See concepts, code, and live demos that build on each other.";
                    line-height: "1.6";
                    color: $fg;
                    margin-bottom: "16px";
                }

                tutorial_cta {
                    text: "Start Tutorial →";
                    background: $accent;
                    color: "white";
                    padding: "12px 24px";
                    border-radius: "6px";
                    cursor: "pointer";
                    font-weight: "600";
                    border: "none";
                    font-size: "1rem";
                    display: "inline-block";

                    ?click {
                        $show_home: "none";
                        $show_tutorial: "flex";
                    }
                }
            }

            section_how {
                margin-bottom: "56px";

                how_title {
                    text: "How it works";
                    font-size: "1.35rem";
                    font-weight: "700";
                    letter-spacing: "-0.01em";
                    margin-bottom: "20px";
                }
                how_body_1 {
                    text: "CUI is a Rust procedural macro. Your source is parsed, analyzed, and compiled at build time:";
                    line-height: "1.6";
                    color: $fg;
                    margin-bottom: "16px";
                }
                how_pipeline {
                    text: "CUI source\n  -> parse\n  -> AST\n  -> analyze\n  -> semantics tree\n  -> cascade classes into instances\n  -> compile\n  -> HTML + CSS + Wasm";
                    background: $code_bg;
                    color: "#e2e8f0";
                    padding: "20px 24px";
                    border-radius: "10px";
                    font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                    font-size: "0.85rem";
                    overflow-x: "auto";
                    line-height: "1.6";
                    white-space: "pre";
                    margin: "20px 0";
                }
                how_body_2 {
                    text: "The cascade phase resolves class inheritance and variable scoping, and assigns each piece of content to a compilation layer. Only reactive parts incur runtime cost.";
                    line-height: "1.6";
                    color: $fg;
                    margin-bottom: "16px";
                }
            }

            section_links {
                margin-bottom: "56px";

                links_title {
                    text: "Get started";
                    font-size: "1.35rem";
                    font-weight: "700";
                    letter-spacing: "-0.01em";
                    margin-bottom: "20px";
                }
                github_link {
                    color: $accent;
                    font-weight: "500";
                    link: "https://github.com/thisminute/cascading-ui";
                    text: "github.com/thisminute/cascading-ui";
                    text-decoration: "none";
                }
            }

            home_footer {
                text: "Built with CUI.";
                margin-top: "80px";
                padding-top: "24px";
                border-top: "1px solid #e8e8e8";
                text-align: "center";
                font-size: "0.85rem";
                opacity: "0.5";
            }
        }

        // ══════════════════════════════════════════════
        //  TUTORIAL PAGE (flex layout: sidebar + content)
        // ══════════════════════════════════════════════
        tutorial_page {
            display: $show_tutorial;
            max-width: "1100px";
            margin: "0 auto";
            padding: "24px";

            // ── Sidebar ──
            sidebar {
                width: "200px";
                padding-right: "24px";
                padding-top: "48px";
                flex-shrink: "0";

                sidebar_title {
                    text: "Tutorial";
                    font-weight: "700";
                    font-size: "0.95rem";
                    margin-bottom: "16px";
                }

                home_link {
                    text: "← Home";
                    cursor: "pointer";
                    padding: "8px 16px";
                    border-radius: "4px";
                    font-size: "0.85rem";
                    margin-bottom: "16px";
                    color: $accent;
                    font-weight: "500";

                    ?click {
                        $show_tutorial: "none";
                        $show_home: "block";
                    }
                }

                menu_overview {
                    text: "Overview";
                    cursor: "pointer";
                    padding: "8px 16px";
                    border-radius: "4px";
                    font-size: "0.85rem";
                    margin-bottom: "4px";
                    color: $fg;

                    ?click {
                        $show_cover: "block";
                        $show_l1: "none";
                        $show_l2: "none";
                        $show_l3: "none";
                        $show_l4: "none";
                        $show_l5: "none";
                    }
                }
                menu_1 {
                    text: "1. Text";
                    cursor: "pointer";
                    padding: "8px 16px";
                    border-radius: "4px";
                    font-size: "0.85rem";
                    margin-bottom: "4px";
                    color: $fg;

                    ?click {
                        $show_cover: "none";
                        $show_l1: "block";
                        $show_l2: "none";
                        $show_l3: "none";
                        $show_l4: "none";
                        $show_l5: "none";
                    }
                }
                menu_2 {
                    text: "2. Structure";
                    cursor: "pointer";
                    padding: "8px 16px";
                    border-radius: "4px";
                    font-size: "0.85rem";
                    margin-bottom: "4px";
                    color: $fg;

                    ?click {
                        $show_cover: "none";
                        $show_l1: "none";
                        $show_l2: "block";
                        $show_l3: "none";
                        $show_l4: "none";
                        $show_l5: "none";
                    }
                }
                menu_3 {
                    text: "3. Classes";
                    cursor: "pointer";
                    padding: "8px 16px";
                    border-radius: "4px";
                    font-size: "0.85rem";
                    margin-bottom: "4px";
                    color: $fg;

                    ?click {
                        $show_cover: "none";
                        $show_l1: "none";
                        $show_l2: "none";
                        $show_l3: "block";
                        $show_l4: "none";
                        $show_l5: "none";
                    }
                }
                menu_4 {
                    text: "4. Events";
                    cursor: "pointer";
                    padding: "8px 16px";
                    border-radius: "4px";
                    font-size: "0.85rem";
                    margin-bottom: "4px";
                    color: $fg;

                    ?click {
                        $show_cover: "none";
                        $show_l1: "none";
                        $show_l2: "none";
                        $show_l3: "none";
                        $show_l4: "block";
                        $show_l5: "none";
                    }
                }
                menu_5 {
                    text: "5. All Together";
                    cursor: "pointer";
                    padding: "8px 16px";
                    border-radius: "4px";
                    font-size: "0.85rem";
                    margin-bottom: "4px";
                    color: $fg;

                    ?click {
                        $show_cover: "none";
                        $show_l1: "none";
                        $show_l2: "none";
                        $show_l3: "none";
                        $show_l4: "none";
                        $show_l5: "block";
                    }
                }
            }

            // ── Main tutorial content ──
            tutorial_content {
                flex: "1";
                min-width: "0";
                padding-top: "48px";

                // ── Cover ──
                cover {
                    display: $show_cover;

                    cover_title {
                        text: "CUI Interactive Tutorial";
                        font-size: "2rem";
                        font-weight: "700";
                        margin-bottom: "16px";
                    }
                    cover_body {
                        text: "Learn CUI by building up concepts one at a time. Each lesson shows you: what a feature does, the CUI code that uses it, a live demo, and the compiled output.";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }
                    cover_start {
                        text: "Start with Lesson 1 →";
                        background: $accent;
                        color: "white";
                        padding: "12px 24px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "600";
                        border: "none";
                        font-size: "1rem";
                        display: "inline-block";

                        ?click {
                            $show_cover: "none";
                            $show_l1: "block";
                        }
                    }
                }

                // ═══════════════════════════════════
                //  LESSON 1: Text
                // ═══════════════════════════════════
                lesson_1 {
                    display: $show_l1;

                    l1_title {
                        text: "Lesson 1: The Text Property";
                        font-size: "1.6rem";
                        font-weight: "700";
                        color: $accent;
                        margin-bottom: "8px";
                    }
                    l1_subtitle {
                        text: "The simplest thing: putting words on the page";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }
                    l1_intro {
                        text: "Every element can have a text property. It sets the text content of that element. The compiler detects that this is static and bakes it directly into the HTML file. No runtime cost.";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l1_code_label {
                        text: "CUI source:";
                        font-weight: "600";
                        margin-bottom: "8px";
                    }
                    l1_code {
                        text: "greeting {\n    text: \"Hello, CUI!\";\n}";
                        background: $code_bg;
                        color: "#e2e8f0";
                        padding: "16px 20px";
                        border-radius: "8px";
                        font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                        font-size: "0.85rem";
                        overflow-x: "auto";
                        white-space: "pre";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l1_demo_label {
                        text: "Live result:";
                        font-weight: "600";
                        margin-bottom: "8px";
                    }
                    l1_demo {
                        background: $tile;
                        padding: "24px";
                        border-radius: "8px";
                        margin-bottom: "24px";

                        greeting {
                            text: "Hello, CUI!";
                        }
                    }

                    // Expand / collapse compiled output
                    l1_show {}
                    l1_compiled {
                        display: $compiled_1;
                        background: $tile;
                        padding: "16px 20px";
                        border-radius: "8px";
                        margin-top: "16px";

                        c1_html_label {
                            text: "Compiled HTML:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c1_html {
                            text: "<div>Hello, CUI!</div>";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "16px";
                            white-space: "pre";
                        }
                        c1_css_label {
                            text: "Compiled CSS:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c1_css {
                            text: "/* No CSS needed — pure text content */";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "12px";
                            white-space: "pre";
                        }
                        c1_note {
                            text: "Wasm: none needed. Static text is baked directly into HTML.";
                            font-size: "0.85rem";
                            color: "#888";
                            font-style: "italic";
                        }
                    }

                    l1_nav {
                        margin-top: "40px";
                        display: "flex";
                        justify-content: "flex-end";

                        l1_next {
                            text: "Next: Elements & Structure →";
                            background: $accent;
                            color: "white";
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $show_l1: "none";
                                $show_l2: "block";
                            }
                        }
                    }
                }

                // ═══════════════════════════════════
                //  LESSON 2: Elements & Structure
                // ═══════════════════════════════════
                lesson_2 {
                    display: $show_l2;

                    l2_title {
                        text: "Lesson 2: Elements & Structure";
                        font-size: "1.6rem";
                        font-weight: "700";
                        color: $accent;
                        margin-bottom: "8px";
                    }
                    l2_subtitle {
                        text: "Building hierarchy without thinking about tags";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }
                    l2_intro {
                        text: "Elements in CUI are just names followed by braces. Nesting creates parent-child relationships in the DOM. The compiler maps names to HTML tags.";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l2_code_label {
                        text: "CUI source:";
                        font-weight: "600";
                        margin-bottom: "8px";
                    }
                    l2_code {
                        text: "card {\n    card_title {\n        text: \"My Card\";\n    }\n    card_body {\n        text: \"Content inside the card.\";\n    }\n}";
                        background: $code_bg;
                        color: "#e2e8f0";
                        padding: "16px 20px";
                        border-radius: "8px";
                        font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                        font-size: "0.85rem";
                        overflow-x: "auto";
                        white-space: "pre";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l2_demo_label {
                        text: "Live result:";
                        font-weight: "600";
                        margin-bottom: "8px";
                    }
                    l2_demo {
                        background: $tile;
                        padding: "24px";
                        border-radius: "8px";
                        margin-bottom: "24px";

                        card {
                            border-left: "4px solid #5865f2";
                            padding: "16px";
                            border-radius: "4px";

                            card_title {
                                text: "My Card";
                                font-weight: "700";
                                margin-bottom: "8px";
                            }
                            card_body {
                                text: "Content inside the card.";
                                opacity: "0.7";
                            }
                        }
                    }

                    l2_show {}
                    l2_compiled {
                        display: $compiled_2;
                        background: $tile;
                        padding: "16px 20px";
                        border-radius: "8px";
                        margin-top: "16px";

                        c2_html_label {
                            text: "Compiled HTML:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c2_html {
                            text: "<div class=\"a\">\n  <div class=\"b\">My Card</div>\n  <div class=\"c\">Content inside the card.</div>\n</div>";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "16px";
                            white-space: "pre";
                        }
                        c2_css_label {
                            text: "Compiled CSS:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c2_css {
                            text: ".a { border-left: 4px solid #5865f2;\n     padding: 16px; border-radius: 4px; }\n.b { font-weight: 700; margin-bottom: 8px; }\n.c { opacity: 0.7; }";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "12px";
                            white-space: "pre";
                        }
                        c2_note {
                            text: "Wasm: none needed. All static content. Each element gets a short generated class selector.";
                            font-size: "0.85rem";
                            color: "#888";
                            font-style: "italic";
                        }
                    }

                    l2_nav {
                        margin-top: "40px";
                        display: "flex";
                        justify-content: "space-between";

                        l2_prev {
                            text: "← Text";
                            background: $tile;
                            color: $fg;
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $show_l2: "none";
                                $show_l1: "block";
                            }
                        }
                        l2_next {
                            text: "Classes & Cascading →";
                            background: $accent;
                            color: "white";
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $show_l2: "none";
                                $show_l3: "block";
                            }
                        }
                    }
                }

                // ═══════════════════════════════════
                //  LESSON 3: Classes & Cascading
                // ═══════════════════════════════════
                lesson_3 {
                    display: $show_l3;

                    l3_title {
                        text: "Lesson 3: Classes & Cascading";
                        font-size: "1.6rem";
                        font-weight: "700";
                        color: $accent;
                        margin-bottom: "8px";
                    }
                    l3_subtitle {
                        text: "Define once, reuse everywhere";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }
                    l3_intro {
                        text: "A class (name starting with .) defines properties that cascade into all same-named instances. Instance properties override class properties. Classes can appear before or after instances — the compiler hoists them.";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l3_code_label {
                        text: "CUI source:";
                        font-weight: "600";
                        margin-bottom: "8px";
                    }
                    l3_code {
                        text: ".tag {\n    padding: \"6px 12px\";\n    border-radius: \"4px\";\n    color: \"white\";\n    font-size: \"0.9rem\";\n}\n\ntag { text: \"Default\"; background: \"#5865f2\"; }\ntag { text: \"Custom\";  background: \"#ff6b6b\"; }\ntag { text: \"Another\"; background: \"#ffa500\"; }";
                        background: $code_bg;
                        color: "#e2e8f0";
                        padding: "16px 20px";
                        border-radius: "8px";
                        font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                        font-size: "0.85rem";
                        overflow-x: "auto";
                        white-space: "pre";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l3_demo_label {
                        text: "Live result:";
                        font-weight: "600";
                        margin-bottom: "8px";
                    }
                    l3_demo {
                        background: $tile;
                        padding: "24px";
                        border-radius: "8px";
                        margin-bottom: "24px";
                        display: "flex";
                        gap: "8px";

                        .tag {
                            padding: "6px 12px";
                            border-radius: "4px";
                            color: "white";
                            font-size: "0.9rem";
                        }

                        tag {
                            text: "Default";
                            background: "#5865f2";
                        }
                        tag {
                            text: "Custom";
                            background: "#ff6b6b";
                        }
                        tag {
                            text: "Another";
                            background: "#ffa500";
                        }
                    }

                    l3_show {}
                    l3_compiled {
                        display: $compiled_3;
                        background: $tile;
                        padding: "16px 20px";
                        border-radius: "8px";
                        margin-top: "16px";

                        c3_html_label {
                            text: "Compiled HTML:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c3_html {
                            text: "<div class=\"d\">Default</div>\n<div class=\"d\">Custom</div>\n<div class=\"d\">Another</div>";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "16px";
                            white-space: "pre";
                        }
                        c3_css_label {
                            text: "Compiled CSS:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c3_css {
                            text: ".d { padding: 6px 12px; border-radius: 4px;\n     color: white; font-size: 0.9rem; }\n/* backgrounds are instance-specific, set inline */";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "12px";
                            white-space: "pre";
                        }
                        c3_note {
                            text: "Wasm: none. The class compiles to one CSS rule shared by all instances. Instance overrides go inline.";
                            font-size: "0.85rem";
                            color: "#888";
                            font-style: "italic";
                        }
                    }

                    l3_nav {
                        margin-top: "40px";
                        display: "flex";
                        justify-content: "space-between";

                        l3_prev {
                            text: "← Structure";
                            background: $tile;
                            color: $fg;
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $show_l3: "none";
                                $show_l2: "block";
                            }
                        }
                        l3_next {
                            text: "Events & Interactivity →";
                            background: $accent;
                            color: "white";
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $show_l3: "none";
                                $show_l4: "block";
                            }
                        }
                    }
                }

                // ═══════════════════════════════════
                //  LESSON 4: Events
                // ═══════════════════════════════════
                lesson_4 {
                    display: $show_l4;

                    l4_title {
                        text: "Lesson 4: Events & Interactivity";
                        font-size: "1.6rem";
                        font-weight: "700";
                        color: $accent;
                        margin-bottom: "8px";
                    }
                    l4_subtitle {
                        text: "Making things respond to user actions";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }
                    l4_intro {
                        text: "Listeners (?click, ?blur, ?focus, etc.) respond to browser events. Inside a listener you can change text, modify CSS properties, or create new elements. Everything compiles to WebAssembly.";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l4_code_label {
                        text: "CUI source:";
                        font-weight: "600";
                        margin-bottom: "8px";
                    }
                    l4_code {
                        text: "let $msg: \"Click me\";\n\nbutton {\n    text: $msg;\n    color: \"white\";\n    background: \"#5865f2\";\n    cursor: \"pointer\";\n\n    ?click {\n        $msg: \"Clicked! This runs in Wasm.\";\n        background: \"#ff6b6b\";\n    }\n}";
                        background: $code_bg;
                        color: "#e2e8f0";
                        padding: "16px 20px";
                        border-radius: "8px";
                        font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                        font-size: "0.85rem";
                        overflow-x: "auto";
                        white-space: "pre";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l4_demo_label {
                        text: "Live result:";
                        font-weight: "600";
                        margin-bottom: "8px";
                    }
                    l4_demo {
                        background: $tile;
                        padding: "24px";
                        border-radius: "8px";
                        margin-bottom: "24px";

                        let $msg: "Click me";

                        button {
                            text: $msg;
                            color: "white";
                            background: "#5865f2";
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "500";
                            border: "none";
                            display: "inline-block";

                            ?click {
                                $msg: "Clicked! This runs in Wasm.";
                                background: "#ff6b6b";
                            }
                        }
                    }

                    l4_show {}
                    l4_compiled {
                        display: $compiled_4;
                        background: $tile;
                        padding: "16px 20px";
                        border-radius: "8px";
                        margin-top: "16px";

                        c4_html_label {
                            text: "Compiled HTML:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c4_html {
                            text: "<div class=\"e\">Click me</div>";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "16px";
                            white-space: "pre";
                        }
                        c4_css_label {
                            text: "Compiled CSS:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c4_css {
                            text: ".e { color: white; background: #5865f2;\n     padding: 10px 20px; cursor: pointer; }";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "16px";
                            white-space: "pre";
                        }
                        c4_wasm_label {
                            text: "Compiled to Wasm:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c4_wasm {
                            text: "// The ?click listener compiles to a Wasm function:\n//   - Updates text content: \"Click me\" -> \"Clicked!...\"\n//   - Updates style.background: \"#5865f2\" -> \"#ff6b6b\"\n// (binary Wasm — can't display the actual bytes)";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "12px";
                            white-space: "pre";
                        }
                        c4_note {
                            text: "This is the first lesson where Wasm is generated. Static content goes to HTML; event handlers go to Wasm.";
                            font-size: "0.85rem";
                            color: "#888";
                            font-style: "italic";
                        }
                    }

                    l4_nav {
                        margin-top: "40px";
                        display: "flex";
                        justify-content: "space-between";

                        l4_prev {
                            text: "← Classes";
                            background: $tile;
                            color: $fg;
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $show_l4: "none";
                                $show_l3: "block";
                            }
                        }
                        l4_next {
                            text: "Putting It Together →";
                            background: $accent;
                            color: "white";
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $show_l4: "none";
                                $show_l5: "block";
                            }
                        }
                    }
                }

                // ═══════════════════════════════════
                //  LESSON 5: Putting It All Together
                // ═══════════════════════════════════
                lesson_5 {
                    display: $show_l5;

                    l5_title {
                        text: "Lesson 5: Putting It All Together";
                        font-size: "1.6rem";
                        font-weight: "700";
                        color: $accent;
                        margin-bottom: "8px";
                    }
                    l5_subtitle {
                        text: "Structure + classes + events in one component";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }
                    l5_intro {
                        text: "Now let's combine everything: nested elements for structure, a class for shared styling, and click handlers for interactivity. This is how real components work in CUI.";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l5_code_label {
                        text: "CUI source:";
                        font-weight: "600";
                        margin-bottom: "8px";
                    }
                    l5_code {
                        text: ".item {\n    display: \"flex\";\n    align-items: \"center\";\n    padding: \"12px\";\n    border-left: \"4px solid #5865f2\";\n    margin-bottom: \"8px\";\n}\n\nitem {\n    checkbox {\n        background: \"#ddd\";\n        cursor: \"pointer\";\n        ?click { background: \"#5865f2\"; }\n    }\n    label { text: \"Learn CUI\"; }\n}\nitem {\n    checkbox {\n        background: \"#ddd\";\n        cursor: \"pointer\";\n        ?click { background: \"#5865f2\"; }\n    }\n    label { text: \"Build something\"; }\n}";
                        background: $code_bg;
                        color: "#e2e8f0";
                        padding: "16px 20px";
                        border-radius: "8px";
                        font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                        font-size: "0.85rem";
                        overflow-x: "auto";
                        white-space: "pre";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l5_demo_label {
                        text: "Live result (click the squares):";
                        font-weight: "600";
                        margin-bottom: "8px";
                    }
                    l5_demo {
                        background: $tile;
                        padding: "24px";
                        border-radius: "8px";
                        margin-bottom: "24px";

                        .item {
                            display: "flex";
                            align-items: "center";
                            padding: "12px";
                            border-left: "4px solid #5865f2";
                            margin-bottom: "8px";
                            border-radius: "4px";
                        }

                        item {
                            checkbox {
                                width: "20px";
                                height: "20px";
                                background: "#ddd";
                                border-radius: "3px";
                                margin-right: "12px";
                                cursor: "pointer";
                                flex-shrink: "0";

                                ?click {
                                    background: "#5865f2";
                                }
                            }
                            label {
                                text: "Learn CUI";
                            }
                        }
                        item {
                            checkbox {
                                width: "20px";
                                height: "20px";
                                background: "#ddd";
                                border-radius: "3px";
                                margin-right: "12px";
                                cursor: "pointer";
                                flex-shrink: "0";

                                ?click {
                                    background: "#5865f2";
                                }
                            }
                            label {
                                text: "Build something";
                            }
                        }
                    }

                    l5_show {}
                    l5_compiled {
                        display: $compiled_5;
                        background: $tile;
                        padding: "16px 20px";
                        border-radius: "8px";
                        margin-top: "16px";

                        c5_html_label {
                            text: "Compiled HTML:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c5_html {
                            text: "<div class=\"f\">\n  <div class=\"g\"></div>\n  <div>Learn CUI</div>\n</div>\n<div class=\"f\">\n  <div class=\"g\"></div>\n  <div>Build something</div>\n</div>";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "16px";
                            white-space: "pre";
                        }
                        c5_css_label {
                            text: "Compiled CSS:";
                            font-weight: "600";
                            font-size: "0.9rem";
                            margin-bottom: "8px";
                        }
                        c5_css {
                            text: ".f { display: flex; align-items: center;\n     padding: 12px; border-left: 4px solid #5865f2; }\n.g { width: 20px; height: 20px;\n     background: #ddd; cursor: pointer; }";
                            background: $code_bg;
                            color: "#e2e8f0";
                            padding: "12px 16px";
                            border-radius: "6px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            margin-bottom: "12px";
                            white-space: "pre";
                        }
                        c5_note {
                            text: "The .item class compiles to one CSS rule. Each checkbox has a Wasm click handler. The labels are static HTML.";
                            font-size: "0.85rem";
                            color: "#888";
                            font-style: "italic";
                        }
                    }

                    l5_nav {
                        margin-top: "40px";
                        display: "flex";
                        justify-content: "space-between";
                        align-items: "center";

                        l5_prev {
                            text: "← Events";
                            background: $tile;
                            color: $fg;
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $show_l5: "none";
                                $show_l4: "block";
                            }
                        }
                    }

                    l5_finish {
                        margin-top: "48px";
                        padding: "32px";
                        background: $tile;
                        border-radius: "10px";
                        text-align: "center";

                        finish_title {
                            text: "That's the core of CUI.";
                            font-size: "1.3rem";
                            font-weight: "700";
                            margin-bottom: "12px";
                        }
                        finish_body {
                            text: "Three block types — instances, classes, and listeners — plus properties and variables. The compiler handles the rest: cascading, scoping, and splitting your code across HTML, CSS, and WebAssembly.";
                            line-height: "1.6";
                            margin-bottom: "20px";
                            opacity: "0.8";
                        }
                        finish_link {
                            link: "https://github.com/thisminute/cascading-ui";
                            text: "Explore on GitHub →";
                            color: $accent;
                            font-weight: "600";
                            text-decoration: "none";
                        }
                    }
                }
            }
        }
    }
}
