/* eslint-disable camelcase */
export interface License {
  id: string;
  name: string;
  url: string;
}
export default interface Mod {
  id: string;
  slug: string;
  team: string;
  title: string;
  description: string;
  body_url: string;
  published: Date;
  updated: Date;
  status: string;
  license: License;
  client_side: string;
  server_side: string;
  downloads: number;
  categories: string[];
  versions: string[];
  discord_url?: string;
  donation_urls?: string[];
  icon_url?: string;
  issues_url?: string;
  source_url?: string;
  wiki_url?: string;
}
