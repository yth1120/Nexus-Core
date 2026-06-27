import { useLocation, useNavigate } from 'react-router-dom';
import { cn } from '@/utils';
import { useI18n } from '@/i18n';
import type { NavItem } from '@/constants';

interface SidebarNavItemProps {
  item: NavItem;
  collapsed?: boolean;
}

export function SidebarNavItem({ item, collapsed = false }: SidebarNavItemProps) {
  const location = useLocation();
  const navigate = useNavigate();
  const { t } = useI18n();

  const isActive = location.pathname === item.path;
  const Icon = item.icon;

  return (
    <button
      onClick={() => navigate(item.path)}
      title={collapsed ? t(item.labelKey) : undefined}
      className={cn(
        'group flex h-10 w-full items-center overflow-hidden rounded-md transition-colors duration-150',
        collapsed ? 'justify-center px-0' : 'gap-2.5 px-3',
        isActive
          ? 'bg-primary text-white shadow-sm'
          : 'text-text-secondary hover:bg-bg hover:text-text',
      )}
    >
      <Icon size={collapsed ? 20 : 18} className="shrink-0" />
      {!collapsed && (
        <span className="whitespace-nowrap text-sm font-medium">{t(item.labelKey)}</span>
      )}
    </button>
  );
}
