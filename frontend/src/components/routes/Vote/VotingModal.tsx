import { useHistory } from 'react-router-dom';
import { Button, Message, Modal } from 'semantic-ui-react';
import { getErrorInformation } from 'api';
import { nestedSelectorHook } from 'redux/helpers';
import { VotingStatus } from 'redux/state';
import { changeBallot, useElectionId, vote } from './voteActions';

const useSelector = nestedSelectorHook('vote');

export const VotingModal = (): JSX.Element => {
  const electionId = useElectionId();
  const questions = useSelector((state) => state.questions);
  const status = useSelector((state) => state.votingStatus);

  const history = useHistory();

  return (
    <Modal
      open
      centered
      size="small"
      onClose={() => {
        if (status === VotingStatus.Error) changeBallot();
      }}
    >
      <Modal.Header style={{ textAlign: 'center' }}>
        {
          {
            [VotingStatus.Init]: 'Submitting Votes...',
            [VotingStatus.Voting]: 'Submitting Votes...',
            [VotingStatus.Error]: 'One or more ballots had errors...',
            [VotingStatus.Success]: 'Vote Successful!',
          }[status]
        }
      </Modal.Header>
      <Modal.Content scrolling>
        {questions.map((question, i) => {
          if (question.voting.loading) {
            return (
              <Message
                key={question.id}
                positive
                icon={{ name: 'circle notched', loading: true }}
                header={`${i + 1}. ${question.name}`}
              />
            );
          }

          return question.voting.success ? (
            <Message
              key={question.id}
              color="green"
              header={`${i + 1}. ${question.name}`}
              icon={question.voting.data ? 'check square outline' : 'check circle outline'}
              content={question.voting.data ? 'Vote Successful!' : 'Already Voted'}
            />
          ) : (
            <Message
              key={question.id}
              negative
              icon="exclamation triangle"
              header={`${i + 1}. Failed to cast vote!`}
              content={getErrorInformation(question.voting.error).description}
            />
          );
        })}
      </Modal.Content>

      {status === VotingStatus.Error && (
        <Modal.Actions style={{ textAlign: 'center' }}>
          <Button primary size="large" icon="redo" content="Retry" onClick={() => vote(electionId, true)} />
          <Button size="large" icon="check square outline" content="Change Ballot" onClick={changeBallot} />
        </Modal.Actions>
      )}

      {status === VotingStatus.Success && (
        <Modal.Actions style={{ textAlign: 'center' }}>
          <Button
            primary
            size="large"
            icon="list ordered"
            content="View Results"
            onClick={() => history.push(`/elections/${electionId}/results`)}
          />

          <Button
            size="large"
            icon="left arrow"
            content="Go Home"
            onClick={() => history.push(`/elections/${electionId}`)}
          />
        </Modal.Actions>
      )}
    </Modal>
  );
};
