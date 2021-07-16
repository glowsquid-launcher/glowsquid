<template>
  <div class="mb-2 sticky z-10">
    <v-app-bar color="primary" height="50">
      <v-btn fab color="secondary" class="mr-2" x-small @click="$router.back()">
        <v-icon>mdi-arrow-left</v-icon>
      </v-btn>
      <v-toolbar-title> Glowsquid </v-toolbar-title>

      <v-spacer />

      <v-btn-toggle v-model="route" dense>
        <v-btn class="flex" color="secondary">
          <v-icon class="mr-1">mdi-package</v-icon>
          <p class="text-center h-full mb-0">{{ $t('header.instances') }}</p>
        </v-btn>

        <v-btn class="flex" color="secondary">
          <v-icon class="mr-1">mdi-home</v-icon>
          <p class="text-center h-full mb-0">{{ $t('header.home') }}</p>
        </v-btn>

        <v-btn class="flex" color="secondary">
          <v-icon class="mr-1">mdi-information</v-icon>
          <p class="text-center h-full mb-0">{{ $t('header.about') }}</p>
        </v-btn>
      </v-btn-toggle>

      <v-spacer />
      <v-select
        v-model="account"
        :items="accounts"
        :label="$t('header.accounts.placeholder')"
        color="primary"
        class="mt-4 mr-3"
        style="max-width: 15%;"
        height="45"
      >
        <template #item="{ item }">
          <v-img
            :max-height="32"
            :max-width="32"
            :src="`https://minotar.net/avatar/${item.uuid}`"
            class="mr-3 rounded-sm"
          />
          <p :class="[$vuetify.theme.dark ? 'white--text' : 'black--text', 'mt-4']">{{ item.name }}</p>
          <v-btn
            class="ml-auto mr-0"
            @click="usersStore.REMOVE_USER(usersStore.users.indexOf(item))"
          >
            <v-icon>mdi-close</v-icon>
          </v-btn>
        </template>
        <template #selection="{ item }">
          <v-img
            :max-height="32"
            :max-width="32"
            :src="`https://minotar.net/avatar/${item.uuid}`"
            class="mr-3 rounded-sm"
          />
          <p :class="[$vuetify.theme.dark ? 'white--text' : 'black--text', 'mt-4']">{{ item.name }}</p>
        </template>
        <template #append-item>
          <v-divider class="mb-2" />
          <v-list-item>
            <v-list-item-avatar>
              <v-btn @click="uiStore.TOGGLE_AUTH_MODAL()">
                <v-icon>mdi-plus</v-icon>
              </v-btn>
            </v-list-item-avatar>

            <v-list-item-content>
              <v-list-item-title>
                {{ $t('header.accounts.addAcc') }}
              </v-list-item-title>
              <v-list-item-subtitle>
                {{ $t('header.accounts.subtitle') }}
              </v-list-item-subtitle>
            </v-list-item-content>
          </v-list-item>
        </template>
      </v-select>
      <v-btn fab x-small color="secondary" @click="toggleSettings">
        <v-icon>mdi-cog</v-icon>
      </v-btn>
    </v-app-bar>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { getModule } from 'vuex-module-decorators'
import { IUser } from 'minecraft-launcher-core'
import UiModule from '~/store/ui'
import UserModule from '~/store/users'

export default Vue.extend({
  data () {
    return {
      uiStore: getModule(UiModule, this.$store),
      usersStore: getModule(UserModule, this.$store),
      routeIndex: 1
    }
  },
  computed: {
    accounts (): IUser[] {
      return this.usersStore.users
    },
    account: {
      get () { return this.usersStore.selected },
      set (val) { this.usersStore.SET_USER(this.usersStore.users.indexOf(val as any)) }
    },
    route: {
      get () { return this.$data.routeIndex },
      set (val: 0 | 1 | 2) {
        switch (val) {
        case 1:
          this.$router.push({ path: this.localePath('/') })
          break
        case 0:
          this.$router.push({
            path: this.localePath('/instances')
          })
          break
        case 2:
          this.$router.push({ path: this.localePath('/about') })
          break
        default:
          break
        }
        this.$data.routeIndex = val
      }
    }
  },
  methods: {
    toggleSettings () {
      this.uiStore.TOGGLE_SETTINGS()
    }
  }
})
</script>
