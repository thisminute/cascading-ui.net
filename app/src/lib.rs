extern crate cascading_ui;
use cascading_ui::cui;

cui! {
    title: "CUI -- Cascading UI";

    let $bg: "white";
    let $fg: "#1a1a2e";
    let $tile: "#f7f8fa";
    let $code: "#1a1a2e";
    let $accent: "#5865f2";
    let $accent_light: "#e0e7ff";

    /* Current page: "home" or "tutorial" */
    let $page: "home";

    /* Tutorial lesson number */
    let $lesson: "0";

    /* ===== THEME TOGGLE ===== */
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
            $code: "#131325";
            $accent_light: "#3a3f5f";
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
            $code: "#1a1a2e";
            $accent_light: "#e0e7ff";
            apply: .light_mode_button;
        }
    }

    /* ===== REUSABLE STYLES ===== */
    .section_title {
        font-size: "1.8rem";
        font-weight: "600";
        margin-bottom: "16px";
        color: $fg;
    }

    .description {
        line-height: "1.6";
        color: $fg;
        margin-bottom: "16px";
    }

    .code_block {
        background: $code;
        color: "#e2e8f0";
        padding: "16px";
        border-radius: "8px";
        margin: "20px 0";
        overflow-x: "auto";
        font-family: "monospace";
        font-size: "0.85rem";
        border-left: "4px solid #5865f2";
    }

    .demo_box {
        background: $tile;
        padding: "24px";
        border-radius: "8px";
        margin: "20px 0";
        border: "1px solid #ddd";
        text-align: "center";
    }

    .point {
        margin-bottom: "24px";
    }

    .point_title {
        font-weight: "600";
        font-size: "1.1rem";
        margin-bottom: "8px";
        color: $fg;
    }

    .point_body {
        color: $fg;
        line-height: "1.6";
    }

    page {
        background: $bg;
        color: $fg;
        min-height: "100vh";
        transition: "background-color 0.3s";
        font-family: "system-ui, -apple-system, sans-serif";
        line-height: "1.7";

        theme_toggle {
            light_mode_button {}
        }

        main_content {
            max-width: "900px";
            margin: "0 auto";
            padding: "40px 20px";

            /* ===== HOME PAGE ===== */
            home_page {
                text-align: "center";
                margin-bottom: "40px";

                hero_title {
                    text: "CUI";
                    font-size: "3rem";
                    font-weight: "700";
                    margin-bottom: "16px";
                }

                hero_subtitle {
                    text: "A web language based on CSS syntax that compiles to HTML/CSS/Wasm. No JavaScript.";
                    font-size: "1.2rem";
                    color: "#666";
                    margin-bottom: "24px";
                }

                hero_example {
                    text: ".button {\n    background: \"blue\";\n    color: \"white\";\n    cursor: \"pointer\";\n}\n\npage {\n    button {\n        text: \"click me\";\n        ?click {\n            text: \"clicked!\";\n        }\n    }\n}";
                    background: $code;
                    color: "#e2e8f0";
                    padding: "20px";
                    border-radius: "8px";
                    font-family: "monospace";
                    font-size: "0.85rem";
                    text-align: "left";
                    overflow-x: "auto";
                    margin-bottom: "24px";
                }

                tutorial_link {
                    text: "Learn CUI with Interactive Tutorial →";
                    background: $accent;
                    color: "white";
                    padding: "12px 24px";
                    border-radius: "6px";
                    cursor: "pointer";
                    font-weight: "600";
                    font-size: "1rem";
                    border: "none";

                    ?click {
                        $page: "tutorial";
                        $lesson: "0";
                    }
                }
            }

            section_1 {
                section_title {
                    text: "What is CUI?";
                }
                description {
                    text: "CUI is a compiled language where structure, style, and behavior live in one CSS-like syntax. Classes define how things look. Instances create them. Listeners handle events. The compiler figures out the rest -- what's static gets baked into HTML, what's dynamic gets compiled to WebAssembly.";
                }
            }

            section_2 {
                section_title {
                    text: "Why CUI?";
                }

                point {
                    point_title {
                        text: "Zero JavaScript";
                    }
                    point_body {
                        text: "CUI compiles to WebAssembly. No bundler, no transpiler, no node_modules. Event handlers and reactivity run as native Wasm.";
                    }
                }

                point {
                    point_title {
                        text: "Three-layer compilation";
                    }
                    point_body {
                        text: "The compiler detects what's fully static (baked into HTML), what needs one-time setup (wired up at page load), and what's truly reactive. Static pages have zero runtime cost.";
                    }
                }

                point {
                    point_title {
                        text: "CSS semantics you already know";
                    }
                    point_body {
                        text: "Classes cascade. Properties inherit. If you know CSS, you know the mental model. CUI extends it to structure and behavior.";
                    }
                }

                point {
                    point_title {
                        text: "Three block types, that's it";
                    }
                    point_body {
                        text: "Instances create elements. Classes define their appearance. Listeners handle events. Everything else is a property. No components, no hooks, no lifecycle methods.";
                    }
                }
            }

            section_3 {
                section_title {
                    text: "Try it";
                }
                description {
                    text: "This page is built with CUI. The button below is a live Wasm element:";
                }
                demo_area {
                    demo_button {
                        let $label: "Click me";
                        text: $label;
                        background: $accent;
                        color: "white";
                        padding: "12px 24px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "600";
                        border: "none";

                        ?click {
                            $label: "Clicked! This ran through Wasm.";
                        }
                    }
                }
            }

            section_4 {
                section_title {
                    text: "How it works";
                }
                description {
                    text: "CUI is a Rust procedural macro. Your source is parsed, analyzed, and compiled at build time:";
                }
                code_block {
                    text: "CUI source\n  -> parse\n  -> AST\n  -> analyze\n  -> semantics tree\n  -> cascade classes into instances\n  -> compile\n  -> HTML + CSS + Wasm";
                }
                description {
                    text: "The cascade phase resolves class inheritance and variable scoping, and assigns each piece of content to a compilation layer. Only reactive parts incur runtime cost.";
                }
            }

            section_5 {
                section_title {
                    text: "Get started";
                }
                github_link {
                    color: $accent;
                    font-weight: "500";
                    link: "https://github.com/thisminute/cascading-ui";
                    text: "github.com/thisminute/cascading-ui";
                }
            }

            footer {
                text: "Built with CUI.";
                margin-top: "60px";
                padding-top: "20px";
                border-top: "1px solid #ccc";
                text-align: "center";
                color: "#999";
                font-size: "0.9rem";
            }

            /* ===== TUTORIAL PAGE ===== */
            tutorial_page {
                /* ===== LESSON 0: Cover ===== */
                cover_page {
                    text-align: "center";
                    margin-bottom: "40px";

                    back_btn {
                        text: "← Back to Home";
                        background: $tile;
                        color: $fg;
                        padding: "10px 16px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "1px solid #ccc";
                        margin-bottom: "32px";

                        ?click {
                            $page: "home";
                        }
                    }

                    hero_title {
                        text: "CUI Interactive Tutorial";
                        font-size: "2.5rem";
                        font-weight: "700";
                        margin-bottom: "16px";
                    }

                    hero_subtitle {
                        text: "Learn through working examples";
                        font-size: "1.1rem";
                        color: "#888";
                        margin-bottom: "32px";
                    }

                    start_button {
                        text: "Begin Lesson 1 →";
                        background: $accent;
                        color: "white";
                        padding: "16px 32px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "600";
                        font-size: "1.1rem";
                        border: "none";

                        ?click {
                            $lesson: "1";
                        }
                    }
                }

                /* ===== LESSON 1: Text Property ===== */
                lesson_1 {
                    let $expanded_1: "false";

                    back_btn {
                        text: "← Back to Home";
                        background: $tile;
                        color: $fg;
                        padding: "10px 16px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "1px solid #ccc";
                        margin-bottom: "24px";

                        ?click {
                            $page: "home";
                        }
                    }

                    lesson_header {
                        text: "Lesson 1: The Text Property";
                        font-size: "2rem";
                        font-weight: "700";
                        margin-bottom: "8px";
                        color: $accent;
                    }

                    lesson_subtitle {
                        text: "The simplest thing: putting words on the page";
                        font-size: "1rem";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }

                    concept_intro {
                        text: "Every element can have a 'text' property. It's the most basic way to put content on the page. This is static content—the compiler bakes it into HTML at build time.";
                        margin-bottom: "24px";
                        line-height: "1.6";
                    }

                    demo_label {
                        text: "Demo:";
                        font-weight: "600";
                        margin-bottom: "12px";
                    }

                    demo_area {
                        demo_text {
                            text: "Hello, CUI!";
                            font-size: "1.4rem";
                            font-weight: "500";
                            color: $accent;
                        }
                    }

                    code_label {
                        text: "Code:";
                        font-weight: "600";
                        margin-top: "24px";
                        margin-bottom: "12px";
                    }

                    code {
                        text: "element {\n    text: \"Hello, CUI!\";\n}";
                    }

                    explanation {
                        text: "That's it! Every element can display text. The compiler detects that this is static and bakes it directly into HTML. No runtime cost.";
                        margin-top: "24px";
                        line-height: "1.6";
                    }

                    expand_btn {
                        text: "▶ See compiled result";
                        background: "transparent";
                        color: $accent;
                        padding: "8px 0";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "none";
                        margin-top: "24px";
                        text-align: "left";

                        ?click {
                            $expanded_1: "true";
                        }
                    }

                    compiled {
                        collapse_btn {
                            text: "▼ Hide compiled result";
                            background: "transparent";
                            color: $accent;
                            padding: "8px 0";
                            cursor: "pointer";
                            font-weight: "500";
                            border: "none";
                            text-align: "left";

                            ?click {
                                $expanded_1: "false";
                            }
                        }

                        compiled_html {
                            text: "Generated HTML:";
                            font-weight: "600";
                            margin-top: "16px";
                            margin-bottom: "8px";
                            font-size: "0.95rem";
                        }

                        html_code {
                            text: "<div class=\"a\">Hello, CUI!</div>";
                            background: $code;
                            color: "#e2e8f0";
                            padding: "12px";
                            border-radius: "6px";
                            margin-bottom: "16px";
                            overflow-x: "auto";
                            font-family: "monospace";
                            font-size: "0.85rem";
                        }

                        compiled_css {
                            text: "Generated CSS (selector a):";
                            font-weight: "600";
                            margin-bottom: "8px";
                            font-size: "0.95rem";
                        }

                        css_code {
                            text: ".a { /* static styles would appear here */ }";
                            background: $code;
                            color: "#e2e8f0";
                            padding: "12px";
                            border-radius: "6px";
                            margin-bottom: "12px";
                            overflow-x: "auto";
                            font-family: "monospace";
                            font-size: "0.85rem";
                        }

                        note {
                            text: "The compiler generates unique selectors (a, b, c...) and bakes the HTML string directly into the HTML file. Static content = zero runtime overhead.";
                            font-size: "0.9rem";
                            color: "#666";
                            font-style: "italic";
                            margin-top: "12px";
                        }
                    }

                    nav {
                        margin-top: "40px";
                        text-align: "center";

                        next_btn {
                            text: "Next Lesson →";
                            background: $accent;
                            color: "white";
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $lesson: "2";
                            }
                        }
                    }
                }

                /* ===== LESSON 2: Elements & Structure ===== */
                lesson_2 {
                    let $expanded_2: "false";

                    back_btn {
                        text: "← Back to Home";
                        background: $tile;
                        color: $fg;
                        padding: "10px 16px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "1px solid #ccc";
                        margin-bottom: "24px";

                        ?click {
                            $page: "home";
                        }
                    }

                    lesson_header {
                        text: "Lesson 2: Elements & Structure";
                        font-size: "2rem";
                        font-weight: "700";
                        margin-bottom: "8px";
                        color: $accent;
                    }

                    lesson_subtitle {
                        text: "Building hierarchy without thinking about tags";
                        font-size: "1rem";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }

                    concept_intro {
                        text: "Elements in CUI are just names. The compiler figures out which HTML tag to use. You define the structure through nesting, and CUI creates the DOM hierarchy automatically.";
                        margin-bottom: "24px";
                        line-height: "1.6";
                    }

                    demo_label {
                        text: "Demo:";
                        font-weight: "600";
                        margin-bottom: "12px";
                    }

                    demo_area {
                        card {
                            background: $tile;
                            padding: "16px";
                            border-radius: "6px";
                            text-align: "left";
                            border-left: "4px solid #5865f2";

                            title {
                                text: "Nested Content";
                                font-weight: "600";
                                color: $accent;
                                margin-bottom: "8px";
                            }

                            body {
                                text: "This demonstrates nesting—a title and body inside a card.";
                                color: "#666";
                            }
                        }
                    }

                    code_label {
                        text: "Code:";
                        font-weight: "600";
                        margin-top: "24px";
                        margin-bottom: "12px";
                    }

                    code {
                        text: "card {\n    title {\n        text: \"Title\";\n    }\n    body {\n        text: \"Content\";\n    }\n}";
                    }

                    explanation {
                        text: "Nesting creates a DOM hierarchy. Each element becomes a block in the final HTML. Names like 'card', 'title', 'body' are semantic for you; the browser gets appropriate tags.";
                        margin-top: "24px";
                        line-height: "1.6";
                    }

                    expand_btn {
                        text: "▶ See compiled result";
                        background: "transparent";
                        color: $accent;
                        padding: "8px 0";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "none";
                        margin-top: "24px";
                        text-align: "left";

                        ?click {
                            $expanded_2: "true";
                        }
                    }

                    compiled {
                        collapse_btn {
                            text: "▼ Hide compiled result";
                            background: "transparent";
                            color: $accent;
                            padding: "8px 0";
                            cursor: "pointer";
                            font-weight: "500";
                            border: "none";
                            text-align: "left";

                            ?click {
                                $expanded_2: "false";
                            }
                        }

                        compiled_html {
                            text: "Generated HTML (with nesting):";
                            font-weight: "600";
                            margin-top: "16px";
                            margin-bottom: "8px";
                            font-size: "0.95rem";
                        }

                        html_code {
                            text: "<div class=\"b\">\n  <div class=\"c\">Nested Content</div>\n  <div class=\"d\">This demonstrates...</div>\n</div>";
                            background: $code;
                            color: "#e2e8f0";
                            padding: "12px";
                            border-radius: "6px";
                            margin-bottom: "16px";
                            overflow-x: "auto";
                            font-family: "monospace";
                            font-size: "0.85rem";
                        }

                        compiled_css {
                            text: "Generated CSS (selectors b, c, d):";
                            font-weight: "600";
                            margin-bottom: "8px";
                            font-size: "0.95rem";
                        }

                        css_code {
                            text: ".b { /* card styles */ }\n.c { /* title styles */ }\n.d { /* body styles */ }";
                            background: $code;
                            color: "#e2e8f0";
                            padding: "12px";
                            border-radius: "6px";
                            margin-bottom: "12px";
                            overflow-x: "auto";
                            font-family: "monospace";
                            font-size: "0.85rem";
                        }

                        note {
                            text: "Each nesting level gets its own CSS selector (a-z, then AA-AZ, etc.). The HTML preserves the DOM structure. No extra wrapper divs or shadow DOM.";
                            font-size: "0.9rem";
                            color: "#666";
                            font-style: "italic";
                            margin-top: "12px";
                        }
                    }

                    nav {
                        margin-top: "40px";
                        text-align: "center";

                        prev_btn {
                            text: "← Previous";
                            background: $tile;
                            color: $fg;
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "1px solid #ccc";
                            margin-right: "12px";

                            ?click {
                                $lesson: "1";
                            }
                        }

                        next_btn {
                            text: "Next →";
                            background: $accent;
                            color: "white";
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $lesson: "3";
                            }
                        }
                    }
                }

                /* ===== LESSON 3: Classes & Cascading ===== */
                lesson_3 {
                    let $expanded_3: "false";

                    back_btn {
                        text: "← Back to Home";
                        background: $tile;
                        color: $fg;
                        padding: "10px 16px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "1px solid #ccc";
                        margin-bottom: "24px";

                        ?click {
                            $page: "home";
                        }
                    }

                    lesson_header {
                        text: "Lesson 3: Classes & Cascading";
                        font-size: "2rem";
                        font-weight: "700";
                        margin-bottom: "8px";
                        color: $accent;
                    }

                    lesson_subtitle {
                        text: "Define once, reuse everywhere";
                        font-size: "1rem";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }

                    concept_intro {
                        text: "Classes (starting with .) define reusable styles. Instances automatically inherit class properties. Instance properties override class properties—just like CSS cascading.";
                        margin-bottom: "24px";
                        line-height: "1.6";
                    }

                    demo_label {
                        text: "Demo:";
                        font-weight: "600";
                        margin-bottom: "12px";
                    }

                    demo_area {
                        .button_style {
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "500";
                            border: "none";
                            margin: "4px";
                            color: "white";
                        }

                        button_1 {
                            text: "Primary";
                            background: $accent;
                        }

                        button_2 {
                            text: "Secondary";
                            background: "#666";
                        }

                        button_3 {
                            text: "Custom";
                            background: "#ff6b6b";
                        }
                    }

                    code_label {
                        text: "Code:";
                        font-weight: "600";
                        margin-top: "24px";
                        margin-bottom: "12px";
                    }

                    code {
                        text: ".button {\n    padding: \"12px 24px\";\n    cursor: \"pointer\";\n    color: \"white\";\n}\n\nbutton { text: \"One\"; background: \"blue\"; }\nbutton { text: \"Two\"; background: \"gray\"; }";
                    }

                    explanation {
                        text: "Each button inherits .button properties. But instance properties (like background) override the class. This is CSS cascading applied to all properties.";
                        margin-top: "24px";
                        line-height: "1.6";
                    }

                    expand_btn {
                        text: "▶ See compiled result";
                        background: "transparent";
                        color: $accent;
                        padding: "8px 0";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "none";
                        margin-top: "24px";
                        text-align: "left";

                        ?click {
                            $expanded_3: "true";
                        }
                    }

                    compiled {
                        collapse_btn {
                            text: "▼ Hide compiled result";
                            background: "transparent";
                            color: $accent;
                            padding: "8px 0";
                            cursor: "pointer";
                            font-weight: "500";
                            border: "none";
                            text-align: "left";

                            ?click {
                                $expanded_3: "false";
                            }
                        }

                        compiled_html {
                            text: "Generated HTML (all buttons share class):";
                            font-weight: "600";
                            margin-top: "16px";
                            margin-bottom: "8px";
                            font-size: "0.95rem";
                        }

                        html_code {
                            text: "<button class=\"e\">Primary</button>\n<button class=\"e\">Secondary</button>\n<button class=\"e\">Custom</button>";
                            background: $code;
                            color: "#e2e8f0";
                            padding: "12px";
                            border-radius: "6px";
                            margin-bottom: "16px";
                            overflow-x: "auto";
                            font-family: "monospace";
                            font-size: "0.85rem";
                        }

                        compiled_css {
                            text: "Generated CSS (one class, inline overrides):";
                            font-weight: "600";
                            margin-bottom: "8px";
                            font-size: "0.95rem";
                        }

                        css_code {
                            text: ".e { padding: 12px; cursor: pointer; color: white; }\n/* button 1: background blue (inline) */\n/* button 2: background gray (inline) */\n/* button 3: background red (inline) */";
                            background: $code;
                            color: "#e2e8f0";
                            padding: "12px";
                            border-radius: "6px";
                            margin-bottom: "12px";
                            overflow-x: "auto";
                            font-family: "monospace";
                            font-size: "0.85rem";
                        }

                        note {
                            text: "The .button_style class is compiled once. Each instance applies the class and adds instance-specific styles (like background color) inline or as additional rules. This minimizes CSS size.";
                            font-size: "0.9rem";
                            color: "#666";
                            font-style: "italic";
                            margin-top: "12px";
                        }
                    }

                    nav {
                        margin-top: "40px";
                        text-align: "center";

                        prev_btn {
                            text: "← Previous";
                            background: $tile;
                            color: $fg;
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "1px solid #ccc";
                            margin-right: "12px";

                            ?click {
                                $lesson: "2";
                            }
                        }

                        next_btn {
                            text: "Next →";
                            background: $accent;
                            color: "white";
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $lesson: "4";
                            }
                        }
                    }
                }

                /* ===== LESSON 4: Events & Interactivity ===== */
                lesson_4 {
                    let $expanded_4: "false";

                    back_btn {
                        text: "← Back to Home";
                        background: $tile;
                        color: $fg;
                        padding: "10px 16px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "1px solid #ccc";
                        margin-bottom: "24px";

                        ?click {
                            $page: "home";
                        }
                    }

                    lesson_header {
                        text: "Lesson 4: Events & Interactivity";
                        font-size: "2rem";
                        font-weight: "700";
                        margin-bottom: "8px";
                        color: $accent;
                    }

                    lesson_subtitle {
                        text: "Making things respond to user actions";
                        font-size: "1rem";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }

                    concept_intro {
                        text: "Use listeners (?click, ?blur, ?focus, etc.) to respond to user actions. Inside a listener, change properties or create new elements. All of this runs in WebAssembly—no JavaScript.";
                        margin-bottom: "24px";
                        line-height: "1.6";
                    }

                    demo_label {
                        text: "Demo:";
                        font-weight: "600";
                        margin-bottom: "12px";
                    }

                    demo_area {
                        let $counter: "0";

                        counter_display {
                            text: $counter;
                            font-size: "2rem";
                            font-weight: "700";
                            color: $accent;
                            margin-bottom: "16px";
                        }

                        increment_btn {
                            text: "Increment";
                            background: $accent;
                            color: "white";
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";
                            margin: "4px";

                            ?click {
                                $counter: "1";
                            }
                        }

                        reset_btn {
                            text: "Reset";
                            background: $tile;
                            color: $fg;
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "1px solid #ccc";
                            margin: "4px";

                            ?click {
                                $counter: "0";
                            }
                        }
                    }

                    code_label {
                        text: "Code:";
                        font-weight: "600";
                        margin-top: "24px";
                        margin-bottom: "12px";
                    }

                    code {
                        text: "let $count: \"0\";\n\nbutton {\n    text: $count;\n    ?click { $count: \"1\"; }\n}";
                    }

                    explanation {
                        text: "Variables (starting with $) are reactive. When you change them inside a listener, the DOM updates. This code compiles to Wasm and runs entirely in the browser.";
                        margin-top: "24px";
                        line-height: "1.6";
                    }

                    expand_btn {
                        text: "▶ See compiled result";
                        background: "transparent";
                        color: $accent;
                        padding: "8px 0";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "none";
                        margin-top: "24px";
                        text-align: "left";

                        ?click {
                            $expanded_4: "true";
                        }
                    }

                    compiled {
                        collapse_btn {
                            text: "▼ Hide compiled result";
                            background: "transparent";
                            color: $accent;
                            padding: "8px 0";
                            cursor: "pointer";
                            font-weight: "500";
                            border: "none";
                            text-align: "left";

                            ?click {
                                $expanded_4: "false";
                            }
                        }

                        compiled_html {
                            text: "Generated HTML (interactive button):";
                            font-weight: "600";
                            margin-top: "16px";
                            margin-bottom: "8px";
                            font-size: "0.95rem";
                        }

                        html_code {
                            text: "<div class=\"h\">\n  <div class=\"i\">0</div>\n  <button class=\"j\">Increment</button>\n  <button class=\"k\">Reset</button>\n</div>";
                            background: $code;
                            color: "#e2e8f0";
                            padding: "12px";
                            border-radius: "6px";
                            margin-bottom: "16px";
                            overflow-x: "auto";
                            font-family: "monospace";
                            font-size: "0.85rem";
                        }

                        compiled_css {
                            text: "Generated CSS (minimal):";
                            font-weight: "600";
                            margin-bottom: "8px";
                            font-size: "0.95rem";
                        }

                        css_code {
                            text: ".h { /* container styles */ }\n.i { /* counter display */ }\n.j { /* increment button */ }\n.k { /* reset button */ }";
                            background: $code;
                            color: "#e2e8f0";
                            padding: "12px";
                            border-radius: "6px";
                            margin-bottom: "12px";
                            overflow-x: "auto";
                            font-family: "monospace";
                            font-size: "0.85rem";
                        }

                        note {
                            text: "The event handlers (?click) and reactive variables are compiled into the Wasm binary. The HTML and CSS are static. When you click, Wasm updates the DOM. No JavaScript anywhere.";
                            font-size: "0.9rem";
                            color: "#666";
                            font-style: "italic";
                            margin-top: "12px";
                        }
                    }

                    nav {
                        margin-top: "40px";
                        text-align: "center";

                        prev_btn {
                            text: "← Previous";
                            background: $tile;
                            color: $fg;
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "1px solid #ccc";
                            margin-right: "12px";

                            ?click {
                                $lesson: "3";
                            }
                        }

                        next_btn {
                            text: "Next →";
                            background: $accent;
                            color: "white";
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $lesson: "5";
                            }
                        }
                    }
                }

                /* ===== LESSON 5: Complete Example ===== */
                lesson_5 {
                    let $expanded_5: "false";

                    back_btn {
                        text: "← Back to Home";
                        background: $tile;
                        color: $fg;
                        padding: "10px 16px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "1px solid #ccc";
                        margin-bottom: "24px";

                        ?click {
                            $page: "home";
                        }
                    }

                    lesson_header {
                        text: "Lesson 5: Putting It All Together";
                        font-size: "2rem";
                        font-weight: "700";
                        margin-bottom: "8px";
                        color: $accent;
                    }

                    lesson_subtitle {
                        text: "A complete example that uses everything";
                        font-size: "1rem";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }

                    concept_intro {
                        text: "Let's build a todo component combining structure, classes, and events. This demonstrates how to build real UI components in CUI.";
                        margin-bottom: "24px";
                        line-height: "1.6";
                    }

                    demo_label {
                        text: "Demo:";
                        font-weight: "600";
                        margin-bottom: "12px";
                    }

                    demo_area {
                        .todo_item {
                            display: "flex";
                            align-items: "center";
                            padding: "12px";
                            margin: "8px 0";
                            background: $tile;
                            border-radius: "6px";
                            border-left: "4px solid #5865f2";
                        }

                        item_1 {
                            checkbox {
                                width: "20px";
                                height: "20px";
                                margin-right: "12px";
                                cursor: "pointer";
                                background: "#ddd";
                                border-radius: "3px";

                                ?click {
                                    background: $accent;
                                }
                            }

                            label {
                                text: "Learn CUI basics";
                                flex: "1";
                                cursor: "pointer";
                            }
                        }

                        item_2 {
                            checkbox {
                                width: "20px";
                                height: "20px";
                                margin-right: "12px";
                                cursor: "pointer";
                                background: "#ddd";
                                border-radius: "3px";

                                ?click {
                                    background: $accent;
                                }
                            }

                            label {
                                text: "Build a component";
                                flex: "1";
                                cursor: "pointer";
                            }
                        }
                    }

                    code_label {
                        text: "Code:";
                        font-weight: "600";
                        margin-top: "24px";
                        margin-bottom: "12px";
                    }

                    code {
                        text: ".todo_item {\n    display: \"flex\";\n    padding: \"12px\";\n}\n\ntodo_item {\n    checkbox {\n        ?click { /* change style */ }\n    }\n    label { text: \"Item\"; }\n}";
                    }

                    explanation {
                        text: "This combines: (1) nested structure, (2) class cascading for shared styles, (3) event handlers for interactivity. Everything compiles to static HTML and Wasm.";
                        margin-top: "24px";
                        line-height: "1.6";
                    }

                    expand_btn {
                        text: "▶ See compiled result";
                        background: "transparent";
                        color: $accent;
                        padding: "8px 0";
                        cursor: "pointer";
                        font-weight: "500";
                        border: "none";
                        margin-top: "24px";
                        text-align: "left";

                        ?click {
                            $expanded_5: "true";
                        }
                    }

                    compiled {
                        collapse_btn {
                            text: "▼ Hide compiled result";
                            background: "transparent";
                            color: $accent;
                            padding: "8px 0";
                            cursor: "pointer";
                            font-weight: "500";
                            border: "none";
                            text-align: "left";

                            ?click {
                                $expanded_5: "false";
                            }
                        }

                        compiled_html {
                            text: "Generated HTML (composed structure):";
                            font-weight: "600";
                            margin-top: "16px";
                            margin-bottom: "8px";
                            font-size: "0.95rem";
                        }

                        html_code {
                            text: "<div class=\"o p\">\n  <div class=\"q\"></div>\n  <div class=\"r\">Learn CUI basics</div>\n</div>\n<div class=\"o p\">\n  <div class=\"q\"></div>\n  <div class=\"r\">Build a component</div>\n</div>";
                            background: $code;
                            color: "#e2e8f0";
                            padding: "12px";
                            border-radius: "6px";
                            margin-bottom: "16px";
                            overflow-x: "auto";
                            font-family: "monospace";
                            font-size: "0.85rem";
                        }

                        compiled_css {
                            text: "Generated CSS (reusable class):";
                            font-weight: "600";
                            margin-bottom: "8px";
                            font-size: "0.95rem";
                        }

                        css_code {
                            text: ".o { display: flex; padding: 12px; }\n.p { border-left: 4px solid blue; }\n.q { width: 20px; height: 20px; }\n.r { flex: 1; }";
                            background: $code;
                            color: "#e2e8f0";
                            padding: "12px";
                            border-radius: "6px";
                            margin-bottom: "12px";
                            overflow-x: "auto";
                            font-family: "monospace";
                            font-size: "0.85rem";
                        }

                        note {
                            text: "The .todo_item class is defined once. Each instance (item_1, item_2) applies the class and adds specific content. Click handlers go into Wasm. This is efficient reuse: minimal HTML, shared CSS.";
                            font-size: "0.9rem";
                            color: "#666";
                            font-style: "italic";
                            margin-top: "12px";
                        }
                    }

                    nav {
                        margin-top: "40px";
                        text-align: "center";

                        prev_btn {
                            text: "← Previous";
                            background: $tile;
                            color: $fg;
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "1px solid #ccc";
                            margin-right: "12px";

                            ?click {
                                $lesson: "4";
                            }
                        }

                        back_home {
                            text: "Back to Home";
                            background: $accent;
                            color: "white";
                            padding: "12px 24px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $page: "home";
                            }
                        }
                    }
                }
            }
        }
    }
}
