/* eslint-disable camelcase */
export default interface ModResult {
  mod_id: string;
  description: string;
  project_type: string;
  author: string;
  title: string;
  categories: string[];
  versions: string[];
  downloads: number;
  page_url: string;
  icon_url: string;
  author_url: string;
  date_created: string;
  date_modified: string;
  latest_version: string;
  license: string;
  client_side: string;
  server_side: string;
  host: 'modrinth';
}
