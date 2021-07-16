export interface ModVersion {
  /**
   * The ID of the version
   */
  id: string
  /**
   * name of this version
   */
  name: string
  /**
   * build/version number
   */
  number?: string
  /**
   * sha1 hash
   */
  sha1?: string
  /**
   * the download URL for the main .jar file
   */
  downloadUrl: string
  /**
   * changelog specified by the mods author(s)
   */
  changelog?: string
  /**
   * supported minecraft versions
   *
   * @example 1.16.1
   * @example 1.2.1
   * @example 20w51a
   * @example 1.17.0
   */
  supportedVersions: string[]

  /**
   * Other mods' IDs required for this
   *
   * don't worry about dependencies for dependencies we will handle that for you
   */
  dependencies?: string[]
}
