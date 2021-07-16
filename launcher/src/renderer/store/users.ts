import { IUser, Authenticator } from 'minecraft-launcher-core'
import { Action, Module, Mutation, VuexModule } from 'vuex-module-decorators'
import { store } from '@/plugins/store'
import { typedIpcRenderer } from '../../types/Ipc'

@Module({
  name: 'users',
  stateFactory: true,
  namespaced: true
})
export default class UserModule extends VuexModule {
  users: IUser[] = store.get('users', []);
  selected: IUser = store.get('selected', {} as IUser);

  @Mutation
  PUSH_USER (user: IUser) {
    this.users.push(user)
    store.set('users', this.users)
  }

  @Mutation
  SET_USER (idx: number) {
    this.selected = this.users[idx]
    typedIpcRenderer.invoke('UpdatePresence', {
      state: `Signed in as ${this.selected.name}`
    }).then(r => r)
    store.set('selectedUser', this.selected)
  }

  @Mutation
  REMOVE_USER (idx: number) {
    this.users.splice(idx, 1)
    store.set('users', this.users)
  }

  @Action
  async ADD_USER ({ username, password }: { username:string, password:string}) {
    console.log(username, password)
    try {
      const user = await Authenticator.getAuth(username, password)

      if (this.users.find(val => val.name === user.name)) {
        throw new Error('User already logged in')
      } else this.context.commit('PUSH_USER', user)
    } catch (e) {
      if (e.message === 'Validation error: OK') throw new Error('You do not have a minecraft account')
      throw e
    }
  }
}
