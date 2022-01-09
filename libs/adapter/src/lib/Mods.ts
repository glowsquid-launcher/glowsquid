import { sources } from './Sources';
import { Mod, ModVersion, SimpleMod, User } from './types';

function getAdapter(adapterID: string) {
  const source = sources.find((source) => source.config.platform === adapterID);
  if (!source) throw new Error('source adapter cannot be found');
  return source;
}

export async function getAllMods(
  limitPerAdapter: number,
  version: string[],
  platform: string,
  filter: string
): Promise<SimpleMod[]> {
  return (
    await Promise.all(
      sources.map((source) =>
        source.getModList(limitPerAdapter, version, platform, filter)
      )
    )
  ).flat();
}

export async function getMod(adapter: string, modID: string): Promise<Mod> {
  const source = getAdapter(adapter);
  return await source.getMod(modID);
}

export async function getModAuthors(
  adapter: string,
  modId: string
): Promise<User[]> {
  const source = getAdapter(adapter);
  return await source.getModAuthors(modId);
}

export async function getAllVersions(
  adapter: string,
  modId: string
): Promise<ModVersion[]> {
  const source = getAdapter(adapter);
  return await source.getAllVersions(modId);
}

/**
 * This gets a list of ALL dependencies for a mod recursively and filters out duplicates.
 *
 * @param adapter the adapter the mod came from
 * @param modId the mods ID
 * @param versionId the version of the mod
 */
export async function getDependencyList(
  adapter: string,
  modId: string,
  versionId: string
): Promise<
  {
    modId: string;
    versionId: string;
  }[]
> {
  const source = getAdapter(adapter);
  const version = await source.getVersion(modId, versionId);
  if (!version.dependencies) return [];
  const dependencyList = version.dependencies;

  for (const dep of dependencyList) {
    const depListForDep = await getDependencyList(
      adapter,
      dep.modId,
      dep.versionId
    );

    dependencyList.push(...depListForDep);
  }

  return dependencyList.filter(
    (value, index, self) =>
      index ===
      self.findIndex(
        (mod) => mod.modId === value.modId && mod.versionId === value.versionId
      )
  );
}
