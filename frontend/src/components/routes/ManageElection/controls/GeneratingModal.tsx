import { Header, Loader, Modal } from 'semantic-ui-react';

export interface GeneratingModalProps {
  open?: boolean;
}

export const GeneratingModal = ({ open }: GeneratingModalProps) => (
  <Modal basic size="large" open={open}>
    <Loader indeterminate size="big" active>
      <Header inverted>Generating Election Parameters...</Header>
      <p>
        This might take awhile depending on the election size.
        <br />
        Please do not navigate away from this page.
      </p>
    </Loader>
  </Modal>
);
