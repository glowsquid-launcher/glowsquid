module.exports = {
  header: {
    home: 'главная',
    instances: 'сборки',
    about: 'о программе',
    accounts: {
      addAcc: 'Добавить аккаунт',
      subtitle: 'Вперёд, добавь ещё один аккаунт!',
      placeholder: 'Аккаунты'
    }
  },
  authModal: {
    title: 'Добавить пользователя',
    email: 'email',
    password: 'пароль',
    submit: 'добавить',
    close: 'закрыть'
  },
  addInstanceModal: {
    instanceName: 'Название сборки',
    selectVersion: 'Версия',
    selectFabricVersion: 'Версия Fabric',
    showUnstable: 'Показать нестабильные версии',
    title: 'Создать сборку | Выбрать шаблон | Дополнительные настройки',
    ramSettings: {
      title: 'Настройки памяти',
      minRam: {
        title: 'Мин ОЗУ',
        hint: 'Минималньный объём выделенной памяти'
      },
      maxRam: {
        title: 'Макс ОЗУ',
        hint: 'Максимальный объём выделенной памяти'
      }
    },
    randomSettings: {
      title: 'Дополнительные настройки',
      assetRoot: {
        title: 'Корневая папка',
        hint: 'Где хранятся файлы Minecraft. Не рекомендуется менять. Оставьте пустым для стандартного пути'
      }
    }
  },
  settings: {
    title: 'Настройки',
    sections: {
      general: {
        title: 'Общие',
        listView: {
          name: 'Показывать списком',
          subtitle: 'Показывает сборки списком.'
        }
      }
    }
  },
  pages: {
    home: {
      text: 'Встрейчайте Glowsquid, первый Fabric Minecraft лаунчер с поддержкой Modrinth'
    },
    about: {
      title: 'Это тестовая страница, тут нечего смотреть.',
      toast: 'toast'
    },
    instances: {
      search: 'Поиск',
      status: 'Загрузка {download} | Тип: {type}',
      launch: 'запуск',
      moreInfo: 'Подробнее',
      settings: 'Настройки(Удаляет сборку)',
      mcVersion: 'Версия Minecraft: <span class="font-bold">{version}</span>',
      fabricVersion: 'Версия Fabric loader: <span class="font-bold">{version}</span>'
    },
    instance: {
      status: 'Загрузка {download} | Тип: {type}',
      addMods: 'Добавить моды',
      settings: 'Настройки',
      tabs: {
        description: 'Описание',
        mods: 'Моды'
      }
    },
    mods: {
      search: 'Поиск модов',
      about: 'О моде',
      install: 'Установить',
      hint: 'Сюда можно ввести название мода, автора и категорию'
    },
    mod: {
      install: 'Установить'
    }
  }
}
