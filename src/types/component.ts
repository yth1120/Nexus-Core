import type React from 'react';

// ===== Button =====

export type ButtonVariant = 'primary' | 'secondary' | 'danger' | 'ghost';
export type ButtonSize = 'sm' | 'md' | 'lg';

// ===== Modal =====

export type ModalSize = 'sm' | 'md' | 'lg' | 'xl' | 'full';

// ===== Toast =====

export type ToastType = 'success' | 'warning' | 'error' | 'info';

export interface Toast {
  id: string;
  type: ToastType;
  title?: string;
  message: string;
  duration: number;
}

// ===== Table =====

export type TableSortDirection = 'asc' | 'desc';

export interface TableColumn<T> {
  key: string;
  label: string;
  sortable?: boolean;
  render: (item: T) => React.ReactNode;
  width?: string;
  align?: 'left' | 'center' | 'right';
  className?: string;
}

// ===== Select =====

export interface SelectOption {
  label: string;
  value: string;
}

// ===== Dropdown =====

export interface DropdownItem {
  label: string;
  onClick: () => void;
  icon?: React.ComponentType<{ size?: number | string }>;
  disabled?: boolean;
}

// ===== Badge =====

export type BadgeVariant = 'default' | 'success' | 'warning' | 'error' | 'info';

// ===== Confirm Dialog =====

export type ConfirmVariant = 'danger' | 'warning' | 'info';
