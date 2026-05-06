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
    let $show_l6: "none";

    // ── Sidebar active state ──
    let $fw_o: "700";
    let $fw_1: "400";
    let $fw_2: "400";
    let $fw_3: "400";
    let $fw_4: "400";
    let $fw_5: "400";
    let $fw_6: "400";

    // ── Code viewer tab state (CUI vs compiled HTML) ──
    let $cui_0: "block";
    let $html_0: "none";
    let $cui_1: "block";
    let $html_1: "none";
    let $cui_2: "block";
    let $html_2: "none";
    let $cui_3: "block";
    let $html_3: "none";
    let $cui_4: "block";
    let $html_4: "none";
    let $cui_5: "block";
    let $html_5: "none";
    let $cui_6: "block";
    let $html_6: "none";

    // ── Per-code-block tab toggle classes (apply pattern) ──
    .h_to_html { text: "HTML →"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_0: "none"; $html_0: "block"; apply: .h_to_cui; } }
    .h_to_cui { text: "← CUI"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_0: "block"; $html_0: "none"; apply: .h_to_html; } }

    .l1_to_html { text: "HTML →"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_1: "none"; $html_1: "block"; apply: .l1_to_cui; } }
    .l1_to_cui { text: "← CUI"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_1: "block"; $html_1: "none"; apply: .l1_to_html; } }

    .l2_to_html { text: "HTML →"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_2: "none"; $html_2: "block"; apply: .l2_to_cui; } }
    .l2_to_cui { text: "← CUI"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_2: "block"; $html_2: "none"; apply: .l2_to_html; } }

    .l3_to_html { text: "HTML →"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_3: "none"; $html_3: "block"; apply: .l3_to_cui; } }
    .l3_to_cui { text: "← CUI"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_3: "block"; $html_3: "none"; apply: .l3_to_html; } }

    .l4_to_html { text: "HTML →"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_4: "none"; $html_4: "block"; apply: .l4_to_cui; } }
    .l4_to_cui { text: "← CUI"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_4: "block"; $html_4: "none"; apply: .l4_to_html; } }

    .l5_to_html { text: "HTML →"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_5: "none"; $html_5: "block"; apply: .l5_to_cui; } }
    .l5_to_cui { text: "← CUI"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_5: "block"; $html_5: "none"; apply: .l5_to_html; } }

    .l6_to_html { text: "HTML →"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_6: "none"; $html_6: "block"; apply: .l6_to_cui; } }
    .l6_to_cui { text: "← CUI"; cursor: "pointer"; color: "#94a3b8"; background: "transparent"; border: "none"; padding: "4px 10px"; font-size: "0.8rem"; font-family: "ui-monospace, SFMono-Regular, Menlo, monospace"; ?click { $cui_6: "block"; $html_6: "none"; apply: .l6_to_html; } }

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
                hero_code_wrap {
                    background: $code_bg;
                    border-radius: "10px 10px 0 0";
                    text-align: "left";
                    position: "relative";

                    hero_tab {
                        position: "absolute";
                        top: "8px";
                        right: "12px";
                        h_to_html {}
                    }

                    hero_cui {
                        display: $cui_0;
                        text: ".button {\n    background: \"blue\";\n    color: \"white\";\n    cursor: \"pointer\";\n}\n\npage {\n    button {\n        text: \"click me\";\n        ?click {\n            text: \"clicked!\";\n        }\n    }\n}";
                        color: "#e2e8f0";
                        padding: "24px 28px";
                        font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                        font-size: "0.85rem";
                        overflow-x: "auto";
                        line-height: "1.6";
                        white-space: "pre";
                    }
                    hero_html {
                        display: $html_0;
                        text: "&lt;div style='display:inline-block;\n  border-radius:4px;padding:8px 16px;\n  border:none;font-size:0.9rem;'\n  class='g'&gt;click me&lt;/div&gt;\n\n/* CSS (from .button class) */\n.g { color:white; cursor:pointer;\n     background:blue; }\n\n/* Wasm: click updates textContent */";
                        color: "#e2e8f0";
                        padding: "24px 28px";
                        font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                        font-size: "0.85rem";
                        overflow-x: "auto";
                        line-height: "1.6";
                        white-space: "pre";
                    }
                }
                hero_demo {
                    background: $tile;
                    padding: "20px 28px";
                    border-radius: "0 0 10px 10px";
                    border-top: "1px solid #333";
                    text-align: "left";

                    hero_demo_label {
                        text: "Output:";
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
                        text: "click me";
                        padding: "8px 16px";
                        border-radius: "4px";
                        border: "none";
                        font-size: "0.9rem";
                        display: "inline-block";

                        ?click {
                            text: "clicked!";
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
                    text: "CUI is a compiled language where structure, style, and behavior live in one CSS-like syntax. Classes define how things look. Instances create them. Listeners handle events. Variables wire them together. The compiler figures out the rest -- what's static gets baked into HTML, what's dynamic gets compiled to WebAssembly.";
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
                    why_point_title { text: "Four building blocks"; }
                    why_point_body { text: "Instances create elements. Classes define reusable styling. Listeners handle events. Variables connect them with reactive state. No components, no hooks, no lifecycle methods."; }
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
                    font-weight: $fw_o;

                    ?click {
                        $show_cover: "block"; $show_l1: "none"; $show_l2: "none"; $show_l3: "none"; $show_l4: "none"; $show_l5: "none"; $show_l6: "none";
                        $fw_o: "700"; $fw_1: "400"; $fw_2: "400"; $fw_3: "400"; $fw_4: "400"; $fw_5: "400"; $fw_6: "400";
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
                    font-weight: $fw_1;

                    ?click {
                        $show_cover: "none"; $show_l1: "block"; $show_l2: "none"; $show_l3: "none"; $show_l4: "none"; $show_l5: "none"; $show_l6: "none";
                        $fw_o: "400"; $fw_1: "700"; $fw_2: "400"; $fw_3: "400"; $fw_4: "400"; $fw_5: "400"; $fw_6: "400";
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
                    font-weight: $fw_2;

                    ?click {
                        $show_cover: "none"; $show_l1: "none"; $show_l2: "block"; $show_l3: "none"; $show_l4: "none"; $show_l5: "none"; $show_l6: "none";
                        $fw_o: "400"; $fw_1: "400"; $fw_2: "700"; $fw_3: "400"; $fw_4: "400"; $fw_5: "400"; $fw_6: "400";
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
                    font-weight: $fw_3;

                    ?click {
                        $show_cover: "none"; $show_l1: "none"; $show_l2: "none"; $show_l3: "block"; $show_l4: "none"; $show_l5: "none"; $show_l6: "none";
                        $fw_o: "400"; $fw_1: "400"; $fw_2: "400"; $fw_3: "700"; $fw_4: "400"; $fw_5: "400"; $fw_6: "400";
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
                    font-weight: $fw_4;

                    ?click {
                        $show_cover: "none"; $show_l1: "none"; $show_l2: "none"; $show_l3: "none"; $show_l4: "block"; $show_l5: "none"; $show_l6: "none";
                        $fw_o: "400"; $fw_1: "400"; $fw_2: "400"; $fw_3: "400"; $fw_4: "700"; $fw_5: "400"; $fw_6: "400";
                    }
                }
                menu_5 {
                    text: "5. Variables";
                    cursor: "pointer";
                    padding: "8px 16px";
                    border-radius: "4px";
                    font-size: "0.85rem";
                    margin-bottom: "4px";
                    color: $fg;
                    font-weight: $fw_5;

                    ?click {
                        $show_cover: "none"; $show_l1: "none"; $show_l2: "none"; $show_l3: "none"; $show_l4: "none"; $show_l5: "block"; $show_l6: "none";
                        $fw_o: "400"; $fw_1: "400"; $fw_2: "400"; $fw_3: "400"; $fw_4: "400"; $fw_5: "700"; $fw_6: "400";
                    }
                }
                menu_6 {
                    text: "6. All Together";
                    cursor: "pointer";
                    padding: "8px 16px";
                    border-radius: "4px";
                    font-size: "0.85rem";
                    margin-bottom: "4px";
                    color: $fg;
                    font-weight: $fw_6;

                    ?click {
                        $show_cover: "none"; $show_l1: "none"; $show_l2: "none"; $show_l3: "none"; $show_l4: "none"; $show_l5: "none"; $show_l6: "block";
                        $fw_o: "400"; $fw_1: "400"; $fw_2: "400"; $fw_3: "400"; $fw_4: "400"; $fw_5: "400"; $fw_6: "700";
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
                            $fw_o: "400";
                            $fw_1: "700";
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

                    l1_code_wrap {
                        background: $code_bg;
                        border-radius: "8px 8px 0 0";
                        position: "relative";
                        margin-bottom: "0";

                        l1_tab { position: "absolute"; top: "8px"; right: "12px"; l1_to_html {} }

                        l1_cui {
                            display: $cui_1;
                            text: "greeting {\n    text: \"Hello, CUI!\";\n}";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                        l1_html {
                            display: $html_1;
                            text: "&lt;div&gt;Hello, CUI!&lt;/div&gt;\n\n/* No CSS needed — pure text content */\n/* Wasm: none — static HTML only */";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                    }

                    l1_demo {
                        background: $tile;
                        padding: "20px 24px";
                        border-radius: "0 0 8px 8px";
                        margin-bottom: "24px";
                        border-top: "1px solid #333";

                        l1_demo_label {
                            text: "Output:";
                            font-size: "0.75rem";
                            text-transform: "uppercase";
                            letter-spacing: "0.05em";
                            opacity: "0.5";
                            margin-bottom: "8px";
                        }

                        greeting {
                            text: "Hello, CUI!";
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
                                $fw_1: "400";
                                $fw_2: "700";
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
                        text: "Building hierarchy with nesting and styling";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }
                    l2_intro {
                        text: "Elements in CUI are just names followed by braces. Nesting creates parent-child relationships in the DOM. CSS properties go directly on the element — just like CSS selectors, but inline.";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l2_code_wrap {
                        background: $code_bg;
                        border-radius: "8px 8px 0 0";
                        position: "relative";

                        l2_tab { position: "absolute"; top: "8px"; right: "12px"; l2_to_html {} }

                        l2_cui {
                            display: $cui_2;
                            text: "card {\n    border-left: \"4px solid #5865f2\";\n    padding: \"16px\";\n    border-radius: \"4px\";\n\n    card_title {\n        text: \"My Card\";\n        font-weight: \"700\";\n        margin-bottom: \"8px\";\n    }\n    card_body {\n        text: \"Content inside the card.\";\n        opacity: \"0.7\";\n    }\n}";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                        l2_html_code {
                            display: $html_2;
                            text: "&lt;div style='border-left:4px solid #5865f2;\n  padding:16px;border-radius:4px;'&gt;\n  &lt;div style='font-weight:700;\n    margin-bottom:8px;'&gt;My Card&lt;/div&gt;\n  &lt;div style='opacity:0.7;'&gt;\n    Content inside the card.&lt;/div&gt;\n&lt;/div&gt;\n\n/* All inline styles — no classes needed */\n/* Wasm: none — static HTML only */";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                    }

                    l2_demo {
                        background: $tile;
                        padding: "20px 24px";
                        border-radius: "0 0 8px 8px";
                        margin-bottom: "24px";
                        border-top: "1px solid #333";

                        l2_demo_label {
                            text: "Output:";
                            font-size: "0.75rem";
                            text-transform: "uppercase";
                            letter-spacing: "0.05em";
                            opacity: "0.5";
                            margin-bottom: "8px";
                        }

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
                                $fw_2: "400";
                                $fw_1: "700";
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
                                $fw_2: "400";
                                $fw_3: "700";
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

                    l3_code_wrap {
                        background: $code_bg;
                        border-radius: "8px 8px 0 0";
                        position: "relative";

                        l3_tab { position: "absolute"; top: "8px"; right: "12px"; l3_to_html {} }

                        l3_cui {
                            display: $cui_3;
                            text: ".tag {\n    padding: \"6px 12px\";\n    border-radius: \"4px\";\n    color: \"white\";\n    font-size: \"0.9rem\";\n}\n\ntag { text: \"Default\"; background: \"#5865f2\"; }\ntag { text: \"Custom\";  background: \"#ff6b6b\"; }\ntag { text: \"Another\"; background: \"#ffa500\"; }";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                        l3_html_code {
                            display: $html_3;
                            text: "&lt;div style='background:#5865f2;'\n  class='o'&gt;Default&lt;/div&gt;\n&lt;div style='background:#ff6b6b;'\n  class='o'&gt;Custom&lt;/div&gt;\n&lt;div style='background:#ffa500;'\n  class='o'&gt;Another&lt;/div&gt;\n\n/* CSS (from .tag class) */\n.o { color:white; padding:6px 12px;\n     border-radius:4px; font-size:0.9rem; }\n\n/* Instance backgrounds are inline */\n/* Wasm: none — static HTML only */";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                    }

                    l3_demo {
                        background: $tile;
                        padding: "20px 24px";
                        border-radius: "0 0 8px 8px";
                        margin-bottom: "24px";
                        border-top: "1px solid #333";

                        l3_demo_label {
                            text: "Output:";
                            font-size: "0.75rem";
                            text-transform: "uppercase";
                            letter-spacing: "0.05em";
                            opacity: "0.5";
                            margin-bottom: "8px";
                        }

                        l3_tags {
                            display: "flex";
                            gap: "8px";

                            .tag {
                                padding: "6px 12px";
                                border-radius: "4px";
                                color: "white";
                                font-size: "0.9rem";
                            }

                            tag { text: "Default"; background: "#5865f2"; }
                            tag { text: "Custom"; background: "#ff6b6b"; }
                            tag { text: "Another"; background: "#ffa500"; }
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
                                $fw_3: "400";
                                $fw_2: "700";
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
                                $fw_3: "400";
                                $fw_4: "700";
                            }
                        }
                    }
                }

                // ═══════════════════════════════════
                //  LESSON 4: Events (simplified — no variable needed)
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
                        text: "Listeners (?click, ?blur, ?focus, etc.) respond to browser events. Properties inside a listener cascade onto the parent element when the event fires. Everything compiles to WebAssembly.";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l4_code_wrap {
                        background: $code_bg;
                        border-radius: "8px 8px 0 0";
                        position: "relative";

                        l4_tab { position: "absolute"; top: "8px"; right: "12px"; l4_to_html {} }

                        l4_cui {
                            display: $cui_4;
                            text: "button {\n    text: \"Click me\";\n    color: \"white\";\n    background: \"#5865f2\";\n    cursor: \"pointer\";\n\n    ?click {\n        text: \"Clicked!\";\n        background: \"#ff6b6b\";\n    }\n}";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                        l4_html_code {
                            display: $html_4;
                            text: "&lt;div style='color:white;font-weight:500;\n  cursor:pointer;background:#5865f2;\n  border-radius:6px;border:none;\n  display:inline-block;\n  padding:10px 20px;'&gt;\n  Click me&lt;/div&gt;\n\n/* All instance properties — inline */\n/* Wasm: click updates text, background */";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                    }

                    l4_demo {
                        background: $tile;
                        padding: "20px 24px";
                        border-radius: "0 0 8px 8px";
                        margin-bottom: "24px";
                        border-top: "1px solid #333";

                        l4_demo_label {
                            text: "Output:";
                            font-size: "0.75rem";
                            text-transform: "uppercase";
                            letter-spacing: "0.05em";
                            opacity: "0.5";
                            margin-bottom: "8px";
                        }

                        button {
                            text: "Click me";
                            color: "white";
                            background: "#5865f2";
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "500";
                            border: "none";
                            display: "inline-block";

                            ?click {
                                text: "Clicked!";
                                background: "#ff6b6b";
                            }
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
                                $fw_4: "400";
                                $fw_3: "700";
                            }
                        }
                        l4_next {
                            text: "Variables →";
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
                                $fw_4: "400";
                                $fw_5: "700";
                            }
                        }
                    }
                }

                // ═══════════════════════════════════
                //  LESSON 5: Variables
                // ═══════════════════════════════════
                lesson_5 {
                    display: $show_l5;

                    l5_title {
                        text: "Lesson 5: Variables";
                        font-size: "1.6rem";
                        font-weight: "700";
                        color: $accent;
                        margin-bottom: "8px";
                    }
                    l5_subtitle {
                        text: "Reactive state that connects elements";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }
                    l5_intro {
                        text: "Variables (let $name: value) hold state. When a listener assigns a new value, every element reading that variable updates automatically. This is how one element's event can change another element's appearance.";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l5_code_wrap {
                        background: $code_bg;
                        border-radius: "8px 8px 0 0";
                        position: "relative";

                        l5_tab { position: "absolute"; top: "8px"; right: "12px"; l5_to_html {} }

                        l5_cui {
                            display: $cui_5;
                            text: "let $color: \"#888\";\nlet $status: \"Waiting...\";\n\nlabel {\n    text: $status;\n    color: $color;\n    font-style: \"italic\";\n}\n\nbutton {\n    text: \"Activate\";\n    cursor: \"pointer\";\n    ?click {\n        $status: \"Active!\";\n        $color: \"#5865f2\";\n    }\n}";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                        l5_html_code {
                            display: $html_5;
                            text: "&lt;div style='font-style:italic;color:#888;\n  font-size:1.1rem;margin-bottom:12px;'&gt;\n  Waiting...&lt;/div&gt;\n&lt;div style='font-weight:500;\n  border-radius:6px;color:white;\n  display:inline-block;background:#5865f2;\n  padding:10px 20px;cursor:pointer;\n  border:none;'&gt;Activate&lt;/div&gt;\n\n/* Wasm: click updates $status text */\n/* and $color on the label element */";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                    }

                    l5_demo {
                        background: $tile;
                        padding: "20px 24px";
                        border-radius: "0 0 8px 8px";
                        margin-bottom: "24px";
                        border-top: "1px solid #333";

                        l5_demo_label {
                            text: "Output:";
                            font-size: "0.75rem";
                            text-transform: "uppercase";
                            letter-spacing: "0.05em";
                            opacity: "0.5";
                            margin-bottom: "8px";
                        }

                        let $l5_color: "#888";
                        let $l5_status: "Waiting...";

                        label {
                            text: $l5_status;
                            color: $l5_color;
                            font-style: "italic";
                            font-size: "1.1rem";
                            margin-bottom: "12px";
                        }

                        button {
                            text: "Activate";
                            color: "white";
                            background: "#5865f2";
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "500";
                            border: "none";
                            display: "inline-block";

                            ?click {
                                $l5_status: "Active!";
                                $l5_color: "#5865f2";
                            }
                        }
                    }

                    l5_nav {
                        margin-top: "40px";
                        display: "flex";
                        justify-content: "space-between";

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
                                $fw_5: "400";
                                $fw_4: "700";
                            }
                        }
                        l5_next {
                            text: "Putting It Together →";
                            background: $accent;
                            color: "white";
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $show_l5: "none";
                                $show_l6: "block";
                                $fw_5: "400";
                                $fw_6: "700";
                            }
                        }
                    }
                }

                // ═══════════════════════════════════
                //  LESSON 6: Putting It All Together
                // ═══════════════════════════════════
                lesson_6 {
                    display: $show_l6;

                    l6_title {
                        text: "Lesson 6: Putting It All Together";
                        font-size: "1.6rem";
                        font-weight: "700";
                        color: $accent;
                        margin-bottom: "8px";
                    }
                    l6_subtitle {
                        text: "Structure + classes + events + variables in one component";
                        color: "#888";
                        font-style: "italic";
                        margin-bottom: "24px";
                    }
                    l6_intro {
                        text: "Now let's combine everything. This to-do list uses: nested elements for structure, a class for shared item styling, apply to toggle checkbox state, and a variable so checking a box changes a separate status element.";
                        line-height: "1.6";
                        margin-bottom: "24px";
                    }

                    l6_code_wrap {
                        background: $code_bg;
                        border-radius: "8px 8px 0 0";
                        position: "relative";

                        l6_tab { position: "absolute"; top: "8px"; right: "12px"; l6_to_html {} }

                        l6_cui {
                            display: $cui_6;
                            text: "let $status: \"nothing checked\";\n\n.item {\n    display: \"flex\";\n    align-items: \"center\";\n    padding: \"12px\";\n    border-left: \"4px solid #5865f2\";\n    margin-bottom: \"8px\";\n}\n\n.unchecked {\n    width: \"20px\"; height: \"20px\";\n    background: \"#ddd\";\n    border-radius: \"3px\";\n    cursor: \"pointer\";\n    margin-right: \"12px\";\n    ?click {\n        $status: \"making progress!\";\n        apply: .checked;\n    }\n}\n.checked {\n    width: \"20px\"; height: \"20px\";\n    background: \"#5865f2\";\n    border-radius: \"3px\";\n    cursor: \"pointer\";\n    margin-right: \"12px\";\n    ?click {\n        $status: \"unchecked one\";\n        apply: .unchecked;\n    }\n}\n\nstatus { text: $status; }\n\nitem {\n    unchecked {}\n    label { text: \"Learn CUI\"; }\n}\nitem {\n    unchecked {}\n    label { text: \"Build something\"; }\n}";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                        l6_html_code {
                            display: $html_6;
                            text: "&lt;div style='color:#888;margin-bottom:12px;\n  font-style:italic;'&gt;nothing checked&lt;/div&gt;\n&lt;div class='s'&gt;\n  &lt;div class='t'&gt;&lt;/div&gt;\n  &lt;div&gt;Learn CUI&lt;/div&gt;\n&lt;/div&gt;\n&lt;div class='s'&gt;\n  &lt;div class='t'&gt;&lt;/div&gt;\n  &lt;div&gt;Build something&lt;/div&gt;\n&lt;/div&gt;\n\n/* CSS (from .item and .unchecked) */\n.s { display:flex; align-items:center;\n     padding:12px; margin-bottom:8px;\n     border-left:4px solid #5865f2; }\n.t { background:#ddd; width:20px;\n     height:20px; cursor:pointer;\n     margin-right:12px; border-radius:3px; }\n\n/* Wasm: click toggles class (apply),\n   updates $status text */";
                            color: "#e2e8f0";
                            padding: "16px 20px";
                            font-family: "ui-monospace, SFMono-Regular, Menlo, monospace";
                            font-size: "0.85rem";
                            overflow-x: "auto";
                            white-space: "pre";
                            line-height: "1.6";
                        }
                    }

                    l6_demo {
                        background: $tile;
                        padding: "20px 24px";
                        border-radius: "0 0 8px 8px";
                        margin-bottom: "24px";
                        border-top: "1px solid #333";

                        l6_demo_label {
                            text: "Output:";
                            font-size: "0.75rem";
                            text-transform: "uppercase";
                            letter-spacing: "0.05em";
                            opacity: "0.5";
                            margin-bottom: "8px";
                        }

                        let $l6_status: "nothing checked";

                        .item {
                            display: "flex";
                            align-items: "center";
                            padding: "12px";
                            border-left: "4px solid #5865f2";
                            margin-bottom: "8px";
                            border-radius: "4px";
                        }

                        .unchecked {
                            width: "20px";
                            height: "20px";
                            background: "#ddd";
                            border-radius: "3px";
                            margin-right: "12px";
                            cursor: "pointer";
                            flex-shrink: "0";
                            ?click {
                                $l6_status: "making progress!";
                                apply: .checked;
                            }
                        }
                        .checked {
                            width: "20px";
                            height: "20px";
                            background: "#5865f2";
                            border-radius: "3px";
                            margin-right: "12px";
                            cursor: "pointer";
                            flex-shrink: "0";
                            ?click {
                                $l6_status: "unchecked one";
                                apply: .unchecked;
                            }
                        }

                        l6_status_display {
                            text: $l6_status;
                            font-style: "italic";
                            color: "#888";
                            margin-bottom: "12px";
                        }

                        item {
                            unchecked {}
                            label {
                                text: "Learn CUI";
                            }
                        }
                        item {
                            unchecked {}
                            label {
                                text: "Build something";
                            }
                        }
                    }

                    l6_nav {
                        margin-top: "40px";
                        display: "flex";
                        justify-content: "space-between";
                        align-items: "center";

                        l6_prev {
                            text: "← Variables";
                            background: $tile;
                            color: $fg;
                            padding: "10px 20px";
                            border-radius: "6px";
                            cursor: "pointer";
                            font-weight: "600";
                            border: "none";

                            ?click {
                                $show_l6: "none";
                                $show_l5: "block";
                                $fw_6: "400";
                                $fw_5: "700";
                            }
                        }
                    }

                    l6_finish {
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
                            text: "Four building blocks — instances, classes, listeners, and variables — plus properties. The compiler handles the rest: cascading, scoping, and splitting your code across HTML, CSS, and WebAssembly.";
                            line-height: "1.6";
                            margin-bottom: "20px";
                            opacity: "0.8";
                        }
                        finish_home {
                            text: "← Back to Home";
                            color: $accent;
                            font-weight: "600";
                            cursor: "pointer";

                            ?click {
                                $show_tutorial: "none";
                                $show_home: "block";
                            }
                        }
                    }
                }
            }
        }
    }
}
