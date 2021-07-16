import type { User } from '@glowsquid/glowsquid-adapter'
import { Modrinth } from 'modrinth'
import { config } from '../Config'
import { getMod } from './getMod'

export const getModAuthors = async (modId: string): Promise<User[]> => {
  const modrinth = new Modrinth()

  const mod = await getMod(modId)
  const users = await modrinth.users(mod.authors)

  return users.map<User>((user) => {
    return {
      id: user.id,
      platform: config.platform,
      username: user.username,
      iconUrl: user.avatar_url,
    }
  })
}
