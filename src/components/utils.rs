use dioxus::prelude::*;

/// Converts polar coordinates to Cartesian coordinates
pub fn polar_to_cartesian(
    radius: f32,
    angle_in_radians: f32,
    center_x: f32,
    center_y: f32,
) -> (f32, f32) {
    let x = center_x + radius * angle_in_radians.cos();
    let y = center_y + radius * angle_in_radians.sin();
    (x, y)
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_polar_to_cartesian() {
        // Test basic conversions
        let center_x = 100.0;
        let center_y = 100.0;

        // Right direction (0 degrees)
        let (x, y) = polar_to_cartesian(10.0, 0.0, center_x, center_y);
        assert_eq!(x as i32, 110);
        assert_eq!(y as i32, 100);

        // Up direction (270 degrees or -90 degrees)
        let (x, y) = polar_to_cartesian(10.0, -PI / 2.0, center_x, center_y);
        assert_eq!(x as i32, 100);
        assert_eq!(y as i32, 90);

        // Left direction (180 degrees)
        let (x, y) = polar_to_cartesian(10.0, PI, center_x, center_y);
        assert_eq!(x as i32, 90);
        assert_eq!(y as i32, 100);

        // Down direction (90 degrees)
        let (x, y) = polar_to_cartesian(10.0, PI / 2.0, center_x, center_y);
        assert_eq!(x as i32, 100);
        assert_eq!(y as i32, 110);
    }
}