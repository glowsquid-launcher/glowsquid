export interface User {
  /**
   * The platform this user came from. This **needs**to be the platform specified in your adapter config
   */
  platform: string;
  /**
   * The users ID
   */
  id: string;
  /**
   * the users username
   */
  username: string;
  /**
   * The users icon url
   */
  iconUrl: string;
}
