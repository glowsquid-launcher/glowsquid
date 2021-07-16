<template>
  <v-dialog
    v-model="visible"
    transition="dialog-bottom-transition"
    max-width="600"
    @submit="addInstance()"
  >
    <v-card>
      <v-toolbar
        color="secondary"
      >
        {{ $tc('addInstanceModal.title', idx) }}
      </v-toolbar>
      <v-form v-model="valid" @submit="addInstance()">
        <v-container>
          <v-item-group
            v-model="idx"
            class="w-full flex justify-center"
            mandatory
            tag="v-flex"
          >
            <v-item
              v-for="n in pages"
              :key="n"
              v-slot="{ active, toggle }"
            >
              <div>
                <v-btn
                  :input-value="active"
                  icon
                  @click="toggle"
                >
                  <v-icon>mdi-record</v-icon>
                </v-btn>
              </div>
            </v-item>
          </v-item-group>

          <v-window
            v-model="idx"
            class="elevation-1"
          >
            <v-window-item class="mt-3">
              <v-row>
                <v-text-field
                  :label="$t('addInstanceModal.instanceName')"
                  :color="$vuetify.theme.dark ? 'grey-lighten-4' : 'grey-darken-3'"
                  :rules="nameRules"
                  :value="name"
                  class="mr-6 ml-6"
                  @input="(e) => name = e"
                />
              </v-row>
              <v-row>
                <v-col>
                  <v-col>
                    <v-select
                      :items="versionsToShow"
                      :label="$t('addInstanceModal.selectVersion')"
                      :color="$vuetify.theme.dark ? 'grey-lighten-4' : 'grey-darken-3'"
                      :value="version"
                      :rules="versionRules"
                      @input="(e) => version = e"
                    >
                      <template #item="{ item }">
                        <p :class="[$vuetify.theme.dark ? 'white--text' : 'black--text']">{{ item.version }}</p>
                      </template>
                      <template #selection="{ item }">
                        <p class="mt-2 ml-1"> {{ item.version }}</p>
                      </template>
                      <template #prepend-item>
                        <v-checkbox v-model="showUnstable" class="ml-2" :label="$t('addInstanceModal.showUnstable')" />
                      </template>
                    </v-select>
                  </v-col>
                  <v-col>
                    <v-select
                      :items="loaderVersions"
                      :label="$t('addInstanceModal.selectFabricVersion')"
                      :color="$vuetify.theme.dark ? 'grey-lighten-4' : 'grey-darken-3'"
                      :value="loaderVersion"
                      :rules="loaderVersionRules"
                      @input="(e) => loaderVersion = e"
                    >
                      <template #item="{ item }">
                        <p :class="[$vuetify.theme.dark ? 'white--text' : 'black--text']">{{ item.version }}</p>
                      </template>
                      <template #selection="{ item }">
                        <p class="mt-2 ml-1"> {{ item.version }}</p>
                      </template>
                    </v-select>
                  </v-col>
                </v-col>
              </v-row>
            </v-window-item>
            <v-window-item>
              Soon™
            </v-window-item>
            <v-window-item>
              <h1 class="text-md">{{ $t('addInstanceModal.ramSettings.title') }}</h1>
              <div class="grid grid-cols-2 gap-2">
                <v-text-field
                  v-model="ram.min"
                  :color="$vuetify.theme.dark ? 'grey-lighten-4' : 'grey-darken-3'"
                  :label="$t('addInstanceModal.ramSettings.minRam.title')"
                  :hint="$t('addInstanceModal.ramSettings.minRam.title')"
                />
                <v-text-field
                  v-model="ram.max"
                  :color="$vuetify.theme.dark ? 'grey-lighten-4' : 'grey-darken-3'"
                  :label="$t('addInstanceModal.ramSettings.maxRam.title')"
                  :hint="$t('addInstanceModal.ramSettings.maxRam.hint')"
                />
              </div>
              <v-divider class="mb-2 mt-2" />
              <h1 class="text-md">{{ $t('addInstanceModal.randomSettings.title') }}</h1>
              <v-text-field
                v-model="assetRoot"
                :color="$vuetify.theme.dark ? 'grey-lighten-4' : 'grey-darken-3'"
                :label="$t('addInstanceModal.randomSettings.assetRoot.title')"
                :hint="$t('addInstanceModal.randomSettings.assetRoot.hint')"
                class="mb-2"
              />
            </v-window-item>
          </v-window>
        </v-container>
        <v-container>
          <v-card-actions>
            <p v-if="error" class="ml-3 error--text">{{ error }}</p>
            <div class="ml-auto">
              <v-btn type="submit" text :disabled="!valid">Submit</v-btn>
              <v-btn
                text
                @click="visible = false"
              >
                Close
              </v-btn>
            </div>
          </v-card-actions>
        </v-container>
      </v-form>
    </v-card>
  </v-dialog>
