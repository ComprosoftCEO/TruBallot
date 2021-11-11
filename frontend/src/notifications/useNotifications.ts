/* eslint-disable no-console */
import { useEffect } from 'react';
import useWebSocket, { ReadyState, SendMessage } from 'react-use-websocket';
import { isDev, NOTIFICATIONS_BASE_URL } from 'env';
import {
  SubscriptionAction,
  WebsocketSubscriptionList,
  WebsocketNotificationEvent,
  WebsocketSubscriptionResponse,
} from 'models/notifications';
import { useAccessToken } from 'redux/auth';

export interface UseNotificationConfig {
  subscribeTo: WebsocketSubscriptionList;
  onMessage?: (data: WebsocketNotificationEvent) => void;
}

/**
 * This hook should only be called on routes that are logged in
 */
export const useNotifications = (config: UseNotificationConfig): void => {
  const accessToken = useAccessToken();

  const { sendMessage, readyState } = useWebSocket(
    NOTIFICATIONS_BASE_URL,
    {
      protocols: ['wsevt', accessToken ?? ''],
      onOpen: (event) => handleOnOpen(event, sendMessage, config),
      onMessage: (event) => handleOnMessage(event, sendMessage, config),
      onClose: handleOnClose,
      onError: handleOnError,
      retryOnError: true,
      shouldReconnect: (event) => handleOnClose(event, true),
    },
    accessToken !== null,
  );

  // Update the subscriptions if the subscribeTo object changes
  useEffect(() => {
    if (readyState === ReadyState.OPEN) {
      sendMessage(JSON.stringify({ type: SubscriptionAction.Replace, ...config.subscribeTo }));
    }
    // We don't want "readyState" to be in the depencencies list
    //  Only call this method if the subscription object changes
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [config.subscribeTo, sendMessage]);
};

/**
 * Handle when the websocket is first opened
 *
 * @param event Websocket event
 * @param sendMessage Function to send a websocket message
 * @param config Hook configuration
 */
function handleOnOpen(event: WebSocketEventMap['open'], sendMessage: SendMessage, config: UseNotificationConfig): void {
  if (isDev()) {
    console.log('Notifications websocket is open and listening for events...');
  }
  sendMessage(JSON.stringify({ type: SubscriptionAction.Subscribe, ...config.subscribeTo }));
}

/**
 * Handle websocket messages
 *
 * @param event Event containinng the message
 * @param sendMessage Function to send a websocket message
 * @param config Hook configuration
 */
function handleOnMessage(
  event: WebSocketEventMap['message'],
  sendMessage: SendMessage,
  config: UseNotificationConfig,
): void {
  const { onMessage, subscribeTo } = config;

  // Try to parse the message as a response to updating our subscription
  const subscriptionResponse: WebsocketSubscriptionResponse = JSON.parse(event.data);
  if (subscriptionResponse.type === 'error') {
    sendMessage(JSON.stringify({ type: SubscriptionAction.Subscribe, ...subscribeTo }));
    return;
  }
  if (subscriptionResponse.type === 'success') {
    return;
  }

  // Okay, message must be an event
  const notificationMessage: WebsocketNotificationEvent = subscriptionResponse as any;
  if (isDev()) {
    console.log('Received event:', notificationMessage);
  }

  // Call the message handler if it is set
  if (onMessage !== undefined && notificationMessage?.type !== undefined) {
    onMessage(notificationMessage);
  }
}

/**
 * Error handler for the websocket
 *
 * @param event Websocket event
 * @returns True if the websocket should reconnect, false otherwise
 */
function handleOnClose(event: WebSocketEventMap['close'], reconnect = false): boolean {
  if (isDev() && !reconnect) {
    console.log(`Websocket closed: ${event.reason} (Code ${event.code})`);
  }

  // Reconnect if it was not a clean close
  return !event.wasClean;
}

/**
 * Error handler for the websocket
 */
function handleOnError(event: WebSocketEventMap['error']): void {
  // We could also log the error to an error reporting service
  console.error(event);
}
