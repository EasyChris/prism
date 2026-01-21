import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

// Import Chinese translations
import zhCommon from './locales/zh/common.json';
import zhDashboard from './locales/zh/dashboard.json';
import zhProfiles from './locales/zh/profiles.json';
import zhLogs from './locales/zh/logs.json';
import zhSettings from './locales/zh/settings.json';

// Import English translations
import enCommon from './locales/en/common.json';
import enDashboard from './locales/en/dashboard.json';
import enProfiles from './locales/en/profiles.json';
import enLogs from './locales/en/logs.json';
import enSettings from './locales/en/settings.json';

const resources = {
  zh: {
    common: zhCommon,
    dashboard: zhDashboard,
    profiles: zhProfiles,
    logs: zhLogs,
    settings: zhSettings,
  },
  en: {
    common: enCommon,
    dashboard: enDashboard,
    profiles: enProfiles,
    logs: enLogs,
    settings: enSettings,
  },
};

i18n
  .use(initReactI18next)
  .init({
    resources,
    lng: localStorage.getItem('language') || 'zh',
    fallbackLng: 'zh',
    defaultNS: 'common',
    interpolation: {
      escapeValue: false,
    },
  });

export default i18n;
