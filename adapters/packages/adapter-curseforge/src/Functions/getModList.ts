import { SimpleMod } from '@glowsquid/glowsquid-adapter'
import { getMods } from 'mc-curseforge-api'
import { config } from '../Adapter/Config'

export const getModList = async (
  limit: number,
  version: string[],
  platform: string,
  filter: string,
): Promise<SimpleMod[]> => {
  const modlist = await getMods({
    pageSize: limit,
    gameVersion: version[0],
    searchFilter: filter,
  })
  if (modlist instanceof Error) throw modlist

  return modlist.map<SimpleMod>((mod) => {
    return {
      id: mod.id.toString(),
      platform: config.platform,
      // quicktype ftw :)
      iconUrl: mod.logo.url as string,
      title: mod.name,
      slug: mod.key,
      shortDescription: mod.summary,
      latestVersion: mod.latestFiles[0].id.toString(),
    }
  })
}
