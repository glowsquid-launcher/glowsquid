export interface SimpleMod {
  /**
   * The platform this mod originated from. This <b>needs</b> be the id specified in your adapter config
   */
  platform: string
  /**
   * The unique ID for this mod
   */
  id: string
  /**
   * human-readable title for this mod
   */
  title: string
  /**
   * the Icon URL for this mod
   */
  iconUrl: string
  /**
   * The mod's slug - it's unique worded identifier used in the url, e.g. `fabric-for-fabric`
   */
  slug: string
  /**
   * A short, simple description of this mod in 1-2 sentences
   */
  shortDescription: string
  /**
   * The latest version ID for this mod
   *
   * Used for quick downloading
   */
  latestVersion: string
}
