import ar from './ar.json';
import fr from './fr.json';
import en from './en.json';

export type Language = 'ar' | 'fr' | 'en';

const translations: Record<Language, Record<string, string>> = {
  ar,
  fr,
  en,
};

let currentLang: Language = (localStorage.getItem('pos_lang') as Language) || 'ar';

export function getLanguage(): Language {
  return currentLang;
}

export function setLanguage(lang: Language) {
  currentLang = lang;
  localStorage.setItem('pos_lang', lang);
  document.documentElement.dir = lang === 'ar' ? 'rtl' : 'ltr';
  document.documentElement.lang = lang;
}

export function t(key: string): string {
  return translations[currentLang]?.[key] || translations['en']?.[key] || key;
}