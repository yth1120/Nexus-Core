import { PanelLeftClose, CheckCircle2 } from 'lucide-react';
import { useAppStore } from '@/stores/appStore';
import { NAV_ITEMS } from '@/constants';
import { SidebarNavItem } from './SidebarNavItem';

export function Sidebar() {
  const sidebarOpen = useAppStore((s) => s.sidebarOpen);
  const toggleSidebar = useAppStore((s) => s.toggleSidebar);

  return (
    <aside
      className="relative z-10 flex shrink-0 flex-col border-r border-border/80 bg-card shadow-sm transition-all duration-200 ease-out"
      style={{ width: sidebarOpen ? 260 : 64 }}
    >
      {/* Top bar */}
      <div className="mb-2 flex h-14 items-center border-b border-border/70 px-3">
        {sidebarOpen ? (
          <>
            <div className="-m-1 flex items-center gap-2.5 overflow-hidden p-1">
              <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border border-border bg-bg">
                <svg viewBox="0 0 64 64" className="w-4 h-4" fill="none">
                  <polygon
                    points="32,4 56,18 56,46 32,60 8,46 8,18"
                    stroke="currentColor"
                    strokeWidth="2.5"
                    className="text-primary"
                    fill="none"
                  />
                  <circle cx="32" cy="32" r="5" className="fill-primary" />
                </svg>
              </div>
              <span className="whitespace-nowrap text-sm font-semibold tracking-tight text-text">
                Nexus Core
              </span>
            </div>
            <button
              onClick={toggleSidebar}
              className="ml-auto shrink-0 rounded-md p-1.5 text-text-secondary transition-colors hover:bg-bg hover:text-text"
              title="收起"
            >
              <PanelLeftClose size={16} />
            </button>
          </>
        ) : (
          <button
            onClick={toggleSidebar}
            className="-m-1 rounded-md p-1 transition-colors hover:bg-bg"
            title="展开"
          >
            <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border border-border bg-bg">
              <svg viewBox="0 0 64 64" className="w-4 h-4" fill="none">
                <polygon
                  points="32,4 56,18 56,46 32,60 8,46 8,18"
                  stroke="currentColor"
                  strokeWidth="2.5"
                  className="text-primary"
                  fill="none"
                />
                <circle cx="32" cy="32" r="5" className="fill-primary" />
              </svg>
            </div>
          </button>
        )}
      </div>

      {/* Navigation */}
      <nav className="scrollbar-thin flex-1 space-y-1 overflow-y-auto px-2.5 py-1">
        {NAV_ITEMS.map((item) => (
          <SidebarNavItem key={item.id} item={item} collapsed={!sidebarOpen} />
        ))}
      </nav>

      {/* Status */}
      {sidebarOpen && (
        <div className="px-3 py-3">
          <div className="rounded-md border border-border bg-bg px-3 py-2">
            <div className="flex items-center gap-1.5">
              <CheckCircle2 size={12} className="text-success" />
              <span className="text-xs text-text-secondary">就绪</span>
              <span className="text-[10px] text-text-secondary ml-auto">v2.4.1</span>
            </div>
          </div>
        </div>
      )}
    </aside>
  );
}
