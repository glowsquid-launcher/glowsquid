module.exports = {
  header: {
    home: 'home',
    instances: 'instanties',
    about: 'over',
    accounts: {
      addAcc: 'Voeg account toe',
      subtitle: 'Ga je gang, voeg nog een account toe, of je eerste!',
      placeholder: 'Accounts'
    }
  },
  authModal: {
    title: 'Voeg gebruiker toe',
    email: 'e-mail',
    password: 'wachtwoord',
    submit: 'log in',
    close: 'sluit'
  },
  addInstanceModal: {
    instanceName: 'Instantie Naam',
    selectVersion: 'Selecteer Versie',
    selectFabricVersion: 'Selecteer Fabric Versie',
    showUnstable: 'Laat Instabiele Versies Zien',
    title: 'Maak Instantie | Selecteer Blauwtekening | Extra Instellingen',
    ramSettings: {
      title: 'Ram Instellingen',
      minRam: {
        title: 'Min ram',
        hint: 'Minimum ram toegewezen aan Minecraft'
      },
      maxRam: {
        title: 'Max ram',
        hint: 'Maximum ram toegewezen aan Minecraft'
      }
    },
    randomSettings: {
      title: 'Willekeurige instellingen',
      assetRoot: {
        title: 'asset Root',
        hint: 'Waar Minecraft gegevens worden opgeslagen. Je zou dit niet hoeven aan te passen. Laat standaard blank'
      }
    }
  },
  settings: {
    title: 'Instellingen',
    sections: {
      general: {
        title: 'Algemeen',
        listView: {
          name: 'Lijst overzicht',
          subtitle: 'Toon mod instanties als Lijst in plaats van een Rooster.'
        }
      }
    }
  },
  pages: {
    home: {
      text: 'Welkom bij Glowsquid, de fabric-eerst modrinth-powered Minecraft launcher'
    },
    about: {
      title: 'Dit is voornamelijk een test pagina, niks te zien hier.',
      toast: 'toost'
    },
    instances: {
      search: 'Zoek',
      status: 'Downloaden {download} | Type: {type}',
      launch: 'lanceer',
      moreInfo: 'Meer info',
      settings: 'Instellingen(Verwijdert instanties atm)',
      mcVersion: 'Minecraft versie: <span class="font-bold">{version}</span>',
      fabricVersion: 'Fabric lader versie: <span class="font-bold">{version}</span>'
    },
    instance: {
      status: 'Downloaden {download} | Type: {type}',
      addMods: 'Voeg mods toe',
      settings: 'Instellingen',
      tabs: {
        description: 'Beschrijving',
        mods: 'Mods'
      }
    },
    mods: {
      search: 'Zoek mods',
      about: 'Over',
      install: 'Installeer',
      hint: 'Je kunt hier de mod naam, auteur en categorie typen'
    },
    mod: {
      install: 'Installeer',
      tabs: {
        description: 'Beschrijving',
        versions: 'Versies'
      }
    }
  }
}
