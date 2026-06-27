import type { NexusEventKey, NexusEventMap, NexusEventHandler } from '@/types/event';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyHandler = (data: any) => void;

class NexusEventBus {
  private handlers = new Map<NexusEventKey, Set<AnyHandler>>();

  emit<K extends NexusEventKey>(event: K, data: NexusEventMap[K]): void {
    const handlers = this.handlers.get(event);
    if (!handlers || handlers.size === 0) return;

    handlers.forEach((handler) => {
      try {
        handler(data);
      } catch (error) {
        console.error(`[EventBus] Error in "${event}" handler:`, error);
      }
    });
  }

  listen<K extends NexusEventKey>(event: K, handler: NexusEventHandler<K>): () => void {
    if (!this.handlers.has(event)) {
      this.handlers.set(event, new Set());
    }
    const set = this.handlers.get(event)!;
    set.add(handler as AnyHandler);

    return () => {
      set.delete(handler as AnyHandler);
      if (set.size === 0) {
        this.handlers.delete(event);
      }
    };
  }

  off<K extends NexusEventKey>(event: K, handler: NexusEventHandler<K>): void {
    const set = this.handlers.get(event);
    if (set) {
      set.delete(handler as AnyHandler);
      if (set.size === 0) {
        this.handlers.delete(event);
      }
    }
  }

  clear(): void {
    this.handlers.clear();
  }
}

export const eventBus = new NexusEventBus();
