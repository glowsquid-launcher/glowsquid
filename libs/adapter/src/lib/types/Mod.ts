export interface Mod {
  /**
   * The platform this mod originated from. This **needs** be the id specified in your adapter config
   */
  platform: string;
  /**
   * The mod's ID or identifier according to the platform
   */
  id: string;
  /**
   * The mod's slug - it's unique worded identifier used in the url
   * * @example moundertweaks
   */
  slug: string;
  /**
   * The mod's authors' IDs
   */
  authors: string[];
  /**
   * All the version IDs of this mod, as specified in the {@link ModVersion}
   * @see ModVersion
   */
  versions: string[];
  /**
   * A human-readable title of this mod
   */
  title: string;
  /**
   * A short, plaintext description of this mod
   */
  description: string;
  /**
   * HTML-rich description of the mod
   */
  body: string;
  /**
   * The icon URL for this mod
   */
  iconUrl: string;
}
