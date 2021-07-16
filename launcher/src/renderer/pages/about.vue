<template>
  <div>
    <h1 class="text-h4 text-center"> {{ $t('pages.about.title') }} </h1>
    <v-slider v-model="dur" min="700" max="5000" label="duration" step="100" thumb-label ticks />
    <v-btn class="elevation-16" @click="toast">
      {{ $t('pages.about.toast') }}
    </v-btn>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { typedIpcRenderer } from '../../types/Ipc'
export default Vue.extend({
  data () {
    return {
      dur: 1000
    }
  },
  mounted () {
    typedIpcRenderer.invoke('UpdatePresence', {
      details: 'testing stuff',
      state: 'Not signed in yet',
      startTimestamp: new Date(),
      largeImageKey: 'glowsquid',
      largeImageText: 'Coming not soon™'
    })
  },
  methods: {
    toast () {
      this.$toast.show('yes', {
        icon: 'Information',
        duration: this.dur
      })
    }
  }
})
</script>
