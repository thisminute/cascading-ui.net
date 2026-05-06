extern crate cascading_ui;
use cascading_ui::cui;

cui! {
    title: "CUI — Cascading UI";

    // Global styles
    .page {
        font-family: "Inter, system-ui, -apple-system, sans-serif";
        max-width: "800px";
        margin: "0 auto";
        padding: "40px 24px";
        color: "#1a1a2e";
        line-height: "1.7";
    }

    .hero {
        padding: "80px 0 60px";
        text-align: "center";
    }

    .hero_title {
        font-size: "3.5rem";
        font-weight: "800";
        letter-spacing: "-0.03em";
        margin-bottom: "0";
        color: "#0f0f23";
    }

    .hero_subtitle {
        font-size: "1.25rem";
        color: "#555";
        margin-top: "12px";
        font-weight: "400";
    }

    .section {
        margin-bottom: "48px";
    }

    .section_title {
        font-size: "1.5rem";
        font-weight: "700";
        margin-bottom: "16px";
        color: "#0f0f23";
        border-bottom: "2px solid #e0e0e0";
        padding-bottom: "8px";
    }

    .point {
        margin-bottom: "24px";
        padding: "20px 24px";
        background: "#f8f9fa";
        border-radius: "8px";
        border-left: "4px solid #4a6cf7";
    }

    .point_title {
        font-weight: "700";
        font-size: "1.1rem";
        margin-bottom: "4px";
        color: "#0f0f23";
    }

    .point_body {
        color: "#444";
    }

    .code_block {
        font-family: "JetBrains Mono, Fira Code, monospace";
        font-size: "0.9rem";
        background: "#1e1e2e";
        color: "#cdd6f4";
        padding: "20px 24px";
        border-radius: "8px";
        margin: "16px 0";
        white-space: "pre";
        overflow-x: "auto";
        line-height: "1.5";
    }

    .demo_area {
        padding: "24px";
        border: "2px solid #e0e0e0";
        border-radius: "8px";
        margin: "16px 0";
        text-align: "center";
    }

    .demo_button {
        padding: "12px 24px";
        background: "#4a6cf7";
        color: "white";
        border: "none";
        border-radius: "6px";
        font-size: "1rem";
        cursor: "pointer";
        font-weight: "600";
    }

    .footer {
        margin-top: "64px";
        padding-top: "24px";
        border-top: "1px solid #e0e0e0";
        color: "#888";
        font-size: "0.9rem";
        text-align: "center";
    }

    .link_style {
        color: "#4a6cf7";
        text-decoration: "none";
        font-weight: "600";
    }

    // Page structure
    page {
        .page {}

        header {
            .hero {}

            h1 {
                .hero_title {}
                text: "CUI";
            }
            tagline {
                .hero_subtitle {}
                text: "A web language based on CSS syntax. No JavaScript required.";
            }
        }

        intro {
            .section {}

            heading {
                .section_title {}
                text: "What is CUI?";
            }
            description {
                text: "CUI (Cascading UI) is a compiled web language where you write your entire application in CSS-like syntax. It compiles to static HTML, CSS, and WebAssembly — producing fast, minimal output with zero JavaScript runtime.";
            }
            example {
                .code_block {}
                text: "text: \"hello world\";\ncolor: \"blue\";\n?click {\n    text: \"clicked!\";\n    color: \"green\";\n}";
            }
        }

        selling_points {
            .section {}

            heading {
                .section_title {}
                text: "Why CUI?";
            }

            point_1 {
                .point {}
                title {
                    .point_title {}
                    text: "Zero JavaScript";
                }
                body {
                    .point_body {}
                    text: "CUI compiles to WebAssembly. No bundler, no transpiler, no npm. Your event handlers and reactivity run as native Wasm — faster than any JS framework.";
                }
            }

            point_2 {
                .point {}
                title {
                    .point_title {}
                    text: "Three-layer compilation";
                }
                body {
                    .point_body {}
                    text: "The compiler detects what's fully static (baked into HTML), what needs one-time initialization (wired up at page load), and what's truly reactive (re-renders on state change). Static pages have zero runtime cost.";
                }
            }

            point_3 {
                .point {}
                title {
                    .point_title {}
                    text: "CSS semantics you already know";
                }
                body {
                    .point_body {}
                    text: "Classes cascade. Properties inherit. Specificity determines priority. If you know CSS, you already understand the mental model — CUI just extends it to structure and behavior.";
                }
            }

            point_4 {
                .point {}
                title {
                    .point_title {}
                    text: "Declarative everything";
                }
                body {
                    .point_body {}
                    text: "Structure, style, and behavior in one unified syntax. No template language. No separate script tag. No context switching between HTML, CSS, and JS.";
                }
            }
        }

        demo_section {
            .section {}

            heading {
                .section_title {}
                text: "Live demo";
            }
            description {
                text: "This entire page is built with CUI. Here's a live interactive element:";
            }
            demo {
                .demo_area {}
                button {
                    .demo_button {}
                    $label: "Click me";
                    text: $label;
                    ?click {
                        $label: "Clicked! CUI handles this through Wasm.";
                    }
                }
            }
        }

        architecture {
            .section {}

            heading {
                .section_title {}
                text: "How it works";
            }
            description {
                text: "CUI is implemented as a Rust procedural macro. Your CUI source code is parsed, analyzed, and compiled at build time:";
            }
            pipeline {
                .code_block {}
                text: "CUI source\n  -> parse (syn)\n  -> AST\n  -> analyze\n  -> semantics tree\n  -> render & cascade\n  -> compile\n  -> HTML + CSS + Wasm";
            }
            explanation {
                margin-top: "16px";
                text: "The cascading phase resolves class inheritance, variable scoping, and determines which layer each piece of content belongs to. Only truly dynamic parts incur any runtime cost.";
            }
        }

        links_section {
            .section {}

            heading {
                .section_title {}
                text: "Get started";
            }
            github {
                .link_style {}
                link: "https://github.com/thisminute/cascading-ui";
                text: "GitHub: cascading-ui";
            }
        }

        site_footer {
            .footer {}
            text: "Built with CUI. MIT License.";
        }
    }
}
