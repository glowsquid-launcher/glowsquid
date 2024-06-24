#!/usr/bin/env npx zx

// create db/glowsquid.db3 files in root and apps/glowsquid if not exists

cd('../..');
if (!fs.existsSync('db/glowsquid.db3')) {
    console.log('Creating db/glowsquid.db3');
    await fs.outputFile('db/glowsquid.db3', '');
    await $("cargo sqlx migrate run -D sqlite://db/glowsquid.db3 --source apps/glowsquid/db/migrations")
}

if (!fs.existsSync('apps/glowsquid/db/glowsquid.db3')) {
    console.log('Creating apps/glowsquid/db/glowsquid.db3');
    await fs.outputFile('apps/glowsquid/db/glowsquid.db3', '');
    await $("cargo sqlx migrate run -D sqlite://apps/glowsquid/db/glowsquid.db3 --source apps/glowsquid/db/migrations")
}
