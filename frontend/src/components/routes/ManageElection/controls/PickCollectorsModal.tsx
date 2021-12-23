import { Button, Checkbox, Divider, Form, Message, Modal, Popup, Segment } from 'semantic-ui-react';
import { nestedSelectorHook } from 'redux/helpers';
import { Flex } from 'components/shared';
import { useState } from 'react';
import { closeCollectorModal, openVoting, toggleCollectorSelected } from './controlsActions';

const useSelector = nestedSelectorHook('manageElection');

export const PickCollectorsModal = () => {
  const [popupOpen, setPopupOpen] = useState(false);

  const modalOpen = useSelector((store) => store.pickCollectorsModalOpen && !store.openingVoting.loading);
  const electionDetails = useSelector((store) => store.electionDetails);
  const allCollectors = useSelector((store) => store.allCollectors);
  const collectorsSelected = useSelector((store) => store.collectorsSelected);

  // These checks are probably unnecessary
  if (allCollectors.loading || !allCollectors.success || electionDetails.loading || !electionDetails.success) {
    return null;
  }

  const tooFewRegistered = electionDetails.data.registered.length < 2 * collectorsSelected.size;
  return (
    <>
      <Modal size="tiny" open={modalOpen} onClose={closeCollectorModal}>
        <Modal.Header style={{ textAlign: 'center' }} content="Open Voting:" />
        <Modal.Content scrolling>
          <Flex direction="column" alignItems="center">
            <b>Choose at least two collectors for the election:</b>

            <Popup
              on="hover"
              wide
              position="right center"
              open={popupOpen && tooFewRegistered}
              onOpen={() => setPopupOpen(true)}
              onClose={() => setPopupOpen(false)}
              content={
                <Message
                  compact
                  icon="users"
                  content={`At least ${2 * collectorsSelected.size} registered users are needed for ${
                    collectorsSelected.size
                  } collectors (Only ${electionDetails.data.registered.length} currently registered)`}
                />
              }
              trigger={
                <Segment raised>
                  {allCollectors.data.length === 0 && <i>No collectors in system...</i>}
                  <Form>
                    {allCollectors.data.map((collector) => (
                      <Form.Field key={collector.id} error={tooFewRegistered && collectorsSelected.has(collector.id)}>
                        <Checkbox
                          label={collector.name}
                          checked={collectorsSelected.has(collector.id)}
                          onChange={() => toggleCollectorSelected(collector.id)}
                        />
                      </Form.Field>
                    ))}
                  </Form>
                </Segment>
              }
            />
          </Flex>
          <Divider />

          <Message
            warning
            icon="exclamation"
            content={
              <>
                <b>Notice: </b>
                This will open the election for voting and prevent any further registrations
              </>
            }
          />
        </Modal.Content>

        <Modal.Actions style={{ textAlign: 'center' }}>
          <Button size="large" icon="cancel" content="Cancel" onClick={closeCollectorModal} />
          <Button
            color="blue"
            size="large"
            icon="list ordered"
            content="Open Voting"
            onClick={() => openVoting(electionDetails.data.id)}
            disabled={collectorsSelected.size < 2 || tooFewRegistered}
          />
        </Modal.Actions>
      </Modal>
    </>
  );
};
