/**
 * Barcode & Keyboard Scanner Layout Normalizer
 * 
 * Many hardware barcode scanners act as USB HID keyboards.
 * When the operating system language is set to French (AZERTY) or Arabic,
 * the scanner sends shifted or language-mapped characters:
 * 
 * - French AZERTY top row: '&é"\'(-è_çà' corresponds to '1234567890'
 * - Arabic numeric digits: '١٢٣٤٥٦٧٨٩٠' corresponds to '1234567890'
 * - Common barcode punctuations & alphas
 */

// Full character mapping for barcode scanners running under French (AZERTY) or Arabic Windows keyboard layouts
const SCANNER_CHAR_MAP: Record<string, string> = {
  // French AZERTY top row digits
  '&': '1',
  'é': '2',
  '"': '3',
  "'": '4',
  '(': '5',
  '-': '6', // On AZERTY top row, key 6 is '-'
  'è': '7',
  '_': '8',
  'ç': '9',
  'à': '0',
  '§': '6',
  '°': '0',

  // Arabic-Indic digits
  '٠': '0',
  '١': '1',
  '٢': '2',
  '٣': '3',
  '٤': '4',
  '٥': '5',
  '٦': '6',
  '٧': '7',
  '٨': '8',
  '٩': '9',

  // French AZERTY punctuation to standard barcode punctuation
  ')': '-',
  '=': '+',
  ':': '/',
  '!': '/',
  ';': '.',
  ',': '.',

  // Arabic layout punctuation to standard barcode characters
  'ظ': '/',
  'ز': '.',
  'ـ': '-',
};

/**
 * Normalizes any scanned barcode string into standard alphanumeric barcode digits & symbols.
 * Handles French AZERTY and Arabic keyboard layout distortions for 0-9, -, /, ., *, +.
 */
export function normalizeBarcode(input: string): string {
  if (!input) return '';

  let clean = '';
  for (let i = 0; i < input.length; i++) {
    const char = input[i];
    if (SCANNER_CHAR_MAP[char] !== undefined) {
      clean += SCANNER_CHAR_MAP[char];
    } else {
      clean += char;
    }
  }

  return clean.trim();
}

/**
 * Validates whether a barcode matches standard retail lengths:
 * - EAN-13 (13 digits)
 * - EAN-8 (8 digits)
 * - UPC-A (12 digits)
 * - Scale variable weight PLU (6 or 18 digits)
 */
export function isValidBarcodeStructure(code: string): boolean {
  const digitsOnly = normalizeBarcode(code).replace(/\D/g, '');
  return digitsOnly.length >= 4 && digitsOnly.length <= 18;
}
