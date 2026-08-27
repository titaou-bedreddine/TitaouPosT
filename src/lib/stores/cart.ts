import { writable, derived, get } from 'svelte/store';
import type { CartItem, Product, HeldSale } from '../types';
import { invoke } from '@tauri-apps/api/core';

export const cartItems = writable<CartItem[]>([]);
export const globalDiscountMode = writable<'none' | 'percent' | 'amount'>('none');
export const globalDiscountValue = writable<number>(0);
export const globalDiscountAmount = writable<number>(0);
export const globalDiscountPercent = writable<number>(0);
export const selectedCustomerId = writable<number | null>(null);
export const isRefundMode = writable<boolean>(false);
export const heldSalesList = writable<HeldSale[]>([]);
export const lastAddedProductId = writable<number | null>(null);
export const heldNotification = writable<string | null>(null);

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
      return [...items, newItem];
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
  globalDiscountAmount.set(0);
  globalDiscountPercent.set(0);
  selectedCustomerId.set(null);
  isRefundMode.set(false);
}

export async function holdCurrentSale(note = 'Auto-held for New Sale'): Promise<boolean> {
  const items = get(cartItems);
  if (items.length === 0) return false;

  try {
    const total = get(cartGrandTotal);
    const customerId = get(selectedCustomerId);
    const timeStr = new Date().toLocaleTimeString();
    await invoke('hold_sale', {
      customerId,
      cartDataJson: JSON.stringify(items),
      totalAmount: total,
      notes: `${note} (${items.length} items - ${timeStr})`,
    });

    clearCart();
    await refreshHeldSales();

    heldNotification.set(`Cart with ${items.length} items moved to Held Sales!`);
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

export const cartSubtotal = derived(cartItems, ($items) => {
  return $items.reduce((sum, item) => {
    const lineVal = item.total_price;
    return item.is_refund ? sum - lineVal : sum + lineVal;
  }, 0);
});

export const cartGrandTotal = derived(
  [cartItems, globalDiscountMode, globalDiscountValue],
  ([$items, $mode, $val]) => {
    let subtotal = $items.reduce((sum, item) => {
      const lineVal = item.total_price;
      return item.is_refund ? sum - lineVal : sum + lineVal;
    }, 0);

    if ($mode === 'percent' && $val > 0) {
      const discount = Math.round((subtotal * Math.min(100, $val)) / 100);
      subtotal -= discount;
    } else if ($mode === 'amount' && $val > 0) {
      subtotal -= $val;
    }

    return subtotal;
  }
);