/**
 * @type {import("@inlang/core/config").DefineConfig}
 */
export async function defineConfig(env) {
    const { default: typesafeI18nPlugin } = await env.$import(
        'https://cdn.jsdelivr.net/gh/ivanhofer/inlang-plugin-typesafe-i18n@2.3.0/dist/index.js'
    );

    const { default: standardLintRules } = await env.$import(
        'https://cdn.jsdelivr.net/gh/inlang/standard-lint-rules@2/dist/index.js'
    );

    return {
        plugins: [typesafeI18nPlugin({}), standardLintRules()]
    };
}
