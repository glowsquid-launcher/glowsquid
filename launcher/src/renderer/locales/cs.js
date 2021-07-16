/* eslint-disable max-len */
module.exports = {
  header: {
    home: 'domů',
    instances: 'instance',
    about: 'info',
    accounts: {
      addAcc: 'Přidat účet',
      subtitle: 'Přidejte svůj vedlejší nebo hlavní účet!',
      placeholder: 'Účty'
    }
  },
  authModal: {
    title: 'Přidat uživatele',
    email: 'email',
    password: 'heslo',
    submit: 'předložit',
    close: 'zavřít'
  },
  addInstanceModal: {
    instanceName: 'Název Instance',
    selectVersion: 'Vybrat verzi',
    selectFabricVersion: 'Vybrat Fabric verzi',
    showUnstable: 'Zobrazit nestabilní verze',
    title: 'Vytvořit Instanci | Vybrat šablonu | Extra nastavení',
    ramSettings: {
      title: 'Nastavení RAM',
      minRam: {
        title: 'Minimální RAM',
        hint: 'Minimální RAM pro Minecraft modpacky.'
      },
      maxRam: {
        title: 'Maximální RAM',
        hint: 'Maximální RAM pro Minecraft modpacky'
      }
    },
    randomSettings: {
      title: 'Náhodné nastavení',
      assetRoot: {
        title: 'Kořeny prostředí',
        hint: 'Zde je uskladňováno prostředí Minecraftu. Neměli by jste potřebovat změnit toto nastavení. Nechte jej prázdné pro výchozí volby.'
      }
    }
  },
  settings: {
    title: 'Nastavení',
    sections: {
      general: {
        title: 'Obecné',
        listView: {
          name: 'Zobrazení seznamu',
          subtitle: 'Zobrazuje instance v seznamu místo mřížky.'
        }
      }
    }
  },
  pages: {
    home: {
      text: 'Vítejte v Modrinth Minecraft launcheru Glowsquid:  Fabric ma prvním místě'
    },
    about: {
      title: 'Tohle je jen testující stránka, nic moc zde není.',
      toast: 'toast'
    },
    instances: {
      search: 'Probíhá stahování {download} | Typ: {type}',
      launch: 'hrát',
      moreInfo: 'Více informací',
      settings: 'Nastavní (Aktuálně maže instance)',
      mcVersion: 'Verze Minecraftu: <span class="font-bold">{version}</span>',
      fabricVersion: 'Fabric verze: <span class="font-bold">{version}</span>'
    },
    instance: {
      status: 'Probíhá stahování {download} | Typ: {type}',
      addMods: 'Přidat mody',
      settings: 'Nastavení',
      tabs: {
        description: 'Popis',
        mods: 'Mody'
      }
    },
    mods: {
      search: 'Hledat mody',
      about: 'O modu',
      install: 'Instalovat',
      hint: 'Zde můžete napsat jméno, autora a kategorii modu'
    },
    mod: {
      install: 'Instalovat'
    }
  }
}
