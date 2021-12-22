import type { Adapter } from '@glowsquid/glowsquid-adapter'
import {
  config,
  getAllVersions,
  getMod,
  getModAuthors,
  getModList,
  getVersion,
} from './Adapter'

const adapter: Adapter = {
  config,
  getAllVersions,
  getVersion,
  getModAuthors,
  getModList,
  getMod,
}

export default adapter
