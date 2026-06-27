import { Card } from '@/components/common/Card';
import { Button } from '@/components/common/Button';
import { useToast } from '@/hooks/useToast';

export function About() {
  const toast = useToast();

  return (
    <div className="app-page-scroll scrollbar-thin flex items-center justify-center">
      <Card className="w-full max-w-lg text-center" padding="lg">
        {/* App Icon */}
        <div className="mb-6 flex justify-center">
          <div className="w-16 h-16 rounded-lg bg-card border border-border flex items-center justify-center">
            <svg viewBox="0 0 64 64" className="w-8 h-8" fill="none">
              <polygon
                points="32,4 56,18 56,46 32,60 8,46 8,18"
                stroke="currentColor"
                strokeWidth="2"
                className="text-primary"
                fill="none"
              />
              <circle cx="32" cy="32" r="5" className="fill-primary" />
            </svg>
          </div>
        </div>

        {/* App Info */}
        <h1 className="text-2xl font-bold text-text mb-1">Nexus Core</h1>
        <p className="text-text-secondary text-sm mb-6">网络工具</p>

        {/* Version Info */}
        <div className="space-y-2 mb-8">
          <div className="flex justify-between items-center px-4 py-2 bg-bg rounded-lg border border-border">
            <span className="text-sm text-text-secondary">核心版本</span>
            <span className="text-sm font-mono font-medium text-text">v1.4.2</span>
          </div>
          <div className="flex justify-between items-center px-4 py-2 bg-bg rounded-lg border border-border">
            <span className="text-sm text-text-secondary">界面版本</span>
            <span className="text-sm font-mono font-medium text-text">v2.4.1</span>
          </div>
          <div className="flex justify-between items-center px-4 py-2 bg-bg rounded-lg border border-border">
            <span className="text-sm text-text-secondary">构建日期</span>
            <span className="text-sm font-mono font-medium text-text">2026.06.24</span>
          </div>
        </div>

        {/* Actions */}
        <div className="flex flex-col gap-3">
          <Button
            variant="primary"
            className="w-full"
            onClick={() => toast.info('当前已是最新版本')}
          >
            检查更新
          </Button>
        </div>

        {/* Copyright */}
        <div className="mt-8 pt-6 border-t border-border">
          <p className="text-xs text-text-secondary">© 2026 Nexus Core</p>
        </div>
      </Card>
    </div>
  );
}
