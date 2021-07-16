import type { Mod } from '@glowsquid/glowsquid-adapter'
import { Modrinth } from 'modrinth'
import { config } from '../Config'

export const getMod = async (modId: string): Promise<Mod> => {
  const modrinth = new Modrinth()

  const mod = await modrinth.mod(modId)
  const authors = await modrinth.api.get<{ user_id: string }[]>(
    `teams/${mod.team}/members`,
  )

  const authorsIds = authors.map((author) => author.user_id)
  const versionIds = (await mod.versions()).map((version) => version.id)

  return {
    id: mod.id,
    title: mod.title,
    description: mod.description,
    body: mod.body,
    platform: config.platform,
    slug: mod.slug ?? mod.id,
    authors: authorsIds,
    // default to modrinth icon if no icon exists for some reason
    iconUrl: mod.icon_url ?? config.iconUrl,
    versions: versionIds,
  }
}
