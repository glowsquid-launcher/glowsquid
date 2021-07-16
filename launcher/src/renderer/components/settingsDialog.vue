<template>
  <v-dialog
    v-model="dialog"
    fullscreen
    hide-overlay
    transition="dialog-bottom-transition"
  >
    <v-card>
      <v-toolbar
        dark
        color="primary"
      >
        <v-btn
          icon
          dark
          @click="dialog = false"
        >
          <v-icon>mdi-close</v-icon>
        </v-btn>
        <v-toolbar-title>{{ $t('settings.title') }}</v-toolbar-title>
      </v-toolbar>
      <v-list
        three-line
        subheader
      >
        <v-subheader>{{ $t('settings.sections.general.title') }}</v-subheader>
        <v-list-item>
          <v-list-item-action>
            <v-checkbox v-model="useGrid" />
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>{{ $t('settings.sections.general.listView.name') }}</v-list-item-title>
            <v-list-item-subtitle>{{ $t('settings.sections.general.listView.subtitle') }}</v-list-item-subtitle>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-card>
  </v-dialog>
</template>

<script lang="ts">
import Vue from 'vue'
import { getModule } from 'vuex-module-decorators'
import UiModule from '~/store/ui'

export default Vue.extend({
  data () {
    return {
      uiStore: getModule(UiModule, this.$store)
    }
  },
  computed: {
    dialog: {
      get (): boolean {
        return this.uiStore.settingsVisible
      },
      set (val) {
        if (val !== this.uiStore.settingsVisible) this.uiStore.TOGGLE_SETTINGS()
      }
    },
    useGrid: {
      get (): boolean {
        return this.uiStore.listMode
      },
      set (val) {
        if (val !== this.uiStore.listMode) this.uiStore.TOGGLE_LIST_MODE()
      }
    }
  }
})
</script>
