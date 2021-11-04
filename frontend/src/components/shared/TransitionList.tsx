import React, { useLayoutEffect, useState } from 'react';
import lodash from 'lodash';
import { SemanticTRANSITIONS, Transition, TransitionPropDuration } from 'semantic-ui-react';

export interface TransitionListProps {
  children: React.ReactNode;
  animation?: SemanticTRANSITIONS | string;
  directional?: boolean;
  duration?: number | string | TransitionPropDuration;
  totalDuration?: number;
  minDelay?: number;
  maxDelay?: number;
}

export const TransitionList = ({
  children,
  animation,
  directional,
  duration,
  totalDuration = 1000,
  minDelay = 1,
  maxDelay = totalDuration,
}: TransitionListProps) => {
  const [currentIndex, setCurrentIndex] = useState(0);
  if (children === null) {
    return null;
  }

  const numberOfChildren = React.Children.count(children);
  if (numberOfChildren <= 0) {
    return null; /* Avoid divide-by-zero errors */
  }

  const delay = lodash.clamp(totalDuration / numberOfChildren, minDelay, maxDelay);

  return (
    <>
      {React.Children.map(
        children,
        (child, index) =>
          child &&
          index <= currentIndex && (
            <DelayedTransition
              animation={animation}
              directional={directional}
              duration={duration}
              delay={delay}
              currentIndex={currentIndex}
              setCurrentIndex={setCurrentIndex}
            >
              {child}
            </DelayedTransition>
          ),
      )}
    </>
  );
};

interface DelayedTransitionProps {
  children: React.ReactNode;
  animation?: SemanticTRANSITIONS | string;
  directional?: boolean;
  duration?: number | string | TransitionPropDuration;
  delay: number;
  currentIndex: number;
  setCurrentIndex: (input: number) => void;
}

function DelayedTransition({
  children,
  animation,
  directional,
  duration,
  delay,
  currentIndex,
  setCurrentIndex,
}: DelayedTransitionProps): JSX.Element {
  // Use a hook so we can clean up the "setInterval" call if the component unmounts prematurely
  useLayoutEffect(() => {
    const timeout = setTimeout(() => {
      setCurrentIndex(currentIndex + 1);
    }, delay);
    return () => clearTimeout(timeout);
  }, [currentIndex, delay, setCurrentIndex]);

  return (
    <Transition animation={animation} directional={directional} duration={duration} transitionOnMount>
      {children}
    </Transition>
  );
}
