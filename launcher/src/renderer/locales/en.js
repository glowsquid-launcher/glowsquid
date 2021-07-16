module.exports = {
  header: {
    home: 'home',
    instances: 'instances',
    about: 'about',
    accounts: {
      addAcc: 'Add account',
      subtitle: 'Go ahead, add another account or your first one!',
      placeholder: 'Accounts'
    }
  },
  authModal: {
    title: 'Add user',
    email: 'email',
    password: 'password',
    submit: 'submit',
    close: 'close'
  },
  addInstanceModal: {
    instanceName: 'Instance Name',
    selectVersion: 'Select Version',
    selectFabricVersion: 'Select Fabric Version',
    showUnstable: 'Show unstable versions',
    title: 'Setup Instance | Select template | Extra Settings',
    ramSettings: {
      title: 'Ram settings',
      minRam: {
        title: 'Min ram',
        hint: 'Minimum ram allocated to Minecraft'
      },
      maxRam: {
        title: 'Max ram',
        hint: 'Maximum ram allocated to Minecraft'
      }
    },
    randomSettings: {
      title: 'Random settings',
      assetRoot: {
        title: 'asset Root',
        hint: 'Where minecraft assets are stored. You shouldn\'t need to change this. Leave blank for default'
      }
    }
  },
  settings: {
    title: 'Settings',
    sections: {
      general: {
        title: 'General',
        listView: {
          name: 'List view',
          subtitle: 'Shows mod instances as a List instead of a Grid.'
        }
      }
    }
  },
  pages: {
    home: {
      text: 'Welcome to Glowsquid, the fabric-first modrinth-powered Minecraft launcher'
    },
    about: {
      title: 'This is mostly a test page, nothing to see here.',
      toast: 'toast'
    },
    instances: {
      search: 'Search',
      status: 'Downloading {download} | Type: {type}',
      launch: 'launch',
      moreInfo: 'More info',
      settings: 'Settings(Deletes instances rn)',
      mcVersion: 'Minecraft version: <span class="font-bold">{version}</span>',
      fabricVersion: 'Fabric loader version: <span class="font-bold">{version}</span>'
    },
    instance: {
      status: 'Downloading {download} | Type: {type}',
      addMods: 'Add mods',
      settings: 'Settings',
      tabs: {
        description: 'Description',
        mods: 'Mods'
      }
    },
    mods: {
      search: 'Search mods',
      about: 'About',
      install: 'Install',
      hint: 'You can type the mod name, author and category here'
    },
    mod: {
      install: 'Install',
      tabs: {
        description: 'Description',
        versions: 'Versions'
      }
    }
  }
}
