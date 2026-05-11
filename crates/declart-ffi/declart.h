/**
 * Declart FFI — C ABI for the Declart declarative diagram engine.
 *
 * All strings are null-terminated UTF-8. Heap-allocated return values must be
 * freed with declart_free(). Passing null to any function that expects a string
 * is safe — it returns null or a null-terminated empty result.
 */
#ifndef DECLART_H
#define DECLART_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Renders a TOML diagram declaration to an SVG string.
 *
 * @param input  Null-terminated UTF-8 TOML declaration.
 * @param theme  Null-terminated theme name: "default" or "monochrome".
 *               Unknown values fall back to "default".
 * @param width  Canvas width override in pixels. Pass 0 for no override.
 * @return       Heap-allocated null-terminated SVG string, or NULL on error.
 *               Must be freed with declart_free().
 */
char* declart_render(const char* input, const char* theme, uint32_t width);

/**
 * Validates a TOML diagram declaration without rendering.
 *
 * @param input  Null-terminated UTF-8 TOML declaration.
 * @return       NULL if valid; heap-allocated error message string on failure.
 *               The error string must be freed with declart_free().
 */
char* declart_validate(const char* input);

/**
 * Frees a string returned by declart_render() or declart_validate().
 *
 * Passing NULL is safe and has no effect.
 */
void declart_free(char* ptr);

#ifdef __cplusplus
}
#endif

#endif /* DECLART_H */
