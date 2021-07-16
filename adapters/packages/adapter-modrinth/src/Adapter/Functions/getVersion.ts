import type { ModVersion } from '@glowsquid/glowsquid-adapter'
import { Modrinth } from 'modrinth'

export const getVersion = async (modId: string, versionId: string): Promise<ModVersion> => {
  const modrinth = new Modrinth()
  const version = await modrinth.version(versionId)

  return {
    id: version.id,
    name: version.name,
    changelog: version.changelog,
    downloadUrl: version.files[0].url,
    number: version.version_number,
    sha1: version.files[0].hashes.sha1,
    supportedVersions: version.game_versions,
    // We checked modrinth api
    dependencies: version.dependencies as string[],
  }
}
