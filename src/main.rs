// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use components::radar::container::RadarContainer;
use components::radar::{DataPoint, RadarCurve};
use components::theme_buttons::ThemeButtons;

/// Define a components module that contains all shared components for our app.
mod components;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // Create sample data for the radar graph
    let axes = vec![
        "Speed".to_string(),
        "Power".to_string(),
        "Accuracy".to_string(),
        "Range".to_string(),
        "Durability".to_string(),
    ];

    let curve1 = RadarCurve {
        name: "Model A".to_string(),
        color: "#3366CC".to_string(),
        data_points: vec![
            DataPoint {
                value: 70.0,
                label: "Speed".to_string(),
            },
            DataPoint {
                value: 85.0,
                label: "Power".to_string(),
            },
            DataPoint {
                value: 65.0,
                label: "Accuracy".to_string(),
            },
            DataPoint {
                value: 90.0,
                label: "Range".to_string(),
            },
            DataPoint {
                value: 75.0,
                label: "Durability".to_string(),
            },
        ],
    };

    let curve2 = RadarCurve {
        name: "Model B".to_string(),
        color: "#DC3912".to_string(),
        data_points: vec![
            DataPoint {
                value: 80.0,
                label: "Speed".to_string(),
            },
            DataPoint {
                value: 65.0,
                label: "Power".to_string(),
            },
            DataPoint {
                value: 90.0,
                label: "Accuracy".to_string(),
            },
            DataPoint {
                value: 70.0,
                label: "Range".to_string(),
            },
            DataPoint {
                value: 85.0,
                label: "Durability".to_string(),
            },
        ],
    };

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    // Initialize dark mode on load - only for web platform
    #[cfg(feature = "web")]
    use_effect(|| {
        let script = r#"
        // Check for color theme preference and apply it
        function applyTheme() {
            if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
                document.documentElement.classList.add('dark');
                console.log('Dark theme applied');
            } else {
                document.documentElement.classList.remove('dark');
                console.log('Light theme applied');
            }
        }
        
        // Apply theme immediately
        applyTheme();
        
        // Listen for system preference changes
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', event => {
            if (!localStorage.theme) {
                if (event.matches) {
                    document.documentElement.classList.add('dark');
                    console.log('System theme changed to dark');
                } else {
                    document.documentElement.classList.remove('dark');
                    console.log('System theme changed to light');
                }
            }
        });
        "#;

        let _ = web_sys::window().map(|_| {
            let _ = js_sys::eval(script);
        });
    });

    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "container mx-auto px-4 py-8 bg-background dark:bg-gray-900 text-text dark:text-white min-h-screen transition-colors duration-300",
            div {
                class: "flex justify-between items-center mb-6",
                h1 {
                    class: "text-3xl font-bold",
                    "Radar Graph Demo"
                }

                // Theme buttons component will be empty on non-web platforms
                ThemeButtons {}
            }
            // Responsive radar graph container
            div {
                class: "container mx-auto",
                RadarContainer {
                    axes: axes,
                    curves: vec![curve1, curve2],
                    max_value: 100.0,
                    width: 600,
                    height: 500,
                }
            }
        }
    }
}
