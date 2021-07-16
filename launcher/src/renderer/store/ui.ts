import { Module, Mutation, VuexModule } from 'vuex-module-decorators'
@Module({
  name: 'ui',
  stateFactory: true,
  namespaced: true
})
export default class UiModule extends VuexModule {
  authModalVisible = false
  settingsVisible = false
  addInstanceVisible = false
  listMode = false

  @Mutation
  TOGGLE_AUTH_MODAL () { this.authModalVisible = !this.authModalVisible }

  @Mutation
  TOGGLE_SETTINGS () { this.settingsVisible = !this.settingsVisible }

  @Mutation
  TOGGLE_ADD_INSTANCE_MODAL () { this.addInstanceVisible = !this.addInstanceVisible }

  @Mutation
  TOGGLE_LIST_MODE () { this.listMode = !this.listMode }
}
