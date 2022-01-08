export interface Instance {
	config: InstanceConfig
	/**
	 * settings to use when launching minecraft
	 */
	settings: Settings
}

/**
 * An instance config file
 *
 * This is based on the modrinth {@link https://docs.modrinth.com/docs/modpacks/format_definition/ modpack spec}
 */
export interface InstanceConfig {
	/**
	 * The version of the format, stored as a number. The current value at the time of writing is 1.
	 */
	formatVersion: 1
	/**
	 * The game of the modpack, stored as a string. The current available types are: 'minecraft'
	 */
	game: 'minecraft'
	/**
	 * A unique identifier for this specific version of the modpack.
	 */
	versionId: string

	/**
	 * Human-readable name of the modpack.
	 */
	name: string

	/**
	 *A short description of this modpack.
	 */
	summary?: string

	/**
	 * A list of files for the modpack that needs to be downloaded.
	 */
	files: File[]

	dependencies: GameDependencies
}

export interface Settings {
	ram: RamSettings
}

export interface RamSettings {
	min: string
	max: string
}

export interface File {
	/**
	 * The destination path of this file, relative to the Minecraft Instance directory.
	 *
	 * @example mods/MyMod.jar resolves to .minecraft/mods/MyMod.jar.
	 */
	path: string

	/**
	 * The hashes of the file specified.
	 */
	hashes: Hashes

	/**
	 * For files that only exist on a specific environment, this field allows that to be specified.
	 */
	env: Environment

	/**
	 * An array containing URLs where this file may be downloaded.
	 */
	downloads: string[]
}

export interface Hashes {
	sha1?: string
	sha256?: string
}

type EnvironmentType = 'required' | 'optional' | 'unsupported'
export interface Environment {
	client: EnvironmentType
	/**
	 * This is for a _dedicated_ server. Even though clients technically have a logical server, if something is marked server only, it should not be installed on the client.
	 */
	server: EnvironmentType
}

export interface GameDependencies {
	minecraft: string
	'fabric-loader': string
	forge: string
}
