import { join } from "path";
import { fileURLToPath } from "url";

import { createConfig } from "../../eslint.config.js";

export default createConfig(
  [
    join(fileURLToPath(import.meta.url), '..', 'tsconfig.json'),
    join(fileURLToPath(import.meta.url), '..', 'tsconfig.app.json'),
    join(fileURLToPath(import.meta.url), '..', 'tsconfig.spec.json')
  ]
)
