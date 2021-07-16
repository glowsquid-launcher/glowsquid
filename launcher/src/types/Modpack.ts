import type { IOverrides } from 'minecraft-launcher-core'
import type ModFile from './ModFile'

export interface CustomOptions {
  memory: {
    max: string | number;
    min: string | number;
  };
  window?: {
    width?: number;
    height?: number;
    fullscreen?: boolean;
  };
  overrides?: IOverrides;
}

export default interface Modpack {
  formatVersion: 1
  versionId: string
  name: string
  summary?: string
  description?: string
  releaseDate?: string
  files: ModFile[]
  dependencies: {
    minecraft: string
    'fabric-loader'?: string
    forge?: string
  }
  extras?: CustomOptions
}
