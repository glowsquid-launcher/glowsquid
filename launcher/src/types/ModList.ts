/* eslint-disable camelcase */
import ModResult from './ModResult'

export default interface ModList {
  hits: ModResult[];
  offset: number
  limit: number
  total_hits: number
}
