import type { LucideIcon } from 'lucide-react';
import {
  Activity,
  List,
  Server,
  Shield,
  Globe,
  Terminal,
  BarChart2,
  Settings,
  Info,
} from 'lucide-react';

export interface NavItem {
  id: string;
  labelKey: string;
  icon: LucideIcon;
  path: string;
}

export const NAV_ITEMS: NavItem[] = [
  { id: 'dashboard', labelKey: 'nav.dashboard', icon: Activity, path: '/dashboard' },
  { id: 'profiles', labelKey: 'nav.profiles', icon: List, path: '/profiles' },
  { id: 'nodes', labelKey: 'nav.nodes', icon: Server, path: '/nodes' },
  { id: 'rules', labelKey: 'nav.rules', icon: Shield, path: '/rules' },
  { id: 'connections', labelKey: 'nav.connections', icon: Globe, path: '/connections' },
  { id: 'logs', labelKey: 'nav.logs', icon: Terminal, path: '/logs' },
  { id: 'statistics', labelKey: 'nav.statistics', icon: BarChart2, path: '/statistics' },
  { id: 'settings', labelKey: 'nav.settings', icon: Settings, path: '/settings' },
  { id: 'about', labelKey: 'nav.about', icon: Info, path: '/about' },
];
