import { Outlet } from 'react-router-dom';
import { Sidebar } from './Sidebar';

export function MainLayout() {
  return (
    <div className="flex h-full w-full overflow-hidden bg-bg text-text">
      <Sidebar />
      <main className="relative flex min-w-0 flex-1 flex-col overflow-hidden bg-bg">
        <Outlet />
      </main>
    </div>
  );
}
