import React, { useEffect, useRef } from 'react';
import ReCAPTCHA from 'react-google-recaptcha';

/**
 * Hook to generate a React ref for the various forms.
 * Also generates an effect to detect when the reCAPTCHA is canceled.
 *
 * @returns reCAPTCHA ref
 */
export const useReCAPTCHARef = (onCancel?: () => void): React.MutableRefObject<ReCAPTCHA | null> => {
  const recaptchaRef = useRef<ReCAPTCHA | null>(null);

  // Use observer to detect if capatcha window has been closed
  useEffect(() => createRecaptchaClosedObserver(recaptchaRef, onCancel), [onCancel, recaptchaRef]);

  return recaptchaRef;
};

/**
 * Hacky system to detech when the user clicks "close" on the reCAPTCHA component
 *
 * @returns Method to clean up the observer
 */
function createRecaptchaClosedObserver(captcha: React.RefObject<ReCAPTCHA>, onCaptchaClosed?: () => void): () => void {
  return detectWhenReCaptchaChallengeIsShown((reCaptchaChallengeOverlayDiv) => {
    const reCaptchaChallengeClosureObserver = new MutationObserver(() => {
      (async () => {
        if (reCaptchaChallengeOverlayDiv.style.visibility === 'hidden' && !captcha.current?.getValue()) {
          // Note: Disconnecting the observer doesn't seem to work anymore, not sure why
          //  The garbage collector should automatically remove this anyway
          //
          // Clean up the current object
          // reCaptchaChallengeClosureObserver.disconnect();

          // Perform any closed actions
          if (onCaptchaClosed !== undefined) {
            onCaptchaClosed();
          }
        }
      })();
    });

    // Detect changes in the style of the reCAPTCHA modal
    reCaptchaChallengeClosureObserver.observe(reCaptchaChallengeOverlayDiv, {
      attributes: true,
      attributeFilter: ['style'],
    });
  });
}

function detectWhenReCaptchaChallengeIsShown(resolve: (element: HTMLElement) => void): () => void {
  const reCaptchaObserver = new MutationObserver((mutationRecords) => {
    mutationRecords.forEach((mutationRecord) => {
      if (mutationRecord.addedNodes.length) {
        const reCaptchaParentContainer = mutationRecord.addedNodes[0] as HTMLElement;

        // Make sure this is the correct parent container for the iFrame
        if (reCaptchaParentContainer?.querySelectorAll) {
          const reCaptchaIframe = reCaptchaParentContainer?.querySelectorAll('iframe[title*="recaptcha"]');
          if (reCaptchaIframe.length > 0) {
            resolve(reCaptchaParentContainer);
          }
        }
      }
    });
  });

  // Watch the body for when any of its children change
  reCaptchaObserver.observe(document.body, { childList: true });

  // React hook must cleanup the function when the component goes out of scope
  return () => {
    reCaptchaObserver.disconnect();
  };
}
