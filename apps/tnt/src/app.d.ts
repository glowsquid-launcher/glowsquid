/* eslint-disable no-unused-vars */
import '@sveltejs/kit'
type Locales = import('$locales/i18n-types').Locales

declare namespace App {
    // interface Locals { }

    // interface Platform { }

    interface Session {
        locale?: Locales
    }

    // interface Stuff { }
}
