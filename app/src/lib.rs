extern crate cascading_ui;
use cascading_ui::cui;

cui! {
    title: "CUI -- Cascading UI";

    let $bg: "white";
    let $fg: "#1a1a2e";
    let $tile: "#f7f8fa";
    let $code: "#1a1a2e";

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
            apply: .light_mode_button;
        }
    }

    page {
        background: $bg;
        color: $fg;
        min-height: "100vh";

        toggle {
            light_mode_button {}
        }

        content {
            hero {
                hero_title {
                    text: "CUI";
                }
                hero_subtitle {
                    text: "A web language based on CSS syntax that compiles to HTML + Wasm. No JavaScript.";
                }
                hero_example {
                    text: ".button {\n    background: \"blue\";\n    color: \"white\";\n    cursor: \"pointer\";\n}\n\npage {\n    button {\n        text: \"click me\";\n        ?click {\n            text: \"clicked!\";\n        }\n    }\n}";
                }
            }

            section {
                section_title {
                    text: "What is CUI?";
                }
                description {
                    text: "CUI is a compiled language where structure, style, and behavior live in one CSS-like syntax. Classes define how things look. Instances create them. Listeners handle events. The compiler figures out the rest -- what's static gets baked into HTML, what's dynamic gets compiled to WebAssembly.";
                }
            }

            section {
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

            section {
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
                        ?click {
                            $label: "Clicked! This ran through Wasm.";
                        }
                    }
                }
            }

            section {
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

            section {
                section_title {
                    text: "Get started";
                }
                github_link {
                    font-weight: "500";
                    link: "https://github.com/thisminute/cascading-ui";
                    text: "github.com/thisminute/cascading-ui";
                }
            }

            footer {
                text: "Built with CUI.";
            }
        }
    }
}
