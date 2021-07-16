import type { Mod } from '@glowsquid/glowsquid-adapter'
import { getMod } from './getMod'

export const getModAuthors = async (modId: string): Promise<string[]> => {
  const mod: Mod | undefined = await getMod(modId)
  if (mod == undefined) {throw Error("Mod not found.")}
  return mod.authors
}
