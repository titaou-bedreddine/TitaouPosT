import { writable, derived } from 'svelte/store';
import type { CartItem, Product } from '../types';

export const cartItems = writable<CartItem[]>([]);
export const globalDiscountPercent = writable<number>(0);
export const globalDiscountAmount = writable<number>(0);
export const selectedCustomerId = writable<number | null>(null);
export const isRefundMode = writable<boolean>(false);

export function addToCart(product: Product, quantity = 1, asRefund = false) {
  cartItems.update((items) => {
    const existingIndex = items.findIndex(
      (item) => item.product_id === product.id && item.is_refund === asRefund
    );

    if (existingIndex > -1) {
      const updated = [...items];
      updated[existingIndex].quantity += quantity;
      updated[existingIndex].total_price =
        updated[existingIndex].quantity * (updated[existingIndex].unit_price - updated[existingIndex].discount_amount);
      return updated;
    } else {
      const newItem: CartItem = {
        product_id: product.id,
        sku: product.sku,
        barcode: product.barcodes[0] || '',
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
        const total = newQty * (item.unit_price - item.discount_amount);
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
        const disc = Math.min(discountPerUnit, item.unit_price);
        const total = item.quantity * (item.unit_price - disc);
        return { ...item, discount_amount: disc, total_price: total };
      }
      return item;
    })
  );
}

export function removeFromCart(productId: number, isRefund: boolean) {
  cartItems.update((items) =>
    items.filter((item) => !(item.product_id === productId && item.is_refund === isRefund))
  );
}

export function clearCart() {
  cartItems.set([]);
  globalDiscountPercent.set(0);
  globalDiscountAmount.set(0);
  selectedCustomerId.set(null);
  isRefundMode.set(false);
}

export const cartSubtotal = derived(cartItems, ($items) =>
  $items.reduce((sum, item) => (item.is_refund ? sum - item.total_price : sum + item.total_price), 0)
);

export const cartGrandTotal = derived(
  [cartSubtotal, globalDiscountAmount, globalDiscountPercent],
  ([$subtotal, $discAmount, $discPercent]) => {
    let total = $subtotal;
    if ($discAmount > 0) {
      total -= $discAmount;
    } else if ($discPercent > 0) {
      total -= Math.round((total * $discPercent) / 100);
    }
    return Math.max(0, total);
  }
);