<template>
  <div>
    <transition name="slide-y-transition" appear duration="100">
      <v-hover v-if="!leaving">
        <template #default="{ hover }">
          <v-card :elevation="hover ? 12 : 6" class="pa-4 transition" color="secondary">
            <h1 class="text-center">{{ $t('pages.home.text') }}</h1>
          </v-card>
        </template>
      </v-hover>
    </transition>
    <v-divider />
    <transition name="slide-y-transition" appear duration="100">
      <v-card v-if="!leaving">
        <v-card-title>Latest News</v-card-title>
        <v-card
          v-for="newsInfo in news"
          :key="newsInfo.title"
          class="news-card"
          color="secondary"
          elevation="6"
          tile
        >
          <v-card-title class="news-title">
            {{ newsInfo.title }}
          </v-card-title>
          <v-card-subtitle>
            <v-img v-if="newsInfo.image != null" class="rounded-3xl" width="50%" height="auto" :src="newsInfo.image" />
            <br>
            {{ newsInfo.date.toLocaleString() }}
          </v-card-subtitle>
          <v-card-text>{{ newsInfo.contents }}</v-card-text>
        </v-card>
      </v-card>
    </transition>
  </div>
</template>

<script lang="ts">
import { ipcRenderer } from 'electron'
import News from '@/../types/News'
import Vue from 'vue'
import { typedIpcRenderer } from '../../types/Ipc'
import NewsType from '~/../types/NewsType'

export default Vue.extend({
  beforeRouteLeave (_, _2, next) {
    this.leaving = true
    setTimeout(() => {
      next()
    }, 100)
  },
  data () {
    const newNews: News[] = [
      {
        title: 'OH MY!',
        type: NewsType.GLOWSQUID,
        contents: 'Glowsquid will soon be getting news article support.',
        date: new Date(),
        // eslint-disable-next-line max-len
        image: 'https://cdn.mos.cms.futurecdn.net/L5o2VmuWEABHm5HsCzGFTk.jpg'
      },
      {
        title: 'Fabric API 0.30.0',
        type: NewsType.GLOWSQUID,
        contents: 'OwO, 30? Fabric API 1.00.0 when :tiny_potato:',
        date: new Date(),
        // eslint-disable-next-line max-len
        image: 'https://i.imgur.com/q6D5d1u.png'
      }
    ]
    return {
      news: newNews as News[],
      leaving: false
    }
  },
  mounted () {
    typedIpcRenderer.invoke('UpdatePresence', {
      details: 'Looking around 👀',
      state: 'Not signed in yet',
      startTimestamp: new Date(),
      largeImageKey: 'glowsquid',
      largeImageText: 'Coming not soon™'
    })
  }
})
</script>
