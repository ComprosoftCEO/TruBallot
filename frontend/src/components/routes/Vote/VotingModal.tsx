import { nestedSelectorHook } from 'redux/helpers';
import { Button, Message, Modal } from 'semantic-ui-react';

const useSelector = nestedSelectorHook('vote');

export const VotingModal = (): JSX.Element => {
  const questions = useSelector((state) => state.questions);

  return (
    <Modal open centered size="small">
      <Modal.Header style={{ textAlign: 'center' }}>Submitting Votes...</Modal.Header>
      <Modal.Content scrolling>
        {questions.map((question, i) => (
          <Message
            key={question.id}
            positive
            icon={{ name: 'circle notched', loading: true }}
            header={`${i + 1}. ${question.name}`}
          />
        ))}
      </Modal.Content>
      <Modal.Actions>
        <Button>Cancel</Button>
        <Button primary>Ok</Button>
      </Modal.Actions>
    </Modal>
  );
};
