import type { Adapter } from '@glowsquid/glowsquid-adapter'
import {
  config,
  getAllVersions,
  getMod,
  getModAuthors,
  getModList,
  getUser,
  getVersion,
} from './Adapter'

const adapter: Adapter = {
  config,
  getAllVersions,
  getVersion,
  getModAuthors,
  getModList,
  getMod,
  getUser,
}

export default adapter
