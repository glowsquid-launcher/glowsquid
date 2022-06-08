import { browser } from '$app/env'
import {
  detectLocale,
  localStorageDetector,
  navigatorDetector,
} from 'typesafe-i18n/detectors'
import { setLocale } from '$locales/i18n-svelte'
import { baseLocale, locales } from '$locales/i18n-util'
import { loadLocale } from './locales/i18n-util.sync'

/**
 * refreshes the current locale and loads whatever new things you wanted it to load
 */
export const refreshLocales = () => {
  const currentLocale = detectLocale(
    baseLocale,
    locales,
    navigatorDetector,
    localStorageDetector
  )

  loadLocale(currentLocale)
  setLocale(currentLocale)
}

export interface PlayerDBMinecraftProfile {
  code: string
  message: string
  data: Data
  success: boolean
}

export interface Data {
  player: Player
}

export interface Player {
  meta: Meta
  username: string
  id: string
  raw_id: string
  avatar: string
}

export interface Meta {
  name_history: NameHistory[]
}

export interface NameHistory {
  name: string
  changedToAt?: number
}
