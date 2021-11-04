import { Header, Loader, Modal } from 'semantic-ui-react';
import { Prompt } from 'react-router-dom';

export interface FinishingModalProps {
  open?: boolean;
}

export const FinishingModal = ({ open }: FinishingModalProps) => (
  <>
    <Modal basic size="large" open={open}>
      <Loader indeterminate size="big" active>
        <Header inverted>Closing Election and Computing Final Tally...</Header>
        <p>
          This might take awhile depending on the election size.
          <br />
          Please do not navigate away from this page.
        </p>
      </Loader>
    </Modal>
    <Prompt when={open} message="Cancel closing election?" />
  </>
);
