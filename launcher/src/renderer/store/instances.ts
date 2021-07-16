/* eslint-disable max-len */
import { promises as fs, existsSync, createWriteStream } from 'fs'
import { get } from 'https'
import * as path from 'path'
import { store } from '@/plugins/store'
import { Action, getModule, Module, Mutation, VuexModule } from 'vuex-module-decorators'
import { mkdir, rm } from 'shelljs'
import axios from 'axios'
import FabricLoaderVersion from '@/../types/FabricLoaderVersion'
import FabricVersion from '@/../types/FabricVersion'
import { Store } from 'vuex/types/index'
import { typedIpcRenderer } from '../../types/Ipc'
import UiModule from './ui'
import Modpack from '~/../types/Modpack'
import ModVersion, { File } from '~/../types/ModVersion'
import ModFile from '~/../types/ModFile'

type AddInstanceType = {
  name: string,
  fabricLoaderVersion: FabricLoaderVersion,
  fabricLoader: FabricVersion,
  ram: {
    min: string,
    max: string
  },
  assetRoot?: string,
  store: Store<any>
}

@Module({
  name: 'instances',
  stateFactory: true,
  namespaced: true
})
export default class InstancesModule extends VuexModule {
  instances: Modpack[] = store.get('instances', [])

  @Mutation
  PUSH_INSTANCE (instance: Modpack) {
    this.instances.push(instance)
    store.set('instances', this.instances)
  }

  @Mutation
  RE_ADD_INSTANCES (instances: Modpack[]) {
    this.instances = instances
    store.set('instances', this.instances)
  }

  @Mutation
  PUSH_MOD ({ mod, instance }: {mod: ModFile, instance: Modpack}) {
    this.instances[this.instances.indexOf(instance)].files.push(mod)
  }

   @Action
  async ADD_INSTANCE ({ name, fabricLoader, fabricLoaderVersion, ram, assetRoot, store }: AddInstanceType) {
    const userData = await typedIpcRenderer.invoke('GetPath', 'userData')
    // calculate weather to add .0 or not
    const splitLoader = fabricLoader.version.split('.')
    splitLoader.pop()
    const minecraftVersion = splitLoader.length === 2 ? fabricLoader.version : `${fabricLoader.version}.0`

    const uiStore = getModule(UiModule, store)
    const version: Modpack = {
      name,
      summary: 'no summary yet',
      description: 'no description yet',
      versionId: '0.0.1',
      releaseDate: new Date().toISOString(),
      formatVersion: 1,
      dependencies: {
        minecraft: minecraftVersion,
        'fabric-loader': fabricLoaderVersion.version
      },
      files: [],
      extras: {
        memory: ram,
        overrides: {
          assetRoot
        }
      }
    }

    mkdir('-p', path.join(userData, 'instances', name, '.minecraft', 'versions', fabricLoader.version + 'fabric'))
    mkdir('-p', path.join(userData, 'instances', name, '.minecraft', 'mods'))

    const content = JSON.stringify((await axios.get(`https://fabricmc.net/download/technic?yarn=${fabricLoader.version}${fabricLoaderVersion.separator}${fabricLoaderVersion.build}&loader=${fabricLoaderVersion.version}`)).data)

    await fs.writeFile(
      path.join(userData, 'instances', name, 'instance.json'),
      JSON.stringify(version))
    await fs.writeFile(path.join(userData, 'instances', name, '.minecraft', 'versions', fabricLoader.version + 'fabric', fabricLoader.version + 'fabric.json'),
      content)

    this.context.commit('PUSH_INSTANCE', version)
    uiStore.TOGGLE_ADD_INSTANCE_MODAL()
  }

  @Action
   async REFRESH_INSTANCES () {
     const userData = await typedIpcRenderer.invoke('GetPath', 'userData')

     // fs.stat breaks for some reason
     if (!existsSync(path.join(userData, 'instances'))) mkdir('-p', path.join(userData, 'instances'))

     const instancePaths = (await fs.readdir(path.join(userData, 'instances'), { withFileTypes: true }))
       .filter(dirent => dirent.isDirectory())
       .map(dirent => path.join(userData, 'instances', dirent.name, 'instance.json'))

     this.context.commit('RE_ADD_INSTANCES', await Promise.all(instancePaths.map(async instancePath =>
       JSON.parse((await fs.readFile(instancePath)).toString('utf-8'))
     )))
   }

  @Action
  async DELETE_INSTANCE (instance: Modpack) {
    const userData = await typedIpcRenderer.invoke('GetPath', 'userData')

    const folderToDelete = path.join(userData, 'instances', instance.name)
    if (existsSync(folderToDelete)) {
      rm('-rf', folderToDelete)
      await this.context.dispatch('REFRESH_INSTANCES')
    }
  }

  @Action
  async DOWNLOAD_MOD ({ mod, instance, deps, id }: {mod: File, instance: Modpack, deps: string[], id: string}) {
    const userData = await typedIpcRenderer.invoke('GetPath', 'userData')

    for (const dep in deps) {
      const modVersions =
      // eslint-disable-next-line max-len
      (await axios.get<ModVersion[]>(`https://api.modrinth.com/api/v1/mod/${dep.replace('local-', '')}/version`))
        .data

      const filteredVersions = modVersions.filter(v => v.game_versions.some(gameVer => {
        if (gameVer === instance.dependencies.minecraft) return true

        const gameVerNoMinor = instance.dependencies.minecraft.split('.')
        gameVerNoMinor.pop()
        return instance.dependencies.minecraft === gameVerNoMinor.join('.')
      }))

      await this.context.dispatch('DOWNLOAD_MOD', {
        instance,
        mod: filteredVersions[0].files[0],
        deps: filteredVersions[0].dependencies,
        id: filteredVersions[0].mod_id
      })
    }

    download(mod.url, path.join(userData, 'instances', instance.name, '.minecraft', 'mods', mod.filename), async err => {
      if (!err) {
        console.log('done downloading, adding to instance.json')

        const modFile: ModFile = {
          id,
          path: `mods/${mod.filename}`,
          downloads: [
            mod.filename
          ]
        }

        const instancePath = path.join(userData, 'instances', instance.name, 'instance.json')

        const newJson: Modpack = JSON.parse((await fs.readFile(instancePath)).toString())
        newJson.files.push(modFile)
        await fs.writeFile(instancePath, JSON.stringify(newJson))
        this.PUSH_MOD({ mod: modFile, instance })
      }
    })
  }
}

function download (url: string, dest: string, cb: (err?: string) => void) {
  const file = createWriteStream(dest)

  get(url, function (response) {
    response.pipe(file)
    file.on('finish', function () {
      file.close()
      cb()
    })
  }).on('error', async function (err) {
    await fs.unlink(dest)
    if (cb) cb(err.message)
  })
}
