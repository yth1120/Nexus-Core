import React from 'react';
import { FolderOpen } from 'lucide-react';
import { Button } from './Button';

interface EmptyProps {
  icon?: React.ComponentType<{ size?: number | string; className?: string }>;
  title: string;
  description?: string;
  action?: { label: string; onClick: () => void };
}

export function Empty({ icon: Icon = FolderOpen, title, description, action }: EmptyProps) {
  return (
    <div className="flex flex-col items-center justify-center gap-3 py-8 text-center">
      <Icon size={40} className="text-text-secondary/40" />
      <div>
        <p className="text-sm font-medium text-text-secondary">{title}</p>
        {description && (
          <p className="text-xs text-text-secondary/60 mt-1 max-w-xs">{description}</p>
        )}
      </div>
      {action && (
        <Button variant="primary" size="sm" onClick={action.onClick}>
          {action.label}
        </Button>
      )}
    </div>
  );
}
