import { Button, Icon, Loader, Modal, Table } from 'semantic-ui-react';
import { nestedSelectorHook } from 'redux/helpers';
import { Flex } from 'components/shared';
import { clearVerifySum } from './panesActions';

const useSelector = nestedSelectorHook('results');

export const VerifySumModal = () => {
  const sum = useSelector((state) => state.verifySum);
  const forwardBallots = useSelector((state) => state.questions[state.currentQuestionIndex].forwardBallots ?? '0');
  const reverseBallots = useSelector((state) => state.questions[state.currentQuestionIndex].reverseBallots ?? '0');

  if (sum.loading) {
    return (
      <Modal open>
        <Loader active indeterminate size="big" content="Computing Sum..." />
      </Modal>
    );
  }

  if (sum.data === null) {
    return null;
  }

  const forwardValid = BigInt(forwardBallots) === sum.data[0];
  const reverseValid = BigInt(reverseBallots) === sum.data[1];

  return (
    <Modal open size="small" onClose={clearVerifySum}>
      <Modal.Header style={{ textAlign: 'center' }}>Results:</Modal.Header>
      <Modal.Content>
        <div style={{ overflowX: 'auto' }}>
          <Table celled>
            <Table.Header>
              <Table.Row>
                <Table.HeaderCell colSpan="2" textAlign="center">
                  <Flex direction="row" justify="center" alignItems="center">
                    {forwardValid ? (
                      <Icon size="large" color="green" name="check circle outline" />
                    ) : (
                      <Icon size="large" color="red" name="cancel" />
                    )}
                    Forward Ballots
                  </Flex>
                </Table.HeaderCell>
              </Table.Row>
            </Table.Header>

            <Table.Body>
              <Table.Row positive={forwardValid} negative={!forwardValid}>
                <Table.Cell collapsing style={{ fontWeight: 'bold' }}>
                  Server Value
                </Table.Cell>
                <Table.Cell>{forwardBallots}</Table.Cell>
              </Table.Row>
              <Table.Row positive={forwardValid} negative={!forwardValid}>
                <Table.Cell collapsing style={{ fontWeight: 'bold' }}>
                  Computed Value
                </Table.Cell>
                <Table.Cell>{sum.data[0].toString(10)}</Table.Cell>
              </Table.Row>
            </Table.Body>
          </Table>

          <Table celled>
            <Table.Header>
              <Table.Row>
                <Table.HeaderCell colSpan="2" textAlign="center">
                  <Flex direction="row" justify="center" alignItems="center">
                    {reverseValid ? (
                      <Icon size="large" color="green" name="check circle outline" />
                    ) : (
                      <Icon size="large" color="red" name="cancel" />
                    )}
                    Reverse Ballots
                  </Flex>
                </Table.HeaderCell>
              </Table.Row>
            </Table.Header>

            <Table.Body>
              <Table.Row positive={reverseValid} negative={!reverseValid}>
                <Table.Cell collapsing style={{ fontWeight: 'bold' }}>
                  Server Value
                </Table.Cell>
                <Table.Cell>{reverseBallots}</Table.Cell>
              </Table.Row>
              <Table.Row positive={reverseValid} negative={!reverseValid}>
                <Table.Cell collapsing style={{ fontWeight: 'bold' }}>
                  Computed Value
                </Table.Cell>
                <Table.Cell>{sum.data[1].toString(10)}</Table.Cell>
              </Table.Row>
            </Table.Body>
          </Table>
        </div>
      </Modal.Content>

      <Modal.Actions style={{ textAlign: 'center' }}>
        <Button icon="cancel" content="Close" onClick={clearVerifySum} />
      </Modal.Actions>
    </Modal>
  );
};
