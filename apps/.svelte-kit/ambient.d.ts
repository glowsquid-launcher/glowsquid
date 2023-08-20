
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://kit.svelte.dev/docs/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const SHELL: string;
	export const npm_command: string;
	export const PKG_CONFIG_FOR_TARGET: string;
	export const __ETC_PROFILE_DONE: string;
	export const OBJDUMP_FOR_TARGET: string;
	export const npm_config_userconfig: string;
	export const COLORTERM: string;
	export const ANKI_WAYLAND: string;
	export const SANE_CONFIG_DIR: string;
	export const HYPRLAND_CMD: string;
	export const XDG_CONFIG_DIRS: string;
	export const npm_config_cache: string;
	export const NIX_LD_LIBRARY_PATH: string;
	export const NIX_BUILD_CORES: string;
	export const TERM_PROGRAM_VERSION: string;
	export const WLR_NO_HARDWARE_CURSORS: string;
	export const configureFlags: string;
	export const CLUTTER_BACKEND: string;
	export const XDG_BACKEND: string;
	export const VSCODE_INSPECTOR_OPTIONS: string;
	export const mesonFlags: string;
	export const LAST_EXIT_CODE: string;
	export const QT_WAYLAND_DISABLE_WINDOWDECORATION: string;
	export const npm_config_resolution_mode: string;
	export const shell: string;
	export const SIZE_FOR_TARGET: string;
	export const depsHostHost: string;
	export const NODE: string;
	export const NODE_OPTIONS: string;
	export const AS_FOR_TARGET: string;
	export const DIRS_POSITION: string;
	export const SSH_AUTH_SOCK: string;
	export const DIRENV_DIR: string;
	export const CC_FOR_TARGET: string;
	export const STRINGS: string;
	export const WLR_BACKEND: string;
	export const LD_FOR_TARGET: string;
	export const depsTargetTarget: string;
	export const XCURSOR_PATH: string;
	export const stdenv: string;
	export const COLOR: string;
	export const PKG_CONFIG_PATH_FOR_TARGET: string;
	export const npm_config_local_prefix: string;
	export const builder: string;
	export const LIBVA_DRIVER_NAME: string;
	export const SSH_AGENT_PID: string;
	export const NO_AT_BRIDGE: string;
	export const shellHook: string;
	export const NIX_BINTOOLS_FOR_TARGET: string;
	export const XCURSOR_SIZE: string;
	export const npm_config_globalconfig: string;
	export const GPG_TTY: string;
	export const NIX_LDFLAGS_FOR_TARGET: string;
	export const DIRENV_FILE: string;
	export const XML_CATALOG_FILES: string;
	export const EDITOR: string;
	export const phases: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const XDG_SEAT: string;
	export const PWD: string;
	export const NIX_PROFILES: string;
	export const SOURCE_DATE_EPOCH: string;
	export const LOGNAME: string;
	export const QT_QPA_PLATFORMTHEME: string;
	export const XDG_SESSION_TYPE: string;
	export const NIX_ENFORCE_NO_NATIVE: string;
	export const CUPS_DATADIR: string;
	export const NIX_BINTOOLS_WRAPPER_TARGET_TARGET_x86_64_unknown_linux_gnu: string;
	export const NIX_PATH: string;
	export const TAURI_PLATFORM: string;
	export const npm_config_init_module: string;
	export const LERNA_PACKAGE_NAME: string;
	export const STRIP_FOR_TARGET: string;
	export const NIXPKGS_CONFIG: string;
	export const RANLIB_FOR_TARGET: string;
	export const CXX: string;
	export const _: string;
	export const CMD_DURATION_MS: string;
	export const system: string;
	export const VSCODE_GIT_ASKPASS_NODE: string;
	export const STRINGS_FOR_TARGET: string;
	export const HOST_PATH: string;
	export const NIX_PKG_CONFIG_WRAPPER_TARGET_TARGET_x86_64_unknown_linux_gnu: string;
	export const QT_STYLE_OVERRIDE: string;
	export const IN_NIX_SHELL: string;
	export const doInstallCheck: string;
	export const HOME: string;
	export const NIX_BINTOOLS: string;
	export const GETTEXTDATADIRS: string;
	export const SSH_ASKPASS: string;
	export const WLR_DRM_DEVICES: string;
	export const LANG: string;
	export const NIXOS_OZONE_WL: string;
	export const __GL_VRR_ALLOWED: string;
	export const _JAVA_AWT_WM_NONREPARENTING: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const depsTargetTargetPropagated: string;
	export const npm_package_version: string;
	export const TAURI_PLATFORM_VERSION: string;
	export const DISABLE_QT5_COMPAT: string;
	export const NX_TASK_TARGET_PROJECT: string;
	export const WAYLAND_DISPLAY: string;
	export const cmakeFlags: string;
	export const GIO_EXTRA_MODULES: string;
	export const outputs: string;
	export const NX_LOAD_DOT_ENV_FILES: string;
	export const NIX_STORE: string;
	export const GIT_ASKPASS: string;
	export const NIX_CFLAGS_COMPILE_FOR_TARGET: string;
	export const TAURI_TARGET_TRIPLE: string;
	export const READELF_FOR_TARGET: string;
	export const LD: string;
	export const buildPhase: string;
	export const AR_FOR_TARGET: string;
	export const TAURI_ARCH: string;
	export const DIRENV_DIFF: string;
	export const NX_CLI_SET: string;
	export const INIT_CWD: string;
	export const READELF: string;
	export const CHROME_DESKTOP: string;
	export const GTK_A11Y: string;
	export const QT_QPA_PLATFORM: string;
	export const NIX_USER_PROFILE_DIR: string;
	export const INFOPATH: string;
	export const npm_lifecycle_script: string;
	export const doCheck: string;
	export const VSCODE_GIT_ASKPASS_EXTRA_ARGS: string;
	export const depsBuildBuild: string;
	export const XDG_SESSION_CLASS: string;
	export const TERM: string;
	export const npm_package_name: string;
	export const XDG_DESKTOP_PORTAL_DIR: string;
	export const GTK_PATH: string;
	export const FLAKE: string;
	export const SIZE: string;
	export const propagatedNativeBuildInputs: string;
	export const NX_TASK_HASH: string;
	export const npm_config_prefix: string;
	export const LESSOPEN: string;
	export const USER: string;
	export const strictDeps: string;
	export const NIX_CC_WRAPPER_TARGET_TARGET_x86_64_unknown_linux_gnu: string;
	export const VSCODE_GIT_IPC_HANDLE: string;
	export const DIRENV_LOG_FORMAT: string;
	export const TZDIR: string;
	export const NX_WORKSPACE_ROOT: string;
	export const NIX_LD: string;
	export const AR: string;
	export const __GL_GSYNC_ALLOWED: string;
	export const AS: string;
	export const HYPRLAND_INSTANCE_SIGNATURE: string;
	export const DISPLAY: string;
	export const NIX_BINTOOLS_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu: string;
	export const LOG_FORMAT: string;
	export const npm_lifecycle_event: string;
	export const SHLVL: string;
	export const MOZ_ENABLE_WAYLAND: string;
	export const CXX_FOR_TARGET: string;
	export const NM: string;
	export const PAGER: string;
	export const NIX_CFLAGS_COMPILE: string;
	export const QTWEBKIT_PLUGIN_PATH: string;
	export const patches: string;
	export const __NIXOS_SET_ENVIRONMENT_DONE: string;
	export const XDG_VTNR: string;
	export const NX_INVOKED_BY_RUNNER: string;
	export const buildInputs: string;
	export const NX_TASK_TARGET_TARGET: string;
	export const XDG_SESSION_ID: string;
	export const LOCALE_ARCHIVE: string;
	export const preferLocalBuild: string;
	export const LESSKEYIN_SYSTEM: string;
	export const npm_config_user_agent: string;
	export const WLR_DRM_NO_ATOMIC: string;
	export const TERMINFO_DIRS: string;
	export const GSETTINGS_SCHEMAS_PATH: string;
	export const npm_execpath: string;
	export const LD_LIBRARY_PATH: string;
	export const MOZ_PLUGIN_PATH: string;
	export const XDG_RUNTIME_DIR: string;
	export const NM_FOR_TARGET: string;
	export const OBJCOPY_FOR_TARGET: string;
	export const NODE_PATH: string;
	export const depsBuildTarget: string;
	export const OBJCOPY: string;
	export const out: string;
	export const TAURI_FAMILY: string;
	export const npm_package_json: string;
	export const GREETD_SOCK: string;
	export const KDEDIRS: string;
	export const TAURI_DEBUG: string;
	export const STRIP: string;
	export const VSCODE_GIT_ASKPASS_MAIN: string;
	export const XDG_DATA_DIRS: string;
	export const GDK_BACKEND: string;
	export const LIBEXEC_PATH: string;
	export const OBJDUMP: string;
	export const npm_config_noproxy: string;
	export const PATH: string;
	export const propagatedBuildInputs: string;
	export const __GLX_VENDOR_LIBRARY_NAME: string;
	export const npm_config_metrics_registry: string;
	export const npm_config_node_gyp: string;
	export const dontAddDisableDepTrack: string;
	export const CC: string;
	export const NIX_CC_FOR_TARGET: string;
	export const NIX_CC: string;
	export const GBM_BACKEND: string;
	export const ORIGINAL_XDG_CURRENT_DESKTOP: string;
	export const TAURI_PLATFORM_TYPE: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const depsBuildTargetPropagated: string;
	export const depsBuildBuildPropagated: string;
	export const DIRENV_WATCHES: string;
	export const npm_config_global_prefix: string;
	export const NIX_CC_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu: string;
	export const QT_PLUGIN_PATH: string;
	export const CONFIG_SHELL: string;
	export const __structuredAttrs: string;
	export const GIO_LAUNCHED_DESKTOP_FILE_PID: string;
	export const npm_node_execpath: string;
	export const RANLIB: string;
	export const npm_config_engine_strict: string;
	export const GIO_LAUNCHED_DESKTOP_FILE: string;
	export const NIX_HARDENING_ENABLE: string;
	export const NIX_LDFLAGS: string;
	export const nativeBuildInputs: string;
	export const name: string;
	export const TERM_PROGRAM: string;
	export const depsHostHostPropagated: string;
	export const NX_TERMINAL_OUTPUT_PATH: string;
	export const NX_STREAM_OUTPUT: string;
	export const NX_TASK_TARGET_CONFIGURATION: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://kit.svelte.dev/docs/modules#$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/master/packages/adapter-node) (or running [`vite preview`](https://kit.svelte.dev/docs/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://kit.svelte.dev/docs/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		SHELL: string;
		npm_command: string;
		PKG_CONFIG_FOR_TARGET: string;
		__ETC_PROFILE_DONE: string;
		OBJDUMP_FOR_TARGET: string;
		npm_config_userconfig: string;
		COLORTERM: string;
		ANKI_WAYLAND: string;
		SANE_CONFIG_DIR: string;
		HYPRLAND_CMD: string;
		XDG_CONFIG_DIRS: string;
		npm_config_cache: string;
		NIX_LD_LIBRARY_PATH: string;
		NIX_BUILD_CORES: string;
		TERM_PROGRAM_VERSION: string;
		WLR_NO_HARDWARE_CURSORS: string;
		configureFlags: string;
		CLUTTER_BACKEND: string;
		XDG_BACKEND: string;
		VSCODE_INSPECTOR_OPTIONS: string;
		mesonFlags: string;
		LAST_EXIT_CODE: string;
		QT_WAYLAND_DISABLE_WINDOWDECORATION: string;
		npm_config_resolution_mode: string;
		shell: string;
		SIZE_FOR_TARGET: string;
		depsHostHost: string;
		NODE: string;
		NODE_OPTIONS: string;
		AS_FOR_TARGET: string;
		DIRS_POSITION: string;
		SSH_AUTH_SOCK: string;
		DIRENV_DIR: string;
		CC_FOR_TARGET: string;
		STRINGS: string;
		WLR_BACKEND: string;
		LD_FOR_TARGET: string;
		depsTargetTarget: string;
		XCURSOR_PATH: string;
		stdenv: string;
		COLOR: string;
		PKG_CONFIG_PATH_FOR_TARGET: string;
		npm_config_local_prefix: string;
		builder: string;
		LIBVA_DRIVER_NAME: string;
		SSH_AGENT_PID: string;
		NO_AT_BRIDGE: string;
		shellHook: string;
		NIX_BINTOOLS_FOR_TARGET: string;
		XCURSOR_SIZE: string;
		npm_config_globalconfig: string;
		GPG_TTY: string;
		NIX_LDFLAGS_FOR_TARGET: string;
		DIRENV_FILE: string;
		XML_CATALOG_FILES: string;
		EDITOR: string;
		phases: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		XDG_SEAT: string;
		PWD: string;
		NIX_PROFILES: string;
		SOURCE_DATE_EPOCH: string;
		LOGNAME: string;
		QT_QPA_PLATFORMTHEME: string;
		XDG_SESSION_TYPE: string;
		NIX_ENFORCE_NO_NATIVE: string;
		CUPS_DATADIR: string;
		NIX_BINTOOLS_WRAPPER_TARGET_TARGET_x86_64_unknown_linux_gnu: string;
		NIX_PATH: string;
		TAURI_PLATFORM: string;
		npm_config_init_module: string;
		LERNA_PACKAGE_NAME: string;
		STRIP_FOR_TARGET: string;
		NIXPKGS_CONFIG: string;
		RANLIB_FOR_TARGET: string;
		CXX: string;
		_: string;
		CMD_DURATION_MS: string;
		system: string;
		VSCODE_GIT_ASKPASS_NODE: string;
		STRINGS_FOR_TARGET: string;
		HOST_PATH: string;
		NIX_PKG_CONFIG_WRAPPER_TARGET_TARGET_x86_64_unknown_linux_gnu: string;
		QT_STYLE_OVERRIDE: string;
		IN_NIX_SHELL: string;
		doInstallCheck: string;
		HOME: string;
		NIX_BINTOOLS: string;
		GETTEXTDATADIRS: string;
		SSH_ASKPASS: string;
		WLR_DRM_DEVICES: string;
		LANG: string;
		NIXOS_OZONE_WL: string;
		__GL_VRR_ALLOWED: string;
		_JAVA_AWT_WM_NONREPARENTING: string;
		XDG_CURRENT_DESKTOP: string;
		depsTargetTargetPropagated: string;
		npm_package_version: string;
		TAURI_PLATFORM_VERSION: string;
		DISABLE_QT5_COMPAT: string;
		NX_TASK_TARGET_PROJECT: string;
		WAYLAND_DISPLAY: string;
		cmakeFlags: string;
		GIO_EXTRA_MODULES: string;
		outputs: string;
		NX_LOAD_DOT_ENV_FILES: string;
		NIX_STORE: string;
		GIT_ASKPASS: string;
		NIX_CFLAGS_COMPILE_FOR_TARGET: string;
		TAURI_TARGET_TRIPLE: string;
		READELF_FOR_TARGET: string;
		LD: string;
		buildPhase: string;
		AR_FOR_TARGET: string;
		TAURI_ARCH: string;
		DIRENV_DIFF: string;
		NX_CLI_SET: string;
		INIT_CWD: string;
		READELF: string;
		CHROME_DESKTOP: string;
		GTK_A11Y: string;
		QT_QPA_PLATFORM: string;
		NIX_USER_PROFILE_DIR: string;
		INFOPATH: string;
		npm_lifecycle_script: string;
		doCheck: string;
		VSCODE_GIT_ASKPASS_EXTRA_ARGS: string;
		depsBuildBuild: string;
		XDG_SESSION_CLASS: string;
		TERM: string;
		npm_package_name: string;
		XDG_DESKTOP_PORTAL_DIR: string;
		GTK_PATH: string;
		FLAKE: string;
		SIZE: string;
		propagatedNativeBuildInputs: string;
		NX_TASK_HASH: string;
		npm_config_prefix: string;
		LESSOPEN: string;
		USER: string;
		strictDeps: string;
		NIX_CC_WRAPPER_TARGET_TARGET_x86_64_unknown_linux_gnu: string;
		VSCODE_GIT_IPC_HANDLE: string;
		DIRENV_LOG_FORMAT: string;
		TZDIR: string;
		NX_WORKSPACE_ROOT: string;
		NIX_LD: string;
		AR: string;
		__GL_GSYNC_ALLOWED: string;
		AS: string;
		HYPRLAND_INSTANCE_SIGNATURE: string;
		DISPLAY: string;
		NIX_BINTOOLS_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu: string;
		LOG_FORMAT: string;
		npm_lifecycle_event: string;
		SHLVL: string;
		MOZ_ENABLE_WAYLAND: string;
		CXX_FOR_TARGET: string;
		NM: string;
		PAGER: string;
		NIX_CFLAGS_COMPILE: string;
		QTWEBKIT_PLUGIN_PATH: string;
		patches: string;
		__NIXOS_SET_ENVIRONMENT_DONE: string;
		XDG_VTNR: string;
		NX_INVOKED_BY_RUNNER: string;
		buildInputs: string;
		NX_TASK_TARGET_TARGET: string;
		XDG_SESSION_ID: string;
		LOCALE_ARCHIVE: string;
		preferLocalBuild: string;
		LESSKEYIN_SYSTEM: string;
		npm_config_user_agent: string;
		WLR_DRM_NO_ATOMIC: string;
		TERMINFO_DIRS: string;
		GSETTINGS_SCHEMAS_PATH: string;
		npm_execpath: string;
		LD_LIBRARY_PATH: string;
		MOZ_PLUGIN_PATH: string;
		XDG_RUNTIME_DIR: string;
		NM_FOR_TARGET: string;
		OBJCOPY_FOR_TARGET: string;
		NODE_PATH: string;
		depsBuildTarget: string;
		OBJCOPY: string;
		out: string;
		TAURI_FAMILY: string;
		npm_package_json: string;
		GREETD_SOCK: string;
		KDEDIRS: string;
		TAURI_DEBUG: string;
		STRIP: string;
		VSCODE_GIT_ASKPASS_MAIN: string;
		XDG_DATA_DIRS: string;
		GDK_BACKEND: string;
		LIBEXEC_PATH: string;
		OBJDUMP: string;
		npm_config_noproxy: string;
		PATH: string;
		propagatedBuildInputs: string;
		__GLX_VENDOR_LIBRARY_NAME: string;
		npm_config_metrics_registry: string;
		npm_config_node_gyp: string;
		dontAddDisableDepTrack: string;
		CC: string;
		NIX_CC_FOR_TARGET: string;
		NIX_CC: string;
		GBM_BACKEND: string;
		ORIGINAL_XDG_CURRENT_DESKTOP: string;
		TAURI_PLATFORM_TYPE: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		depsBuildTargetPropagated: string;
		depsBuildBuildPropagated: string;
		DIRENV_WATCHES: string;
		npm_config_global_prefix: string;
		NIX_CC_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu: string;
		QT_PLUGIN_PATH: string;
		CONFIG_SHELL: string;
		__structuredAttrs: string;
		GIO_LAUNCHED_DESKTOP_FILE_PID: string;
		npm_node_execpath: string;
		RANLIB: string;
		npm_config_engine_strict: string;
		GIO_LAUNCHED_DESKTOP_FILE: string;
		NIX_HARDENING_ENABLE: string;
		NIX_LDFLAGS: string;
		nativeBuildInputs: string;
		name: string;
		TERM_PROGRAM: string;
		depsHostHostPropagated: string;
		NX_TERMINAL_OUTPUT_PATH: string;
		NX_STREAM_OUTPUT: string;
		NX_TASK_TARGET_CONFIGURATION: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
