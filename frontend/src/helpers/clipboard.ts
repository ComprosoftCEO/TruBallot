import { useRef, useState, useCallback } from 'react';
import copy from 'copy-to-clipboard';

export interface ClipboardProps {
  value: string;
  isCopied: boolean;
  copyToClipboard: () => void;
}

export const useCopyClipboard = (value: string, timeoutSec = 4): ClipboardProps => {
  const [isCopied, setCopied] = useState(false);
  const lastTimer = useRef<null | NodeJS.Timeout>(null);

  const copyToClipboard = useCallback(() => {
    copy(value);
    setCopied(true);

    if (lastTimer.current !== null) {
      clearTimeout(lastTimer.current);
    }

    lastTimer.current = setTimeout(() => {
      setCopied(false);
      lastTimer.current = null;
    }, timeoutSec * 1000);
  }, [timeoutSec, value]);

  return { value, isCopied, copyToClipboard };
};
