import type { SimpleMod } from '@glowsquid/glowsquid-adapter'
import { Modrinth } from 'modrinth'
import type { ModList } from '../../Types/ModList'
import { config } from '../Config'

export const getModList = async (
  limit: number,
  version: string[],
  platform: string,
  filter: string,
): Promise<SimpleMod[]> => {
  const modrinth = new Modrinth()
  const modlist = await modrinth.api.get<ModList>(
    `mods?limit=${limit}&filter=categories=${platform} AND ${filter}&version=${version
      .map((v) => `versions=${v}`)
      .join(' OR ')}`,
  )

  return await Promise.all(
    modlist.hits.map((hit) => {
      const simpleMod: SimpleMod = {
        id: hit.mod_id,
        platform: config.platform,
        iconUrl: hit.icon_url,
        latestVersion: hit.latest_version,
        shortDescription: hit.description,
        slug: hit.slug,
        title: hit.title,
      }

      return simpleMod
    }),
  )
}
