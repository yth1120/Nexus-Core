import { AlertTriangle, Info } from 'lucide-react';
import type { ConfirmVariant } from '@/types';
import { Button } from './Button';
import { Modal } from './Modal';

interface ConfirmDialogProps {
  open: boolean;
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  variant?: ConfirmVariant;
  onConfirm: () => void;
  onCancel: () => void;
  loading?: boolean;
}

const variantConfig: Record<
  ConfirmVariant,
  {
    icon: typeof AlertTriangle;
    iconColor: string;
    buttonVariant: 'danger' | 'primary' | 'secondary';
  }
> = {
  danger: {
    icon: AlertTriangle,
    iconColor: 'text-error',
    buttonVariant: 'danger',
  },
  warning: {
    icon: AlertTriangle,
    iconColor: 'text-warning',
    buttonVariant: 'secondary',
  },
  info: {
    icon: Info,
    iconColor: 'text-primary',
    buttonVariant: 'primary',
  },
};

export function ConfirmDialog({
  open,
  title,
  message,
  confirmLabel = 'Confirm',
  cancelLabel = 'Cancel',
  variant = 'danger',
  onConfirm,
  onCancel,
  loading = false,
}: ConfirmDialogProps) {
  const config = variantConfig[variant];
  const Icon = config.icon;

  return (
    <Modal open={open} onClose={onCancel} title="" size="sm">
      <div className="flex flex-col items-center text-center gap-4 py-2">
        <div className="p-3 rounded-full bg-border">
          <Icon size={24} className={config.iconColor} />
        </div>
        <div>
          <h3 className="text-lg font-semibold text-text mb-1">{title}</h3>
          <p className="text-sm text-text-secondary">{message}</p>
        </div>
        <div className="flex gap-3 w-full mt-2">
          <Button variant="ghost" className="flex-1" onClick={onCancel}>
            {cancelLabel}
          </Button>
          <Button
            variant={config.buttonVariant}
            className="flex-1"
            onClick={onConfirm}
            loading={loading}
          >
            {confirmLabel}
          </Button>
        </div>
      </div>
    </Modal>
  );
}
