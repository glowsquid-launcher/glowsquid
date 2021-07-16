import { SimpleMod } from './SimpleMod'
import { Mod } from './Mod'
import { User } from './User'
import { ModVersion } from './ModVersion'
import { AdapterConfig } from './AdapterConfig'

export interface Adapter {
  /**
   * the config for this adapter, used internally to map adapters to mods
   */
  config: AdapterConfig

  /**
   * Gets a list of all mods on this platform
   *
   * @param limit the amount of mods this should return (we don't want to destroy the platform :) )
   * @param version the minecraft version(s) the mod should support
   * @param platform the modding platform the mod should support
   * @param filter a filter-by-name thing that filters by the mod name
   */
  getModList(
    limit: number,
    version: string[],
    platform: string,
    filter: string,
  ): Promise<SimpleMod[]>

  /**
   * all info for a specific mod
   *
   * @param modId the ID for the mod
   *
   * if no mod can be found this should throw an error
   */
  getMod(modId: string): Promise<Mod>

  /**
   * Gets a user
   *
   * @param userId the users ID
   *
   * If no user is found this should throw an error
   */
  getUser(userId: string): Promise<User>

  /**
   * Gets all mod authors for this mod
   *
   * @param modId the mods ID
   *
   * If the mod is not found this should return an error
   * Hint: use the {@link getUser} function to reduce code duplication
   */
  getModAuthors(modId: string): Promise<User[]>

  /**
   * Gets info on a version of a mod
   *
   * @param modId the ID of the mod this version belongs to (some adapters don't need this)
   * @param versionId the version ID
   *
   * If no version is found this should return an error
   */
  getVersion(modId: string, versionId: string): Promise<ModVersion>

  /**
   * Gets all versions for a mod
   *
   * @param modId the mods ID
   *
   * If the mod isn't found this should return an error
   * Hint: use the {@link getVersion} function to reduce code duplication
   */
  getAllVersions(modId: string): Promise<ModVersion[]>
}
