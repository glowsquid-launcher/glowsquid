<template>
  <section>
    <v-app color="primary">
      <app-header />
      <auth-modal />
      <settings-dialog />
      <add-instance-modal />
      <nuxt class="ml-1 mr-1" />
    </v-app>
  </section>
</template>

<script lang="ts">
import appHeader from '@/components/header.vue'
import authModal from '@/components/authModal.vue'
import settingsDialog from '@/components/settingsDialog.vue'
import addInstanceModal from '@/components/addInstanceModal.vue'
export default {
  components: { appHeader, authModal, settingsDialog, addInstanceModal }
}
</script>

<style>
body {
  margin: 0 !important;
}
</style>
