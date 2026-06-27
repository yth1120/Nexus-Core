import { ChevronLeft, ChevronRight } from 'lucide-react';
import { cn } from '@/utils';

interface PaginationProps {
  page: number;
  totalPages: number;
  onPageChange: (page: number) => void;
}

export function Pagination({ page, totalPages, onPageChange }: PaginationProps) {
  if (totalPages <= 1) return null;

  function getPages(): (number | '...')[] {
    const pages: (number | '...')[] = [];
    const maxVisible = 5;

    if (totalPages <= maxVisible + 2) {
      for (let i = 1; i <= totalPages; i++) pages.push(i);
    } else {
      pages.push(1);
      if (page > 3) pages.push('...');

      const start = Math.max(2, page - 1);
      const end = Math.min(totalPages - 1, page + 1);

      for (let i = start; i <= end; i++) pages.push(i);

      if (page < totalPages - 2) pages.push('...');
      pages.push(totalPages);
    }

    return pages;
  }

  return (
    <div className="flex items-center justify-center gap-1 py-3">
      <button
        onClick={() => onPageChange(page - 1)}
        disabled={page <= 1}
        className="p-1.5 rounded-lg text-text-secondary hover:text-text hover:bg-border disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
        aria-label="Previous page"
      >
        <ChevronLeft size={18} />
      </button>

      {getPages().map((p, i) =>
        p === '...' ? (
          <span key={`dots-${i}`} className="px-2 text-text-secondary text-sm">
            ...
          </span>
        ) : (
          <button
            key={p}
            onClick={() => onPageChange(p)}
            className={cn(
              'w-8 h-8 rounded-lg text-sm font-medium transition-colors',
              p === page
                ? 'bg-primary text-white'
                : 'text-text-secondary hover:text-text hover:bg-border',
            )}
          >
            {p}
          </button>
        ),
      )}

      <button
        onClick={() => onPageChange(page + 1)}
        disabled={page >= totalPages}
        className="p-1.5 rounded-lg text-text-secondary hover:text-text hover:bg-border disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
        aria-label="Next page"
      >
        <ChevronRight size={18} />
      </button>
    </div>
  );
}
