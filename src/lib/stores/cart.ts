import { writable, derived, get } from 'svelte/store';
import type { CartItem, Product, HeldSale } from '../types';
import { invoke } from '@tauri-apps/api/core';
import { selectedCustomerId, DEFAULT_WALKIN_CUSTOMER_ID } from './customers';

export const cartItems = writable<CartItem[]>([]);
export const globalDiscountMode = writable<'none' | 'percent' | 'amount'>('none');
export const globalDiscountValue = writable<number>(0);
export const isRefundMode = writable<boolean>(false);
export const heldSalesList = writable<HeldSale[]>([]);
export const lastAddedProductId = writable<number | null>(null);
export const heldNotification = writable<string | null>(null);
export const cartItemOrder = writable<'top' | 'bottom'>('bottom');

// Quantity-edit mode (F6): the cart line currently being edited, keyed by
// "productId[_ref]". null = not editing.
export const qtyEditTarget = writable<string | null>(null);

export function itemKey(item: { product_id: number; is_refund?: boolean }): string {
  return `${item.product_id}${item.is_refund ? '_ref' : ''}`;
}

export function startQtyEdit(item: { product_id: number; is_refund?: boolean }) {
  qtyEditTarget.set(itemKey(item));
}

export function stopQtyEdit() {
  qtyEditTarget.set(null);
}

// Kept for backward compatibility: the canonical store now lives in ./customers.
export { selectedCustomerId };

export function addToCart(product: Product, quantity = 1, asRefund = false) {
  lastAddedProductId.set(product.id);
  setTimeout(() => lastAddedProductId.set(null), 800);

  cartItems.update((items) => {
    const existingIndex = items.findIndex(
      (item) => item.product_id === product.id && item.is_refund === asRefund
    );

    if (existingIndex > -1) {
      const updated = [...items];
      updated[existingIndex].quantity += quantity;
      const unitNet = Math.max(0, updated[existingIndex].unit_price - updated[existingIndex].discount_amount);
      updated[existingIndex].total_price = Math.round(updated[existingIndex].quantity * unitNet);
      return updated;
    } else {
      const newItem: CartItem = {
        product_id: product.id,
        sku: product.sku,
        barcode: (product.barcodes && product.barcodes[0]) ? product.barcodes[0] : (product.sku || ''),
        name_ar: product.name_ar,
        name_fr: product.name_fr,
        name_en: product.name_en,
        image_path: product.image_path,
        unit_price: product.sale_price,
        quantity,
        discount_amount: 0,
        tax_amount: 0,
        total_price: product.sale_price * quantity,
        is_refund: asRefund,
      };
      
      const order = get(cartItemOrder);
      if (order === 'top') {
        return [newItem, ...items];
      } else {
        return [...items, newItem];
      }
    }
  });
}

export function updateItemQuantity(productId: number, isRefund: boolean, newQty: number) {
  if (newQty <= 0) {
    removeFromCart(productId, isRefund);
    return;
  }
  cartItems.update((items) =>
    items.map((item) => {
      if (item.product_id === productId && item.is_refund === isRefund) {
        const unitNet = Math.max(0, item.unit_price - item.discount_amount);
        const total = Math.round(newQty * unitNet);
        return { ...item, quantity: newQty, total_price: total };
      }
      return item;
    })
  );
}

export function applyItemDiscount(productId: number, isRefund: boolean, discountPerUnit: number) {
  cartItems.update((items) =>
    items.map((item) => {
      if (item.product_id === productId && item.is_refund === isRefund) {
        const disc = Math.min(Math.max(0, discountPerUnit), item.unit_price);
        const total = Math.round(item.quantity * (item.unit_price - disc));
        return { ...item, discount_amount: disc, total_price: total };
      }
      return item;
    })
  );
}

export function toggleItemRefund(productId: number, currentRefundState: boolean) {
  cartItems.update((items) =>
    items.map((item) => {
      if (item.product_id === productId && item.is_refund === currentRefundState) {
        return { ...item, is_refund: !currentRefundState };
      }
      return item;
    })
  );
}

export function toggleAllCartRefund() {
  cartItems.update((items) => {
    const anyNormal = items.some((i) => !i.is_refund);
    return items.map((i) => ({ ...i, is_refund: anyNormal }));
  });
}

