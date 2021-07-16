import { IUser } from 'minecraft-launcher-core'
import { ipcMain, ipcRenderer } from 'electron'
import { TypedIpcMain, TypedIpcRenderer } from 'electron-typed-ipc'
import * as RPC from 'discord-rpc'
import Modpack from './Modpack'
import DownloadProgress from './DownloadProgress'

export type Events = {
    MinecraftClosed: (exitCode: number) => void
    DownloadStatus: (status: DownloadProgress) => void
    DownloadProgress: (progress: Record<string, any>) => void
    PackageExtracted: () => void

}

type Commands = {
    LaunchMinecraft: (modpack: Modpack, user: IUser) => void
    UpdatePresence: (newPresence: RPC.Presence) => void
    GetPath: (
      // eslint-disable-next-line max-len
      name: 'home' | 'appData' | 'userData' | 'cache' | 'temp' | 'exe' | 'module' | 'desktop' | 'documents' | 'downloads' | 'music' | 'pictures' | 'videos' | 'recent' | 'logs' | 'pepperFlashSystemPlugin' | 'crashDumps'
    ) => string
}

export const typedIpcMain = ipcMain as TypedIpcMain<Events, Commands>
export const typedIpcRenderer = ipcRenderer as TypedIpcRenderer<Events, Commands>
