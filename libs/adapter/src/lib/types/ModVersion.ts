export interface ModVersion {
  /**
   * The platform this mod version originated from. This **needs** be the id specified in your adapter config
   */
  platform: string;
  /**
   * The ID of the version
   */
  id: string;
  /**
   * The mod ID this version belongs to
   */
  modId: string;
  /**
   * name of this version
   */
  name: string;
  /**
   * build/version number
   */
  number?: string;
  /**
   * sha1 hash
   */
  sha1?: string;
  /**
   * the download URL for the main .jar file
   */
  downloadUrl: string;
  /**
   * changelog specified by the mods author(s)
   */
  changelog?: string;
  /**
   * supported minecraft versions
   *
   * @example 1.16.1
   * @example 1.2.1
   * @example 20w51a
   * @example 1.17.0
   */
  supportedVersions: string[];

  /**
   * Other mods' versions and IDs required for this
   *
   * don't worry about dependencies for dependencies we will handle that for you
   * The mod ID is the key and the version ID is the value
   */
  dependencies?: {
    modId: string;
    versionId: string;
  }[];
}
