import {
    detectLocale,
    htmlLangAttributeDetector,
    localStorageDetector,
    navigatorDetector
} from 'typesafe-i18n/detectors';
import type { Locales } from '../i18n/i18n-types';
import { loadLocaleAsync } from '../i18n/i18n-util.async';
import { locales } from '../i18n/i18n-util.js';
import type { LayoutLoad } from './$types';

export const ssr = false;

export const load: LayoutLoad = async () => {
    const locale = detectLocale(
        'en',
        locales,
        localStorageDetector,
        navigatorDetector,
        htmlLangAttributeDetector
    );

    await loadLocaleAsync(locale);
    return { locale };
};
