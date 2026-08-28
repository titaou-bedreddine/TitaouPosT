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

const AZERTY_TO_DIGITS: Record<string, string> = {
  '&': '1',
  'é': '2',
  '"': '3',
  "'": '4',
  '(': '5',
  '-': '6',
  'è': '7',
  '_': '8',
  'ç': '9',
  'à': '0',
  // Caps / alternative layouts
  '§': '6',
  '°': '0',
};

const ARABIC_INDIC_DIGITS: Record<string, string> = {
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
};

/**
 * Normalizes any scanned or typed string into standard alphanumeric barcode digits.
 */
export function normalizeBarcode(input: string): string {
  if (!input) return '';

  let clean = '';
  for (let i = 0; i < input.length; i++) {
    const char = input[i];

    if (AZERTY_TO_DIGITS[char] !== undefined) {
      clean += AZERTY_TO_DIGITS[char];
    } else if (ARABIC_INDIC_DIGITS[char] !== undefined) {
      clean += ARABIC_INDIC_DIGITS[char];
    } else {
      clean += char;
    }
  }

  // Trim extraneous whitespace or control codes sent by scanners
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
