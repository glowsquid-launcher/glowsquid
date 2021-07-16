<template>
  <v-dialog
    v-model="visible"
    transition="dialog-bottom-transition"
    max-width="600"
  >
    <v-card>
      <v-toolbar
        color="secondary"
      >
        {{ $t('authModal.title') }}
      </v-toolbar>
      <v-form v-model="valid" @submit="addUser()">
        <v-container>
          <v-row>
            <v-col>
              <v-col>
                <v-text-field
                  v-model="email"
                  :rules="emailRules"
                  :color="$vuetify.theme.dark ? 'grey-lighten-4' : 'grey-darken-3'"
                  :label="$t('authModal.email')"
                  required
                />
              </v-col>

              <v-col>
                <v-text-field
                  v-model="password"
                  :rules="passRules"
                  :color="$vuetify.theme.dark ? 'grey-lighten-4' : 'grey-darken-3'"
                  :label="$t('authModal.password')"
                  type="password"
                  required
                />
              </v-col>
            </v-col>
          </v-row>
        </v-container>
        <v-card-actions>
          <p v-if="error" class="ml-3 error--text">{{ error }}</p>
          <div class="ml-auto">
            <v-btn type="submit" text :disabled="!valid">{{ $t('authModal.submit') }}</v-btn>
            <v-btn
              text
              @click="uiStore.TOGGLE_AUTH_MODAL()"
            >
              {{ $t('authModal.close') }}
            </v-btn>
          </div>
        </v-card-actions>
      </v-form>
    </v-card>
  </v-dialog>
</template>

<script lang="ts">
import Vue from 'vue'
import { getModule } from 'vuex-module-decorators'
import UiModule from '~/store/ui'
import UserModule from '~/store/users'

export default Vue.extend({
  data () {
    return {
      uiStore: getModule(UiModule, this.$store),
      usersStore: getModule(UserModule, this.$store),
      emailRules: [
        v => !!v || 'E-mail is required',
        v => /.+@.+/.test(v) || 'E-mail must be valid'
      ],
      email: '',
      passRules: [
        v => !!v || 'Password is required',
        v => v.length >= 8 || 'Password must be more than 8 characters'
      ],
      password: '',
      valid: false,
      error: ''
    }
  },
  computed: {
    visible: {
      get (): boolean {
        return this.uiStore.authModalVisible
      },
      set (value) {
        if (value !== this.uiStore.authModalVisible) this.uiStore.TOGGLE_AUTH_MODAL()
      }
    }
  },
  methods: {
    log (...vals: any[]) {
      console.log(...vals)
    },
    addUser () {
      this.usersStore.ADD_USER({ username: this.email, password: this.password })
        .then(() => {
          this.uiStore.TOGGLE_AUTH_MODAL()
          this.error = ''
        })
        .catch(err => {
          console.log(err)
          this.error = err.message
        })
    }
  }
})
</script>