</template>

<script lang="ts">
import FabricVersion from '@/../types/FabricVersion'
import FabricLoaderVersion from '@/../types/FabricLoaderVersion'
import { getModule } from 'vuex-module-decorators'
import Vue from 'vue'
import InstancesModule from '~/store/instances'
import UiModule from '~/store/ui'

export default Vue.extend({
  data () {
    return {
      instanceStore: getModule(InstancesModule, this.$store),
      uiStore: getModule(UiModule, this.$store),
      version: {} as FabricVersion,
      versions: [] as FabricVersion[],
      loaderVersions: [] as FabricLoaderVersion[],
      loaderVersion: {} as FabricLoaderVersion,
      showUnstable: false,
      name: '',
      valid: false,
      error: '',
      pages: 3,
      idx: 0,
      versionRules: [
        v => v !== {} || 'please select a version'
      ],
      loaderVersionRules: [
        v => v !== {} || 'please select a loader version'
      ],
      ram: {
        min: '4G',
        max: '6G'
      },
      assetRoot: undefined as string | undefined
    }
  },
  async fetch ({ $axios }) {
    this.versions = await $axios.$get('https://meta.fabricmc.net/v2/versions/game')
    this.version = (await $axios.$get('https://meta.fabricmc.net/v2/versions/game')).filter(v => v.stable)[0]
    this.loaderVersions = await $axios.$get('https://meta.fabricmc.net/v2/versions/loader')
    this.loaderVersion = (await $axios.$get('https://meta.fabricmc.net/v2/versions/loader'))[0]
    console.log(this.$data)
  },
  computed: {
    versionsToShow () {
      if (this.showUnstable) {
        return this.versions
      } else {
        return this.versions.filter(ver => ver.stable)
      }
    },
    visible: {
      get () {
        return this.uiStore.addInstanceVisible
      },
      set (val) {
        if (val === false) this.uiStore.TOGGLE_ADD_INSTANCE_MODAL()
      }
    },
    instances () {
      return this.instanceStore.instances
    },
    nameRules () {
      return [
        v => !!v || 'Name is required',
        // @ts-ignore
        v => !this.instances.find(val => val.name === v) || 'Instance already exists'
      ]
    }
  },
  watch: {
    async visible () {
      this.versions = await this.$axios.$get('https://meta.fabricmc.net/v2/versions/game')
      this.version = (await this.$axios.$get('https://meta.fabricmc.net/v2/versions/game')).filter(v => v.stable)[0]
      this.loaderVersions = await this.$axios.$get('https://meta.fabricmc.net/v2/versions/loader')
      this.loaderVersion = await this.$axios.$get('https://meta.fabricmc.net/v2/versions/loader')[0]
    }
  },
  methods: {
    addInstance () {
      this.instanceStore.ADD_INSTANCE({
        name: this.name,
        fabricLoader: this.version,
        fabricLoaderVersion: this.loaderVersion,
        ram: this.ram,
        assetRoot: this.assetRoot ? this.assetRoot : undefined,
        store: this.$store
      })
        .catch(err => { this.error = err })
        .then(() => this.$toast.success(`successfully added ${this.name} to your instances`))
        .then(() => this.visible === false)
    }
  }
})
</script>
