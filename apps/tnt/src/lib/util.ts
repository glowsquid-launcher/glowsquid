import { browser } from "$app/env";
import { detectLocale, localStorageDetector, navigatorDetector } from "typesafe-i18n/detectors";
import { setLocale } from "$locales/i18n-svelte";
import { baseLocale, locales} from "$locales/i18n-util";
import { loadLocale } from "$locales/i18n-util.sync";

export const refreshLocales = () => {
if (browser) {
    const currentLocale = detectLocale(
      baseLocale,
       locales,
      navigatorDetector,
      localStorageDetector
    );

    loadLocale(currentLocale);
    setLocale(currentLocale);
  }
}
