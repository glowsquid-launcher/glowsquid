import * as path from 'path'
import { Plugin } from '@nuxt/types'
import { typedIpcRenderer } from '../../types/Ipc'

const plugin: Plugin = async (_, inject) => {
  inject('$getInstancesPath',
    async (instanceName: string) =>
      path.join(await typedIpcRenderer.invoke('GetPath', 'userData'), 'instances', instanceName)
  )
}

export default plugin
