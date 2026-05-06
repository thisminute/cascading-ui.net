extern crate cascading_ui;
use cascading_ui::cui;

cui! {
    title: "CUI -- Cascading UI Interactive Tutorial";

    let $bg: "white";
    let $fg: "#1a1a2e";
    let $tile: "#f7f8fa";
    let $code: "#1a1a2e";
    let $accent: "#5865f2";
    let $accent_light: "#e0e7ff";

    /* Current lesson display */
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

    .concept_box {
        background: $accent_light;
        padding: "16px";
        border-radius: "6px";
        margin: "20px 0";
        border-left: "4px solid #5865f2";
    }

    .concept_label {
        font-weight: "600";
        color: $accent;
        margin-bottom: "8px";
    }

    /* ===== NAVIGATION ===== */
    .nav_button {
        background: $accent;
        color: "white";
        padding: "10px 16px";
        border-radius: "6px";
        cursor: "pointer";
        font-weight: "500";
        margin: "4px";
        border: "none";
        font-size: "0.95rem";
    }

    .nav_button_secondary {
        background: $tile;
        color: $fg;
        padding: "10px 16px";
        border-radius: "6px";
        cursor: "pointer";
        font-weight: "500";
        margin: "4px";
        border: "1px solid #ccc";
        font-size: "0.95rem";
    }

    .demo_button {
        background: $accent;
        color: "white";
        padding: "12px 24px";
        border-radius: "6px";
        cursor: "pointer";
        font-weight: "500";
        font-size: "1rem";
        border: "none";
        margin: "8px 4px";
    }

    .demo_button_secondary {
        background: $tile;
        color: $fg;
        padding: "10px 20px";
        border-radius: "6px";
        cursor: "pointer";
        font-weight: "500";
        margin: "8px 4px";
        border: "1px solid #ccc";
        font-size: "0.95rem";
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

            /* ===== COVER PAGE (Lesson 0) ===== */
            cover_page {
                text-align: "center";
                margin-bottom: "40px";

                hero_title {
                    text: "CUI";
                    font-size: "3rem";
                    font-weight: "700";
                    margin-bottom: "16px";
                }

                hero_subtitle {
                    text: "A Compiled Web Language Based on CSS Syntax";
                    font-size: "1.2rem";
                    color: "#888";
                    margin-bottom: "24px";
                }

                hero_desc {
                    text: "Learn through interactive examples. No JavaScript. Everything compiles to HTML and WebAssembly.";
                    font-size: "1rem";
                    margin-bottom: "32px";
                    max-width: "600px";
                    margin-left: "auto";
                    margin-right: "auto";
                }

                start_button {
                    text: "Begin Tutorial →";
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

                nav_example {
                    text: ".button { background: \"blue\"; color: \"white\"; cursor: \"pointer\"; }";
                    background: $code;
                    color: "#e2e8f0";
                    padding: "20px";
                    border-radius: "8px";
                    font-family: "monospace";
                    font-size: "0.85rem";
                    text-align: "left";
                    margin-top: "40px";
                    overflow-x: "auto";
                }
            }

            /* ===== LESSON 1: Text Property ===== */
            lesson_1 {
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
                    text: "Live Demo:";
                    font-weight: "600";
                    margin-top: "24px";
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

                lesson_nav {
                    margin-top: "40px";
                    text-align: "center";

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
                            $lesson: "2";
                        }
                    }
                }
            }

            /* ===== LESSON 2: Elements & Structure ===== */
            lesson_2 {
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
                    text: "Live Demo:";
                    font-weight: "600";
                    margin-top: "24px";
                    margin-bottom: "12px";
                }

                demo_area {
                    .demo_card {
                        background: $tile;
                        padding: "16px";
                        border-radius: "6px";
                        text-align: "left";
                        border-left: "4px solid #5865f2";
                    }

                    card {
                        title {
                            text: "Title";
                            font-weight: "600";
                            color: $accent;
                            margin-bottom: "8px";
                        }

                        description {
                            text: "This is a nested element inside the card.";
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
                    text: "card {\n    title {\n        text: \"Title\";\n    }\n    description {\n        text: \"This is a nested element inside the card.\";\n    }\n}";
                }

                explanation {
                    text: "Nesting creates a DOM hierarchy. Each element becomes a block in the final HTML. The names ('card', 'title', 'description') are semantic for you; the browser gets appropriate HTML tags.";
                    margin-top: "24px";
                    line-height: "1.6";
                }

                lesson_nav {
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
                    text: "Classes (starting with .) define reusable styles. Instances of that class automatically inherit the class properties. Instance properties override class properties—just like CSS cascading.";
                    margin-bottom: "24px";
                    line-height: "1.6";
                }

                demo_label {
                    text: "Live Demo:";
                    font-weight: "600";
                    margin-top: "24px";
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
                        text: "Primary Button";
                        background: $accent;
                    }

                    button_2 {
                        text: "Secondary Button";
                        background: "#666";
                    }

                    button_3 {
                        text: "Custom Override";
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
                    text: ".button_style {\n    padding: \"12px 24px\";\n    border-radius: \"6px\";\n    cursor: \"pointer\";\n    color: \"white\";\n}\n\nbutton_1 { text: \"Primary\"; background: \"blue\"; }\nbutton_2 { text: \"Secondary\"; background: \"gray\"; }\nbutton_3 { text: \"Custom\"; background: \"red\"; }";
                }

                explanation {
                    text: "Each button instance inherits the .button_style properties. But instance properties (like background color) override the class. This is CSS cascading applied to all properties.";
                    margin-top: "24px";
                    line-height: "1.6";
                }

                lesson_nav {
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
                    text: "Use listeners (?click, ?blur, ?focus, etc.) to respond to user actions. Inside a listener, change properties, update text, or create new elements. All of this runs in WebAssembly—no JavaScript needed.";
                    margin-bottom: "24px";
                    line-height: "1.6";
                }

                demo_label {
                    text: "Live Demo:";
                    font-weight: "600";
                    margin-top: "24px";
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
                    text: "let $counter: \"0\";\n\nbutton {\n    text: \"Click me\";\n    background: \"blue\";\n    color: \"white\";\n\n    ?click {\n        $counter: \"1\";\n    }\n\n    ?blur {\n        $counter: \"0\";\n    }\n}";
                }

                explanation {
                    text: "Variables (starting with $) are reactive. When you change them inside a listener, the DOM updates. This code compiles to Wasm and runs entirely in the browser—no server roundtrip.";
                    margin-top: "24px";
                    line-height: "1.6";
                }

                lesson_nav {
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

            /* ===== LESSON 5: Variables & Scope ===== */
            lesson_5 {
                lesson_header {
                    text: "Lesson 5: Variables & Scope";
                    font-size: "2rem";
                    font-weight: "700";
                    margin-bottom: "8px";
                    color: $accent;
                }

                lesson_subtitle {
                    text: "Data flows through classes and instances";
                    font-size: "1rem";
                    color: "#888";
                    font-style: "italic";
                    margin-bottom: "24px";
                }

                concept_intro {
                    text: "Define variables with 'let $name: value'. Variables are available throughout their scope and can be changed in event handlers. Instance variables shadow class variables—just like CSS property inheritance.";
                    margin-bottom: "24px";
                    line-height: "1.6";
                }

                demo_label {
                    text: "Live Demo:";
                    font-weight: "600";
                    margin-top: "24px";
                    margin-bottom: "12px";
                }

                demo_area {
                    let $color1: "#5865f2";
                    let $color2: "#ff6b6b";
                    let $color3: "#ffa500";

                    .badge {
                        padding: "8px 12px";
                        border-radius: "4px";
                        color: "white";
                        font-weight: "500";
                        font-size: "0.9rem";
                        margin: "4px";
                    }

                    b1 {
                        text: "Default";
                        background: $color1;
                    }

                    b2 {
                        text: "Custom";
                        background: $color2;
                    }

                    b3 {
                        text: "Another";
                        background: $color3;
                    }
                }

                code_label {
                    text: "Code:";
                    font-weight: "600";
                    margin-top: "24px";
                    margin-bottom: "12px";
                }

                code {
                    text: "let $color: \"blue\";\n\n.badge {\n    padding: \"8px 12px\";\n    color: \"white\";\n}\n\nbadge { text: \"One\"; background: $color; }\nbadge { text: \"Two\"; background: \"red\"; }";
                }

                explanation {
                    text: "Variables defined with 'let $name: value' can be referenced in properties. Each instance can have its own values. This is like CSS custom properties—data flows down, and instances can override. The compiler bakes static variable values directly into HTML and CSS.";
                    margin-top: "24px";
                    line-height: "1.6";
                }

                lesson_nav {
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

                    next_btn {
                        text: "Final Lesson →";
                        background: $accent;
                        color: "white";
                        padding: "12px 24px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "600";
                        border: "none";

                        ?click {
                            $lesson: "6";
                        }
                    }
                }
            }

            /* ===== LESSON 6: Putting It All Together ===== */
            lesson_6 {
                lesson_header {
                    text: "Lesson 6: Putting It All Together";
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
                    text: "Let's build a todo item component that combines structure, classes, variables, and events. This is a real component that could be used multiple times.";
                    margin-bottom: "24px";
                    line-height: "1.6";
                }

                demo_label {
                    text: "Live Demo:";
                    font-weight: "600";
                    margin-top: "24px";
                    margin-bottom: "12px";
                }

                demo_area {
                    .todo_item {
                        let $done: "false";
                        let $text: "Unchecked";
                        display: "flex";
                        align-items: "center";
                        padding: "12px";
                        margin: "8px 0";
                        background: $tile;
                        border-radius: "6px";
                        border-left: "4px solid #5865f2";
                    }

                    item_1 {
                        let $text: "Learn CUI basics";

                        checkbox {
                            width: "20px";
                            height: "20px";
                            margin-right: "12px";
                            cursor: "pointer";
                            background: "#ddd";
                            border-radius: "3px";

                            ?click {
                                background: $accent;
                                $done: "true";
                            }
                        }

                        label {
                            text: $text;
                            flex: "1";
                            cursor: "pointer";
                        }
                    }

                    item_2 {
                        let $text: "Build a component";

                        checkbox {
                            width: "20px";
                            height: "20px";
                            margin-right: "12px";
                            cursor: "pointer";
                            background: "#ddd";
                            border-radius: "3px";

                            ?click {
                                background: $accent;
                                $done: "true";
                            }
                        }

                        label {
                            text: $text;
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
                    text: ".todo_item {\n    let $done: \"false\";\n    display: \"flex\";\n    align-items: \"center\";\n    padding: \"12px\";\n    background: \"lightgray\";\n}\n\ntodo_item {\n    let $text: \"Learn CUI\";\n\n    checkbox {\n        width: \"20px\";\n        height: \"20px\";\n        cursor: \"pointer\";\n        ?click { $done: \"true\"; }\n    }\n\n    label {\n        text: $text;\n        flex: \"1\";\n    }\n}";
                }

                explanation {
                    text: "This combines: (1) nested structure (item > checkbox + label), (2) class cascading for shared styles, (3) instance variables ($text) for unique content, (4) event handlers for interactivity. Everything compiles to static HTML and Wasm—no framework overhead.";
                    margin-top: "24px";
                    line-height: "1.6";
                }

                lesson_nav {
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
                            $lesson: "5";
                        }
                    }

                    back_to_cover {
                        text: "Back to Start";
                        background: $accent;
                        color: "white";
                        padding: "12px 24px";
                        border-radius: "6px";
                        cursor: "pointer";
                        font-weight: "600";
                        border: "none";

                        ?click {
                            $lesson: "0";
                        }
                    }
                }
            }

            /* ===== FOOTER ===== */
            footer {
                text: "This tutorial is built with CUI. Every interactive element—buttons, counters, todos—runs as WebAssembly compiled at build time.";
                margin-top: "80px";
                padding-top: "24px";
                border-top: "1px solid #ccc";
                text-align: "center";
                font-size: "0.9rem";
                color: "#888";
            }
        }
    }
}
