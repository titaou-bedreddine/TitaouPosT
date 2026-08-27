import { writable } from 'svelte/store';
import type { User } from '../types';

export const currentUser = writable<User | null>(null);

export function hasPermission(user: User | null, permissionCode: string): boolean {
  if (!user) return false;
  if (user.role_name === 'Administrator' || user.role_id === 1) return true;
  return user.permissions.includes(permissionCode);
}