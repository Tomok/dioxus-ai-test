use dioxus::prelude::*;

/// ThemeButtons component for controlling light/dark mode theme switching.
///
/// This component is only available on web platforms (with the "web" feature flag) because:
/// 1. It uses web-specific APIs like localStorage and document.documentElement
/// 2. It uses js_sys::eval() which requires WASM and browser environment
/// 3. These APIs are not available in desktop or other non-web environments
///
/// For desktop builds, an alternative theming approach would be needed,
/// but for now we provide an empty component to maintain API compatibility.
#[cfg(feature = "web")]
#[component]
pub fn ThemeButtons() -> Element {
    rsx! {
        div {
            class: "flex gap-2 items-center",
            button {
                class: "p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors",
                onclick: move |_| {
                    let script = r#"
                        localStorage.theme = 'light'; 
                        document.documentElement.classList.remove('dark');
                        console.log('Light mode activated');
                    "#;
                    let _ = js_sys::eval(script);
                },
                title: "Light mode",
                "☀️"
            }
            button {
                class: "p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors",
                onclick: move |_| {
                    let script = r#"
                        localStorage.removeItem('theme'); 
                        if (window.matchMedia('(prefers-color-scheme: dark)').matches) { 
                            document.documentElement.classList.add('dark');
                            console.log('System theme (dark) activated'); 
                        } else { 
                            document.documentElement.classList.remove('dark');
                            console.log('System theme (light) activated');
                        }
                    "#;
                    let _ = js_sys::eval(script);
                },
                title: "System preference",
                "🌓"
            }
            button {
                class: "p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors",
                onclick: move |_| {
                    let script = r#"
                        localStorage.theme = 'dark'; 
                        document.documentElement.classList.add('dark');
                        console.log('Dark mode activated');
                    "#;
                    let _ = js_sys::eval(script);
                },
                title: "Dark mode",
                "🌙"
            }
        }
    }
}

/// Empty ThemeButtons component for non-web platforms.
///
/// This version is provided for non-web builds (desktop, mobile) because:
/// 1. Web-specific APIs like localStorage and document are not available
/// 2. js_sys::eval() would panic on non-web targets as seen in the error:
///    "cannot call wasm-bindgen imported functions on non-wasm targets"
/// 3. Provides API compatibility so the same component can be used regardless of target
///
/// TODO: Implement proper theme switching for desktop/mobile platforms if needed
#[cfg(not(feature = "web"))]
#[component]
pub fn ThemeButtons() -> Element {
    // Return an empty element for non-web platforms
    rsx! { "" }
}
