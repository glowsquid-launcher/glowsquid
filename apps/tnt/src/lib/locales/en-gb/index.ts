import type { BaseTranslation } from '../i18n-types'

// translations for the main UI in english (traditional)
const enGb: BaseTranslation = {
  header: {
    title: 'Glowsquid',
    tabs: {
      home: 'Home',
      browse: 'Browse',
      instances: 'Instances',
    },
  },

  accounts: {
    placeholderText: 'Select an account',
    addAccount: 'Add new account',
    modal: {
      login: 'Login with microsoft',
      states: {
        Complete: 'Completed!',
        Failed: 'Failed',
        RequestRecieved: 'Recieved info from browser, adding...',
        WaitingForBrowser: 'Waiting for browser to send us the info!',
      },
    },
  },
}

export default enGb
