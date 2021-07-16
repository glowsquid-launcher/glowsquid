import { sources } from './Sources'
import { Mod, ModVersion, SimpleMod, User } from './types'

function checkIfAdapterExists(adapterID: string) {
  const source = sources.find((source) => source.config.id === adapterID)
  if (!source) throw new Error('source adapter cannot be found')
  return source
}

export async function getAllMods(
  limitPerAdapter: number,
  version: string[],
  platform: string,
  filter: string,
): Promise<SimpleMod[]> {
  return (
    await Promise.all(
      sources.map((source) =>
        source.getModList(limitPerAdapter, version, platform, filter),
      ),
    )
  ).flat()
}

export async function getMod(adapter: string, modID: string): Promise<Mod> {
  const source = checkIfAdapterExists(adapter)
  return await source.getMod(modID)
}

export async function getModAuthors(
  adapter: string,
  modId: string,
): Promise<User[]> {
  const source = checkIfAdapterExists(adapter)
  return await source.getModAuthors(modId)
}

export async function getAllVersions(
  adapter: string,
  modId: string,
): Promise<ModVersion[]> {
  const source = checkIfAdapterExists(adapter)
  return await source.getAllVersions(modId)
}

/**
 * This gets a list of ALL dependencies for a mod recursively and filters out duplicates because {@link Set}s are awesome
 * @param adapter the adapter the mod came from
 * @param versionId the mods ID
 */
export async function getDependencyList(
  adapter: string,
  versionId: string,
): Promise<string[]> {
  const source = checkIfAdapterExists(adapter)
  const version = await source.getVersion(versionId)
  if (!version.dependencies) return []
  const dependencyList = [...version.dependencies]

  for (const dep of dependencyList) {
    dependencyList.push(...(await getDependencyList(adapter, dep)))
  }

  return [...new Set(dependencyList)]
}
