<template>
  <div v-if="instance && ready" class="flex flex-col">
    <article class="ml-4 flex max-h-40">
      <transition
        name="slide-y-transition"
        appear
        duration="100"
      >
        <div v-if="!leaving" class="w-10/12">
          <h1 class="text-xl text-center mb-2">{{ instance.name }}</h1>
          <h2 class="text-md text-center italic">{{ instance.summary }}</h2>
          <div v-if="downloadState" class="flex flex-row justify-between align-middle">
            <p>
              {{
                $t('pages.instance.status', {
                  download: downloadState.name,
                  type: downloadState.type
                })
              }}
            </p>

            <v-progress-linear
              height="55px"
              class="mr-3 self-center"
              color="secondary"
              background-color="primary"
              :value="downloadState.current / downloadState.total * 100"
            />
          </div>
        </div>
      </transition>
      <transition
        name="slide-x-reverse-transition"
        appear
        duration="100"
      >
        <div v-if="!leaving" class="ml-auto flex flex-col w-2/12">
          <v-btn class="mb-2" color="secondary" @click="launch()">Launch</v-btn>
          <v-btn
            class="mb-2"
            color="accent"
            @click="$router.push({ path: localePath(`/instances/${$route.params.id}/mods`) })"
          >
            {{ $t('pages.instance.addMods') }}
          </v-btn>
          <v-btn color="secondary align-self-center">{{ $t('pages.instance.settings') }}</v-btn>
        </div>
      </transition>
    </article>
    <transition name="fade-transition" appear duration="100">
      <v-tabs v-if="!leaving" color="secondary" class="mt-auto flex flex-grow flex-col">
        <v-tab href="#desc">
          {{ $t('pages.instance.tabs.description') }}
        </v-tab>
        <v-tab href="#mods">
          {{ $t('pages.instance.tabs.mods') }}
        </v-tab>

        <v-tab-item id="desc" key="desc" class="flex-grow">
          <!-- eslint-disable-next-line vue/no-v-html we sanitised this using DOMPurify so we know its safe-->
          <div class="ml-3" v-html="desc" />
        </v-tab-item>
        <v-tab-item id="mods" key="mods">
          <transition
            v-for="(mod) in modList"
            :key="mod.id"
            name="slide-y-transition"
            duration="100"
            appear
          >
            <section class="mt-5 mb-5 hover:shadow-xl">
              <v-card color="primary">
                <v-card-title>
                  <p class="text-h5 text-center w-full">{{ mod.title }}</p>
                </v-card-title>
                <v-card-subtitle>
                  <p class="subtitle-2 text-center">
                    by {{ membersMap.get(mod.id).map(member => member.name).join(", ") }}
                  </p>
                </v-card-subtitle>
                <v-card-text>
                  <p class="body-1 italic text-center"> {{ mod.description }} </p>
                </v-card-text>
                <v-card-actions class="flex justify-center">
                  <section class="grid grid-cols-3 gap-24 w-3/4">
                    <v-btn
                      color="secondary"
                      @click="$router.push({
                        path: localePath(`/instances/${$route.params.id}/mods/${mod.id.replace('%7D', '')}`)
                      })"
                    >
                      About Mod
                    </v-btn>
                    <!-- error looks better than default red -->
                    <v-btn color="error">
                      delete
                    </v-btn>
                    <v-btn color="secondary">
                      reinstall
                    </v-btn>
                  </section>
                </v-card-actions>
              </v-card>
            </section>
          </transition>
        </v-tab-item>
      </v-tabs>
    </transition>
  </div>
</template>

<script lang="ts">
import marked from 'marked'
import DOMPurify from 'dompurify'
import { getModule } from 'vuex-module-decorators'
import Vue from 'vue'
import ModFile from '../../../../types/ModFile'
import Mod from '../../../../types/Mod'
import { TeamMember } from '../../../../types/TeamMember'
import { User } from '../../../../types/User'
import DownloadProgress from '~/../types/DownloadProgress'
import InstancesModule from '~/store/instances'
import { typedIpcRenderer } from '~/../types/Ipc'
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
      instance: getModule(InstancesModule, this.$store).instances.find(v => v.name === this.$route.params.id),
      leaving: false,
      downloadState: null as DownloadProgress | null,
      modList: [] as Mod[],
      // string is the mod id
      membersMap: new Map() as Map<string, User[]>,
      ready: false
    }
  },
  async fetch () {
    const instance = getModule(InstancesModule, this.$store).instances.find(v => v.name === this.$route.params.id)
    if (!instance) return

    this.modList = await Promise.all(instance.files.map(async modFile => {
      const mod: Mod = this.$axios.$get(`https://api.modrinth.com/api/v1/mod/${modFile.id}`)
      return mod
    }))

    await Promise.all(this.modList.map(async mod => {
      const teamMembers: TeamMember[] = await this.$axios.$get(
        `https://api.modrinth.com/api/v1/team/${mod.team}/members`
      )

      const users: User[] = await Promise.all(teamMembers.map(member =>
        this.$axios.$get(`https://api.modrinth.com/api/v1/user/${member.user_id}`)
      ))

      this.membersMap.set(mod.id, users)
    }))

    // to prevent bad errors
    this.ready = true
  },
  computed: {
    desc (): string {
      return marked(this.instance?.description ?? '', {
        sanitize: true,
        sanitizer: DOMPurify.sanitize
      })
    }
  },
  methods: {
    async launch () {
      if (this.instance === undefined) return

      await typedIpcRenderer.invoke('LaunchMinecraft', this.instance!!, getModule(UserModule, this.$store).selected)
      typedIpcRenderer.on('DownloadStatus', (_e, status) => {
        this.downloadState = status
        console.log(status)
      })
    },
    async deleteMod (mod: ModFile) {

    }
  }

})
</script>

<style lang="stylus">
.v-tabs .v-tabs-items {
  height: 100%
}
</style>
