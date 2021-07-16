export enum Separator {
  Build = '+build.',
  Empty = '.',
}

export default interface FabricLoaderVersion {
  separator: Separator;
  build: number;
  maven: string;
  version: string;
  stable: boolean;
}
