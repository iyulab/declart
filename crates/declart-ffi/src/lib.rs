use declart_core::render::Theme;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Renders a TOML diagram declaration to SVG.
///
/// - `input`: null-terminated UTF-8 TOML declaration string (must not be null)
/// - `theme`: null-terminated theme name (`"default"` or `"monochrome"`; unknown → `"default"`)
/// - `width`: canvas width override in pixels; pass `0` for no override
///
/// Returns a null-terminated UTF-8 SVG string allocated on the heap.
/// The caller must free the returned pointer with [`declart_free`].
/// Returns null on null input or internal error.
///
/// # Safety
/// `input` and `theme` must be valid null-terminated C strings.
#[no_mangle]
pub unsafe extern "C" fn declart_render(
    input: *const c_char,
    theme: *const c_char,
    width: u32,
) -> *mut c_char {
    if input.is_null() || theme.is_null() {
        return std::ptr::null_mut();
    }
    let input_str = match CStr::from_ptr(input).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let theme_str = CStr::from_ptr(theme).to_str().unwrap_or("default");
    let t = Theme::by_name(theme_str);
    let width_opt = if width == 0 { None } else { Some(width) };

    let result = declart_core::parse(input_str)
        .and_then(|d| declart_core::render_opts(&d, t, width_opt));

    match result {
        Ok(svg) => match CString::new(svg) {
            Ok(cs) => cs.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        Err(_) => std::ptr::null_mut(),
    }
}

/// Validates a TOML diagram declaration without rendering.
///
/// - `input`: null-terminated UTF-8 TOML declaration string (must not be null)
///
/// Returns null if the declaration is valid.
/// Returns a null-terminated error message string on failure; the caller must free it with [`declart_free`].
///
/// # Safety
/// `input` must be a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn declart_validate(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return CString::new("null input").unwrap().into_raw();
    }
    let input_str = match CStr::from_ptr(input).to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("invalid UTF-8").unwrap().into_raw(),
    };
    match declart_core::parse(input_str) {
        Ok(_) => std::ptr::null_mut(),
        Err(e) => CString::new(e.to_string()).unwrap_or_default().into_raw(),
    }
}

/// Frees a string returned by [`declart_render`] or [`declart_validate`].
///
/// Passing null is safe and has no effect. The pointer must not be used after this call.
///
/// # Safety
/// `ptr` must be a pointer previously returned by `declart_render` or `declart_validate`,
/// or null.
#[no_mangle]
pub unsafe extern "C" fn declart_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn c(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    #[test]
    fn render_valid_pyramid() {
        let input = c("kind = \"sequence\"
view = \"pyramid\"\n[[items]]\nlabel = \"Top\"\n[[items]]\nlabel = \"Bottom\"\n");
        let theme = c("default");
        unsafe {
            let ptr = declart_render(input.as_ptr(), theme.as_ptr(), 0);
            assert!(!ptr.is_null(), "expected SVG, got null");
            let svg = CStr::from_ptr(ptr).to_str().unwrap();
            assert!(svg.contains("<svg"));
            declart_free(ptr);
        }
    }

    #[test]
    fn render_with_width() {
        let input = c("kind = \"sequence\"
view = \"pyramid\"\n[[items]]\nlabel = \"Top\"\n[[items]]\nlabel = \"Bottom\"\n");
        let theme = c("default");
        unsafe {
            let ptr = declart_render(input.as_ptr(), theme.as_ptr(), 400);
            assert!(!ptr.is_null());
            let svg = CStr::from_ptr(ptr).to_str().unwrap();
            assert!(svg.contains("width=\"400\""));
            declart_free(ptr);
        }
    }

    #[test]
    fn render_invalid_returns_null() {
        let input = c("kind = \"nonexistent\"");
        let theme = c("default");
        unsafe {
            let ptr = declart_render(input.as_ptr(), theme.as_ptr(), 0);
            assert!(ptr.is_null(), "expected null for invalid input");
        }
    }

    #[test]
    fn render_null_input_returns_null() {
        let theme = c("default");
        unsafe {
            let ptr = declart_render(std::ptr::null(), theme.as_ptr(), 0);
            assert!(ptr.is_null());
        }
    }

    #[test]
    fn validate_valid_returns_null() {
        let input = c("kind = \"sequence\"
view = \"pyramid\"\n[[items]]\nlabel = \"Item\"\n");
        unsafe {
            let ptr = declart_validate(input.as_ptr());
            assert!(ptr.is_null(), "expected null for valid input");
        }
    }

    #[test]
    fn validate_invalid_returns_error_message() {
        let input = c("kind = \"nonexistent\"");
        unsafe {
            let ptr = declart_validate(input.as_ptr());
            assert!(!ptr.is_null(), "expected error message for invalid input");
            let msg = CStr::from_ptr(ptr).to_str().unwrap();
            assert!(!msg.is_empty());
            declart_free(ptr);
        }
    }

    #[test]
    fn free_null_is_safe() {
        unsafe { declart_free(std::ptr::null_mut()); }
    }
}
