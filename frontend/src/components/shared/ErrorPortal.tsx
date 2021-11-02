import { Message, MessageProps, SemanticTRANSITIONS, TransitionablePortal } from 'semantic-ui-react';
import styles from './errorPortal.module.scss';

type OmitMessageProps = Omit<
  MessageProps,
  'animation' | 'duration' | 'onReload' | 'icon' | 'header' | 'content' | 'style'
>;

export interface ErrorPortalProps extends OmitMessageProps {
  animation?: SemanticTRANSITIONS;
  duration?: number;
  onReload?: () => void;

  icon?: any | boolean;
  header?: string;
  content: string;
}

export const ErrorPortal = ({
  animation = 'fade up',
  duration = 500,
  onReload,
  icon = 'exclamation triangle',
  header,
  content,
  ...rest
}: ErrorPortalProps) => (
  <TransitionablePortal open transition={{ animation, duration }} onHide={onReload}>
    <div className={styles.container}>
      <div className={styles.flexbox}>
        {/* eslint-disable-next-line react/jsx-props-no-spreading */}
        <Message icon={icon} header={header} content={content} style={{ width: 'unset' }} {...rest} />
      </div>
    </div>
  </TransitionablePortal>
);
