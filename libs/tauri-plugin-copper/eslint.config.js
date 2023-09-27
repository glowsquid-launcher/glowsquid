import { join } from "path";
import baseConfig from "../../eslint.config.js";
import { fileURLToPath } from "url";

/** @type {import('eslint').Linter.FlatConfig[]} */
export default baseConfig(
    join(fileURLToPath(import.meta.url), '..', 'tsconfig.lib.json')
)
