import type { Mod } from '/adapters/packages/glowsquid-adapter'
import curseforge from 'mc-curseforge-api'
import { config } from '../Adapter/Config'
import {platform} from "os";

export const getMod = async (modId: string): Promise<Mod> => {
  const mod = await curseforge.getMod(Number(modId))

  const versions: string[] = []
  await curseforge.getModFiles(mod.id).then((files) => {
    for (const file of files) { versions.push(String(file.id)) }
  })

  const authors = []
  const keys = mod.authors.keys();
  for (const author in keys) { authors.push(author) }

  const description = await mod.getDescription()

  return {
    id: String(mod.id),
    title: mod.name,
    description: mod.summary,
    body: description,
    platform: config.platform,
    slug: mod.key,
    authors: authors,
    iconUrl: mod.attachments[0]['thumbnailUrl'],
    versions: versions,
  }
}
