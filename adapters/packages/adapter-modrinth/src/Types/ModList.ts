export interface ModList {
  hits: Hit[]
  offset: number
  limit: number
  total_hits: number
}

export interface Hit {
  mod_id: string
  slug: string
  author: string
  title: string
  description: string
  categories: string[]
  versions: string[]
  downloads: number
  follows: number
  page_url: string
  icon_url: string
  author_url: string
  date_created: Date
  date_modified: Date
  latest_version: string
  license: string
  client_side: Side
  server_side: Side
  host: Host
}

export enum Side {
  Optional = 'optional',
  Required = 'required',
  Unsupported = 'unsupported',
}

export enum Host {
  Modrinth = 'modrinth',
}
