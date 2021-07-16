import type { ModVersion } from "@glowsquid/glowsquid-adapter";
import curseforge from 'mc-curseforge-api'

export const getVersion = async (modId: string, versionId: string): Promise<ModVersion> => {
  const versions = await curseforge.getModFiles(Number(modId))
  const version = versions.find(file => file.id == Number(versionId))

  if (version == undefined) { throw Error("Version not found.") }

  return {
    id: String(version.id),
    name: version.file_name,
    downloadUrl: version.download_url,
    supportedVersions: version.minecraft_versions,
    dependencies: version.mod_dependencies
  }
}
