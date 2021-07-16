/* eslint-disable camelcase */
export default interface ModVersion {
  id: string;
  mod_id: string;
  author_id: string;
  featured: boolean;
  name: string;
  version_number: string;
  changelog?: string;
  changelog_url?: string;
  date_published: Date;
  downloads: number;
  version_type: string;
  files: File[];
  dependencies: string[];
  game_versions: string[];
  loaders: string[];
}

export interface File {
  hashes: Hashes;
  url: string;
  filename: string;
  primary: boolean;
}

export interface Hashes {
  sha1: string;
  sha512?: string;
}
