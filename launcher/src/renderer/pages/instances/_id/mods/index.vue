<template>
  <div>
    <transition name="slide-y-transition" appear>
      <v-text-field
        v-if="!leaving"
        v-model="searchQuery"
        background-color="#2b2b2b"
        :placeholder="$t('pages.mods.search')"
        class="mr-4 ml-4 mt-2 sticky z-20 elevation"
        style="top: 60px;"
        :hint="$t('pages.mods.hint')"
        solo
        dense
      >
        <template #prepend-inner>
          <v-icon>
            mdi-magnify
          </v-icon>
        </template>
      </v-text-field>
    </transition>
    <div
      class="grid sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-2"
    >
      <div v-for="mod in modList.hits" :key="mod.mod_id">
        <v-card height="100%" class="card-outer">
          <v-card-title>
            <transition name="slide-y-transition" appear duration="100">
              <h1 v-if="!leaving" class="flex flex-row mb-2">
                <v-img :max-height="32" :max-width="32" :src="mod.icon_url" class="mr-3 rounded-sm" />
                <p class="h-full ma-0 align-middle">{{ mod.title }}</p>
              </h1>
            </transition>
          </v-card-title>
          <v-card-subtitle>
            <transition name="slide-x-transition" appear duration="100">
              <p v-if="!leaving">by {{ mod.author }}</p>
            </transition>
          </v-card-subtitle>
          <v-card-text>
            <transition name="slide-x-transition" appear duration="100">
              <p v-if="!leaving">{{ mod.description }}</p>
            </transition>
          </v-card-text>
          <v-card-actions class="card-actions w-full">
            <transition name="slide-y-reverse-transition" appear duration="100">
              <div
                v-if="!leaving"
                class="grid-cols-2 gap-1 justify-center w-full"
                style="display: grid !important; "
              >
                <v-btn block @click="$router.push({
                  path: localePath(`/instances/${$route.params.id}/mods/${mod.mod_id.replace('local-', '') }`)
                })"
                >
                  {{ $t('pages.mods.about') }}
                </v-btn>
                <v-btn block :disabled="mod.alreadyInstalled" @click="downloadLatestSupportedVersion(mod)">
                  {{ $t('pages.mods.install') }}
                </v-btn>
              </div>
            </transition>
          </v-card-actions>
        </v-card>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import path from 'path'
import fs from 'fs'
import { getModule } from 'vuex-module-decorators'
import Vue from 'vue'
import ModList from '../../../../../types/ModList'
import ModResult from '../../../../../types/ModResult'
import { typedIpcRenderer } from '../../../../../types/Ipc'
import Modpack from '../../../../../types/Modpack'
import InstancesModule from '~/store/instances'
import ModVersion from '~/../types/ModVersion'

type NewModResult = ModResult & {
  alreadyInstalled: boolean
}

interface NewModList {
  hits: NewModResult[];
  offset: number
  limit: number
  // eslint-disable-next-line camelcase
  total_hits: number
}

export default Vue.extend({
  beforeRouteLeave (_, _2, next) {
    this.leaving = true
    setTimeout(() => {
      next()
    }, 100)
  },
  data () {
    return {
      instanceStore: getModule(InstancesModule, this.$store),
      instance: getModule(InstancesModule, this.$store).instances.find(v => v.name === this.$route.params.id),
      searchQuery: '',
      leaving: false,
      modList: {} as NewModList
    }
  },
  async fetch () {
    // eslint-disable-next-line max-len
    const instance = getModule(InstancesModule, this.$store).instances.find(v => v.name === this.$route.params.id)
    if (!instance) return

    const modList = await this.$axios.$get<ModList>(
      `https://api.modrinth.com/api/v1/mod?filters=categories=fabric&versions=${this.instance?.dependencies.minecraft}`
    )

    const newHits: NewModResult[] = await Promise.all(modList.hits.map(async hit => {
      const newHit: NewModResult = {
        ...hit,
        alreadyInstalled: await checkIfInstalled()
      }

      async function checkIfInstalled () {
        const instanceJsonPath = path.join(
          await typedIpcRenderer.invoke('GetPath', 'userData'),
          'instances', instance!!.name, 'instance.json'
        )

        const instanceJson: Modpack = JSON.parse(fs.readFileSync(instanceJsonPath).toString())
        return instanceJson.files.some(file => file.id === hit.mod_id.replace('local-', ''))
      }

      return newHit
    }))

    this.modList = {
      ...modList,
      hits: newHits
    }
  },
  watch: {
    async searchQuery (newQuery) {
      // eslint-disable-next-line max-len
      this.modList = await this.$axios.$get(`https://api.modrinth.com/api/v1/mod?${newQuery ? `query=${newQuery}` : ''}&filters=categories=fabric&versions=${this.instance?.dependencies.minecraft}`)
    }
  },
  methods: {
    async downloadLatestSupportedVersion (mod: ModResult) {
      const modVersions =
      // eslint-disable-next-line max-len
      (await this.$axios.$get<ModVersion[]>(`https://api.modrinth.com/api/v1/mod/${mod.mod_id.replace('local-', '')}/version`))

      const filteredVersions = modVersions.filter(v => v.game_versions.some(gameVersion => {
        if (gameVersion === this.instance!!.dependencies.minecraft) return true
        // we do the instance version instead as we know it's always 3 `.`s
        const gameVerNoMinor = this.instance!!.dependencies.minecraft.split('.')
        gameVerNoMinor.pop()
        return gameVersion === gameVerNoMinor.join('.')
      }))

      await this.instanceStore.DOWNLOAD_MOD({
        instance: this.instance!!,
        mod: filteredVersions[0].files[0],
        deps: filteredVersions[0].dependencies,
        id: modVersions[0].mod_id
      })
    }
  }
})
</script>

<style lang="stylus">
.card-outer {
  position: relative
  padding-bottom: 50px
}
.card-actions {
  position: absolute
  bottom: 0
}
</style>
