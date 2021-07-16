import type { User } from '@glowsquid/glowsquid-adapter'
import { Modrinth } from 'modrinth'
import { config } from '../Config'

export const getUser = async (userId: string): Promise<User> => {
  const modrinth = new Modrinth()
  const user = await modrinth.user(userId)

  return {
    id: user.id,
    platform: config.platform,
    iconUrl: user.avatar_url,
    username: user.username,
  }
}
