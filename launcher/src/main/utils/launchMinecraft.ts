import path from 'path'
import { Client, IUser } from 'minecraft-launcher-core'
import { app, webContents } from 'electron'
import { TypedWebContents } from 'electron-typed-ipc'
import Modpack from '~/../types/Modpack'
import { Events } from '~/../types/Ipc'

// TODO: impl launching minecraft
// eslint-disable-next-line
export async function launchMinecraft (instance :Modpack, user: IUser) {
  const client = new Client()

  client.on('close', exitCode =>
    webContents.getAllWebContents().forEach((renderer: TypedWebContents<Events>) =>
      renderer.send('MinecraftClosed', exitCode)
    ))

  client.on('download-progress', progress =>
    webContents.getAllWebContents().forEach((renderer: TypedWebContents<Events>) =>
      renderer.send('DownloadProgress', progress)
    ))

  client.on('download-status', status =>
    webContents.getAllWebContents().forEach((renderer: TypedWebContents<Events>) =>
      renderer.send('DownloadStatus', status)
    ))

  client.on('package-extract', () =>
    webContents.getAllWebContents().forEach((renderer: TypedWebContents<Events>) =>
      renderer.send('PackageExtracted')
    ))

  await client.launch({
    authorization: (async () => user)(),
    root: path.join(app.getPath('userData'), 'instances', instance.name, '.minecraft'),
    version: {
      number: instance.dependencies.minecraft,
      type: 'release',
      custom: `${instance.dependencies.minecraft}fabric`
    },
    memory: instance.extras?.memory,
    overrides: {
      assetRoot: instance.extras?.overrides?.assetRoot ?? path.join(app.getPath('userData'), 'assets')
    }
  })
}
