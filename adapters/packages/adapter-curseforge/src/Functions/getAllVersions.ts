import { ModVersion } from '@glowsquid/glowsquid-adapter'
import { getMod } from './getMod'
import { getVersion } from './getVersion'

export const getAllVersions = async (modId: string): Promise<ModVersion> => {
  const mod = await getMod(modId)

  return await Promise.all(
    mod.versions.map(async (modVer: string) => await getVersion(modId, modVer))
  )
}
