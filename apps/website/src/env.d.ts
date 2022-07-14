/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly MICROSOFT_CLIENT_ID: string
  readonly MICROSOFT_CLIENT_SECRET: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
