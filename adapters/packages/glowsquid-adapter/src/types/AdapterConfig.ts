export interface AdapterConfig {
  /**
   * The platform that this adapter supports
   *
   * This is for matching mods with this adapter
   */
  platform: string
  /**
   * the ID of this mod
   *
   * This is for matching mods with this adapter
   */
  id: string
  /**
   * human-readable name for this adapter
   */
  name: string

  /**
   * Semver version string
   *
   * @example 1.2.3
   */
  version: string

  /**
   * @optional github repo for the adapter. Not required but recommended for automatic updates
   * @example glowsquid-launcher/adapters
   *
   * uses github releases to get the latest version
   * gets the package ID and adds a .js extension when looking for the release file
   * example of release file: if your adapter ID is `modrinth` it will look for `modrinth.js` under the latest release
   */
  githubRepo?: string

  /**
   * Icon for this adapter/platform it supports
   */
  iconUrl: string
}
