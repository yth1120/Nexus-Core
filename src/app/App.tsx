import { RouterProvider } from 'react-router-dom';
import { Providers } from './Providers';
import { router } from './Router';
import { ToastContainer } from '@/components/common/Toast';
import { useMockStream } from '@/hooks/useMockStream';
import { useEffect } from 'react';
import { useProfileStore } from '@/stores/profileStore';
import { useNodeStore } from '@/stores/nodeStore';
import { useRuleStore } from '@/stores/ruleStore';
import { useConnectionStore } from '@/stores/connectionStore';
import { useStatisticsStore } from '@/stores/statisticsStore';

export default function App() {
  useMockStream(true);

  // Prefetch initial data
  useEffect(() => {
    useProfileStore.getState().fetchProfiles();
    useNodeStore.getState().fetchNodes();
    useRuleStore.getState().fetchRules();
    useConnectionStore.getState().fetchConnections();
    useStatisticsStore.getState().fetchStatistics();
  }, []);

  return (
    <Providers>
      <RouterProvider router={router} />
      <ToastContainer />
    </Providers>
  );
}
