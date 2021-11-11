import { WebsocketNotificationEvent } from 'notifications';

// Helpful TypeScript code is from:
//   https://stackoverflow.com/questions/50125893/typescript-derive-map-from-discriminated-union
type DiscriminateUnion<T, K extends keyof T, V extends T[K]> = T extends Record<K, V> ? T : never;
type DiscriminantMap<T extends Record<K, string | number>, K extends keyof T> = {
  [V in T[K]]: (input: DiscriminateUnion<T, K, V>) => void;
};

export type NotificationHandlerMap = Partial<DiscriminantMap<WebsocketNotificationEvent, 'type'>>;

/**
 * Higher-order function to build the notification handler from an input map
 *
 * @param map Map of notification handlers
 * @returns Function to use for the "onMessage()" function in useNotifications() hook
 */
export function buildNotificationHandler(map: NotificationHandlerMap): (input: WebsocketNotificationEvent) => void {
  return (input: WebsocketNotificationEvent): void => {
    const handler = map[input.type];
    if (handler !== undefined) {
      handler(input as any);
    }
  };
}
