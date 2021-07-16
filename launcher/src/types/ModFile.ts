export default interface ModFile {
  path: string;
  hashes?: { sha1: string; }
  env?: { client: string; server: string; }
  downloads: string[];
  id: string;
}
