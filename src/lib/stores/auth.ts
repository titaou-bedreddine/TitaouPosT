import { writable, derived } from 'svelte/store';
import type { User } from '../types';

export const currentUser = writable<User | null>({
  id: 1,
  username: 'admin',
  display_name: 'Administrator',
  role_id: 1,
  role_name: 'Administrator',
  max_discount_percent: 100,
  is_active: true,
  permissions: ['*'],
});

export const isAuthenticated = derived(currentUser, ($u) => !!$u);

export function logout() {
  currentUser.set(null);
}

export function hasPermission(user: User | null, permissionCode: string): boolean {
  if (!user) return false;
  if (user.role_name === 'Administrator' || user.role_id === 1) return true;
  return user.permissions.includes(permissionCode);
}