export function removeFromCart(productId: number, isRefund: boolean) {
  cartItems.update((items) =>
    items.filter((item) => !(item.product_id === productId && item.is_refund === isRefund))
  );
}

export function clearCart() {
  cartItems.set([]);
  globalDiscountMode.set('none');
  globalDiscountValue.set(0);
  // New sale resets to the walk-in customer, not "no customer".
  selectedCustomerId.set(DEFAULT_WALKIN_CUSTOMER_ID);
  isRefundMode.set(false);
  stopQtyEdit();
}

// Effective cart-level discount in DZD; never exceeds the cart amount so the total can't go negative.
export function computeCartDiscount(subtotal: number, mode: 'none' | 'percent' | 'amount', value: number): number {
  const base = Math.max(0, subtotal);
  if (mode === 'percent' && value > 0) {
    return Math.min(base, Math.round((base * Math.min(100, value)) / 100));
  }
  if (mode === 'amount' && value > 0) {
    return Math.min(base, Math.round(value));
  }
  return 0;
}

// Held carts saved with a cart-level remise store { items, discountMode, discountValue };
// older rows are plain CartItem[] arrays, so parse both shapes.
export function parseHeldCart(json: string): { items: CartItem[]; discountMode: 'none' | 'percent' | 'amount'; discountValue: number } {
  try {
    const parsed = JSON.parse(json);
    if (Array.isArray(parsed)) {
      return { items: parsed as CartItem[], discountMode: 'none', discountValue: 0 };
    }
    if (parsed && Array.isArray(parsed.items)) {
      const mode = parsed.discountMode === 'percent' || parsed.discountMode === 'amount' ? parsed.discountMode : 'none';
      return { items: parsed.items as CartItem[], discountMode: mode, discountValue: Number(parsed.discountValue) || 0 };
    }
    return { items: [], discountMode: 'none', discountValue: 0 };
  } catch {
    return { items: [], discountMode: 'none', discountValue: 0 };
  }
}

export async function holdCurrentSale(note?: string): Promise<boolean> {
  const items = get(cartItems);
  if (items.length === 0) return false;

  try {
    const total = get(cartGrandTotal);
    const customerId = get(selectedCustomerId);
    const timeStr = new Date().toLocaleTimeString();
    const finalNote = note?.trim()
      ? `${note.trim()} • ${total.toLocaleString()} DZD`
      : `${total.toLocaleString()} DZD (${items.length} items - ${timeStr})`;

    await invoke('hold_sale', {
      customerId,
      cartDataJson: JSON.stringify({
        items,
        discountMode: get(globalDiscountMode),
        discountValue: get(globalDiscountValue),
      }),
      totalAmount: total,
      notes: finalNote,
    });

    clearCart();
    await refreshHeldSales();

    heldNotification.set(`Cart #${total.toLocaleString()} DZD (${items.length} items) held!`);
    setTimeout(() => heldNotification.set(null), 4000);
    return true;
  } catch (e) {
    console.error('Failed to hold sale:', e);
    return false;
  }
}

export async function refreshHeldSales() {
  try {
    const list = await invoke<HeldSale[]>('list_held_sales');
    heldSalesList.set(list);
  } catch (e) {
    console.error(e);
  }
}

function sumCartLines($items: CartItem[]): number {
  return $items.reduce((sum, item) => {
    const lineVal = item.total_price;
    return item.is_refund ? sum - lineVal : sum + lineVal;
  }, 0);
}

export const cartSubtotal = derived(cartItems, ($items) => sumCartLines($items));

// Cart-level remise actually applied, in DZD (clamped so it can never exceed the cart amount).
export const globalDiscountAmount = derived(
  [cartItems, globalDiscountMode, globalDiscountValue],
  ([$items, $mode, $val]) => computeCartDiscount(sumCartLines($items), $mode, $val)
);

export const globalDiscountPercent = derived(
  [globalDiscountMode, globalDiscountValue],
  ([$mode, $val]) => ($mode === 'percent' && $val > 0 ? Math.min(100, $val) : 0)
);

export const cartGrandTotal = derived(
  [cartItems, globalDiscountAmount],
  ([$items, $discount]) => sumCartLines($items) - $discount
);