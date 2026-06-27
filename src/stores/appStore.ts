import { create } from 'zustand';
import type { AppStore } from '@/types/store';
import type { Toast } from '@/types';

let toastCounter = 0;
const DEFAULT_TOAST_DURATION = 4000;

export const useAppStore = create<AppStore>((set, get) => ({
  sidebarOpen: true,
  toasts: [],
  activeModal: null,
  modalData: null,

  addToast: (toast) => {
    const id = `toast-${++toastCounter}`;
    const newToast: Toast = {
      id,
      ...toast,
      duration: toast.duration ?? DEFAULT_TOAST_DURATION,
    };
    set((state) => ({ toasts: [...state.toasts, newToast] }));
  },

  removeToast: (id) => set((state) => ({ toasts: state.toasts.filter((t) => t.id !== id) })),

  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),

  openModal: (id, data) => set({ activeModal: id, modalData: data ?? null }),

  closeModal: () => set({ activeModal: null, modalData: null }),
}));
