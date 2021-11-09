import { Button, Card, Icon, Message } from 'semantic-ui-react';
import { getErrorInformation } from 'api';
import { UserBallotResult } from 'models/election';
import { nestedSelectorHook } from 'redux/helpers';
import { clearVerifyResult, verifyBallot } from './panesActions';

const useSelector = nestedSelectorHook('results');
const useGlobalsSelector = nestedSelectorHook('globals');

export interface VerifyBallotCardProps {
  electionId: string;
  questionIndex: number;
  ballot: UserBallotResult;
  ballotIndex: number;
}

export const VerifyBallotCard = ({ electionId, questionIndex, ballot, ballotIndex }: VerifyBallotCardProps) => {
  const verifying = useSelector((state) => state.questions[questionIndex].ballots[ballotIndex].verifying);
  const currentUserId = useGlobalsSelector((state) => state.userId);

  const extra: JSX.Element = (() => {
    if (verifying.loading) {
      return <Message positive icon={{ name: 'circle notched', loading: true }} header="Verifying Ballot..." />;
    }

    if (!verifying.success) {
      return (
        <Message
          negative
          header="Failed to verify ballot!"
          content={getErrorInformation(verifying.error).description}
          onDismiss={() => clearVerifyResult(questionIndex, ballotIndex)}
        />
      );
    }

    if (verifying.data !== undefined) {
      const allPositive = verifying.data.subProtocol1 && verifying.data.subProtocol2;
      return (
        <Message
          positive={allPositive}
          warning={!allPositive}
          icon={allPositive ? 'check square outline' : 'exclamation'}
          list={[
            `Sub-Protocol 1: ${verifying.data.subProtocol1 ? 'Valid' : 'Invalid'}`,
            `Sub-Protocol 2: ${verifying.data.subProtocol2 ? 'Valid' : 'Invalid'}`,
          ]}
          onDismiss={() => clearVerifyResult(questionIndex, ballotIndex)}
        />
      );
    }

    return (
      <Button
        positive
        icon="check square outline"
        content="Verify Ballot"
        onClick={() => verifyBallot(electionId, questionIndex, ballotIndex)}
      />
    );
  })();

  return (
    <Card
      header={
        ballot.id === currentUserId ? (
          <Card.Header>
            <Icon name="user" />
            <u>Me</u>
          </Card.Header>
        ) : (
          ballot.name
        )
      }
      extra={extra}
    />
  );
};
