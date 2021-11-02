import { SemanticTRANSITIONS, TransitionablePortal } from 'semantic-ui-react';
import styles from './errorPortal.module.scss';

export interface ErrorPortalProps {
  children: React.ReactNode;
  animation?: SemanticTRANSITIONS;
  duration?: number;
  onReload?: () => void;
}

export const ErrorPortal = ({ children, animation = 'fade up', duration = 500, onReload }: ErrorPortalProps) => (
  <TransitionablePortal open transition={{ animation, duration }} onHide={onReload}>
    <div className={styles.container}>
      <div className={styles.flexbox}>{children}</div>
    </div>
  </TransitionablePortal>
);
