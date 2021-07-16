<!-- eslint-disable max-len -->
<template>
  <div>
    <transition name="slide-y-transition" appear duration="100">
      <v-toolbar v-if="!leaving" rounded="lg">
        <v-text-field
          v-model="filter"
          background-color="#272727"
          :placeholder="$t('pages.instances.search')"
          class="mr-4 mt-7"
          solo
        >
          <template #prepend-inner>
            <v-icon>
              mdi-magnify
            </v-icon>
          </template>
        </v-text-field>
        <v-btn class="mr-2" min-width="36px" width="36px" @click="addInstance()">
          <v-icon>mdi-plus-circle-outline</v-icon>
        </v-btn>
        <v-btn class="mr-2" min-width="36px" width="36px" @click="refresh">
          <v-icon>mdi-refresh-circle</v-icon>
        </v-btn>
      </v-toolbar>
    </transition>
    <div
      v-if="!useList"
      class="grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-2 justify-center mt-4 ml-3"
      :style="`margin-right: ${selectedInstance ? '18rem' : '16px' };`"
    >
      <transition
        v-for="(instance) in instances"
        :key="instance.name"
        name="slide-x-transition"
        appear
        duration="100"
      >
        <v-hover>
          <template #default="{ hover }">
            <v-card
              v-if="!leaving"
              rounded="md"
              class="mb-2 card-outter"
              :elevation="hover ? '10' : '0'"
              color="#1a1a1a"
              @click="setInstance(instance)"
            >
              <v-card-title class="mb-2">
                <p class="text-center w-full">{{ instance.name }}</p>
              </v-card-title>
              <v-card-subtitle class="text-center">
                <!-- eslint-disable-next-line vue/no-v-html -->
                <div v-html="$t('pages.instances.mcVersion', {
                  version: instance.dependencies.minecraft
                })"
                />
                <!-- eslint-disable-next-line vue/no-v-html -->
                <div v-html="$t('pages.instances.mcVersion', {
                  version: instance.dependencies['fabric-loader']
                })"
                />
                {{ instance.summary }}
              </v-card-subtitle>
              <v-card-text v-if="downloadState && selectedInstance === instance">
                {{ $t('pages.instances.status', {
                  download: downloadState.name,
                  type: downloadState.type
                }) }}
                <v-progress-linear
                  :value="downloadState.current / downloadState.total * 100"
                />
              </v-card-text>
            </v-card>
          </template>
        </v-hover>
      </transition>
    </div>
    <div v-else>
      <section class="flex flex-col align-center justify-items-start pt-6">
        <transition
          v-for="(instance) in instances"
          :key="instance.name"
          name="slide-y-transition"
          duration="100"
          appear
        >
          <article v-if="!leaving" class="rounded-md pa-4 instance-card mb-4 w-3/4">
            <div class="flex flex-row justify-items-center align-center">
              <section class="card-info">
                <h3 class="text-h6">{{ instance.name }}</h3>
                <h4 class="text-subtitle-1 flex gap-2">
                  <!-- eslint-disable-next-line vue/no-v-html -->
                  <span v-html="$t('pages.instances.mcVersion', {
                    version: instance.dependencies.minecraft
                  })"
                  /> |
                  <!-- eslint-disable-next-line vue/no-v-html -->
                  <span v-html="$t('pages.instances.mcVersion', {
                    version: instance.dependencies['fabric-loader']
                  })"
                  />
                </h4>
              </section>
              <button class="card-action rounded-md pa-1 ml-auto" @click="$router.push({ path: localePath(`/instances/${instance ? instance.name : ''}`) });">
                <v-icon>
                  mdi-information-outline
                </v-icon>
              </button>
              <button class="card-action rounded-md pa-1 ml-2" @click="removeInstance(instance)">
                <v-icon>
                  mdi-delete
                </v-icon>
              </button>
              <button class="card-action rounded-md pa-1 ml-2" @click="launch(instance)">
                <v-icon>
                  mdi-play-circle-outline
                </v-icon>
              </button>
            </div>
          </article>
        </transition>
      </section>
    </div>
    <v-expand-x-transition duration="100">
      <v-navigation-drawer
        v-if="selectedInstance && !leaving"
        v-model="isVisible"
        color="primary"
        right
        permanent
        fixed
      >
        <div class="flex flex-col w-full h-full">
          <div class="mt-10">
            <div class="flex w-full mt-4 mb-3">
              <v-btn
                class="ml-auto mr-2"
                color="secondary"
                fab
                x-small
                @click="isVisible = false"
              >
                <v-icon>mdi-close</v-icon>
              </v-btn>
            </div>
            <v-divider />
            <h1 class="w-full text-center text-lg mt-2 mb-2"> {{ selectedInstance.name }} </h1>
            <h2 class="w-full text-center italic text-md mt-2 mb-2"> {{ selectedInstance.summary }} </h2>
            <v-divider />
          </div>
          <div class="mt-auto mb-6 mr-2 ml-2">
            <v-btn class="mt-4" block @click="launch(selectedInstance)">{{ $t('pages.instances.launch') }}</v-btn>
            <v-btn
              class="mt-4"
              color="secondary"
              block
              @click="$router.push({ path: localePath(`/instances/${selectedInstance ? selectedInstance.name : ''}`) })"
            >
              {{ $t('pages.instances.moreInfo') }}
            </v-btn>
            <v-btn
              class="mt-4"
              color="accent"
              block
              @click="removeInstance(selectedInstance)"
            >
              {{ $t('pages.instances.settings') }}
            </v-btn>
          </div>
        </div>
      </v-navigation-drawer>
    </v-expand-x-transition>
  </div>
