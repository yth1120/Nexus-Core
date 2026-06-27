import { createBrowserRouter, Navigate } from 'react-router-dom';
import { MainLayout } from '@/components/layout/MainLayout';
import { Dashboard } from '@/pages/Dashboard';
import { Profiles } from '@/pages/Profiles';
import { Nodes } from '@/pages/Nodes';
import { Rules } from '@/pages/Rules';
import { Connections } from '@/pages/Connections';
import { Logs } from '@/pages/Logs';
import { Statistics } from '@/pages/Statistics';
import { Settings } from '@/pages/Settings';
import { About } from '@/pages/About';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <MainLayout />,
    children: [
      { index: true, element: <Navigate to="/dashboard" replace /> },
      { path: 'dashboard', element: <Dashboard /> },
      { path: 'profiles', element: <Profiles /> },
      { path: 'nodes', element: <Nodes /> },
      { path: 'rules', element: <Rules /> },
      { path: 'connections', element: <Connections /> },
      { path: 'logs', element: <Logs /> },
      { path: 'statistics', element: <Statistics /> },
      { path: 'settings', element: <Settings /> },
      { path: 'about', element: <About /> },
    ],
  },
]);
