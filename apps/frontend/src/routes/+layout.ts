import {
    detectLocale,
    htmlLangAttributeDetector,
    localStorageDetector,
    navigatorDetector
} from 'typesafe-i18n/detectors';
import { loadLocaleAsync, locales} from '@glowsquid/i18n';
import type {LayoutLoad} from './$types';

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
    return {locale};
};
