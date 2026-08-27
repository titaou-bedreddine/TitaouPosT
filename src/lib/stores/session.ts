import { writable } from 'svelte/store';
import type { CashSession } from '../types';

export const activeSession = writable<CashSession | null>(null);