</template>

<script lang="ts">
import Modpack from '@/../types/Modpack'
import DownloadProgress from '@/../types/DownloadProgress'
import { getModule } from 'vuex-module-decorators'
import Vue from 'vue'
import { typedIpcRenderer } from '../../../types/Ipc'
import InstancesModule from '~/store/instances'
import UiModule from '~/store/ui'
import UserModule from '~/store/users'

export default Vue.extend({
  beforeRouteLeave (_, _2, next) {
    this.leaving = true
    setTimeout(() => {
      next()
    }, 100)
  },
  data () {
    return {
      selectedInstance: null as Modpack | null,
      downloadState: null as DownloadProgress | null,
      leaving: false,
      filter: '',
      instanceStore: getModule(InstancesModule, this.$store),
      uiStore: getModule(UiModule, this.$store)
    }
  },
  computed: {
    isVisible: {
      get (): boolean {
        return this.selectedInstance !== null
      },
      set (val) {
        if (val === false) this.selectedInstance = null
      }
    },
    useList (): boolean {
      return this.uiStore.listMode
    },
    instances (): Modpack[] {
      return this.filter
        ? this.instanceStore.instances.filter(instance => instance.name.includes(this.filter))
        : this.instanceStore.instances
    }
  },
  mounted () {
    typedIpcRenderer.invoke('UpdatePresence', {
      details: 'Looking at their instances 👀',
      startTimestamp: new Date()
    })
  },
  methods: {
    setInstance (instance: Modpack) {
      this.selectedInstance = instance
    },
    refresh () {
      return this.instanceStore.REFRESH_INSTANCES()
    },
    async removeInstance (instance: Modpack | null) {
      if (!instance) return
      await this.instanceStore.DELETE_INSTANCE(instance)
    },
    async launch (instance: Modpack | null) {
      if (this.instance === undefined) return
      await typedIpcRenderer.invoke('LaunchMinecraft', instance!!, getModule(UserModule, this.$store).selected)
      typedIpcRenderer.on('DownloadStatus', (_e, status) => { this.downloadState = status })
      typedIpcRenderer.on('DownloadProgress', (_e, _progress) => {})
    },
    addInstance () {
      this.uiStore.TOGGLE_ADD_INSTANCE_MODAL()
    }
  }
})
</script>

<style lang="stylus">
.card-outter {
  position: relative
  padding-bottom: 50px
}

.v-text-field.v-text-field--solo:not(.v-text-field--solo-flat) > .v-input__control > .v-input__slot {
    box-shadow: 0px 0px 0px 0px !important
}

.instance-card {

  background-color #2a2a2a
  min-width 440px

  .card-action {
    background-color #444444

    &:hover {
      background-color #555555
    }

    &:active {
      background-color #666666
    }
  }

}

</style>
