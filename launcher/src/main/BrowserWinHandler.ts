/* eslint-disable no-unused-vars */
/* eslint-disable @typescript-eslint/no-unused-vars */
import { EventEmitter } from 'events'
import { BrowserWindow, app } from 'electron'
import * as RPC from 'discord-rpc'
import ElectronStore from 'electron-store'
import { typedIpcMain } from '../types/Ipc'
import { launchMinecraft } from './utils/launchMinecraft'

export default class BrowserWinHandler {
  browserWindow: BrowserWindow
  _eventEmitter: EventEmitter
  options: Record<any, any>
  allowRecreate: boolean
  /**
     * @param [options] {object} - browser window options
     * @param [allowRecreate] {boolean}
     */
  constructor (options: Record<any, any>, allowRecreate: boolean = true) {
    this._eventEmitter = new EventEmitter()
    this.allowRecreate = allowRecreate
    this.options = options
    this.browserWindow = null
    this._createInstance()
  }

  _createInstance () {
    // This method will be called when Electron has finished
    // initialization and is ready to create browser windows.
    // Some APIs can only be used after this event occurs.
    app.on('ready', () => {
      const store = new ElectronStore()

      const client = new RPC.Client({
        transport: 'ipc'
      })

      let prevActivity: RPC.Presence = {
        details: 'Looking around 👀',
        state: 'Not signed in yet',
        startTimestamp: new Date(),
        largeImageKey: 'glowsquid',
        largeImageText: 'Coming not soon™'
      }

      client.on('ready', async () => {
        await client.setActivity({
          details: 'Looking around 👀',
          state: 'Not signed in yet',
          startTimestamp: new Date(),
          largeImageKey: 'glowsquid',
          largeImageText: 'Coming not soon™'
        })

        console.log('rpc now active')
      })

      client.login({
        clientId: '795736067675258891'
      }).then(r => r)

      this._create()

      typedIpcMain.handle('UpdatePresence', async (_e, presence) => {
        await client.setActivity({
          ...prevActivity,
          ...presence
        })

        prevActivity = {
          ...prevActivity,
          ...presence
        }
      })

      typedIpcMain.handle('LaunchMinecraft', async (_e, modpack, user) => await launchMinecraft(modpack, user))

      typedIpcMain.handle('GetPath', (_e, name) => { return app.getPath(name) })
    })

    // On macOS, it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (!this.allowRecreate) return
    app.on('activate', () => this._recreate())
  }

  _create () {
    this.browserWindow = new BrowserWindow(
      {
        ...this.options,
        webPreferences: {
          ...this.options.webPreferences,
          webSecurity: false,
          nodeIntegration: true, // allow loading modules via the require () function
          devTools: !process.env.SPECTRON, // disable on e2e test environment
          enableRemoteModule: false
        }
      }
    )

    this.browserWindow.on('closed', () => {
      // Dereference the window object
      this.browserWindow = null
    })

    this._eventEmitter.emit('created')
  }

  _recreate () {
    if (this.browserWindow === null) this._create()
  }

  /**
     * @callback onReadyCallback
     * @param {BrowserWindow}
     */

  /**
     *
     * @param callback
     */
  onCreated (callback: { (browserWindow: any): void; (browserWindow: any): void; (arg0: BrowserWindow): void }) {
    this._eventEmitter.once('created', () => {
      callback(this.browserWindow)
    })
  }

  /**
     *
     * @returns {Promise<BrowserWindow>}
     */
  created () {
    return new Promise(resolve => {
      this._eventEmitter.once('created', () => {
        resolve(this.browserWindow)
      })
    })
  }
}
