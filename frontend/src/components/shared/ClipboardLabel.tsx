import { Button, Label, Popup } from 'semantic-ui-react';
import { Flex } from 'components/shared';
import { useCopyClipboard } from 'helpers/clipboard';

export interface ClipboardLabelProps {
  value: string;
  disabled?: boolean;
}

export const ClipboardLabel = ({ value, disabled = false }: ClipboardLabelProps) => {
  const { isCopied, copyToClipboard } = useCopyClipboard(value, 1);

  return (
    <Flex alignItems="center">
      <Label
        basic
        style={{
          width: '100%',
          fontFamily: 'monospace',
          fontWeight: 'normal',
        }}
        content={value}
        disabled={disabled}
      />
      <Popup
        open={isCopied}
        on={[]}
        position="right center"
        trigger={
          <Button
            basic={isCopied ? undefined : true}
            compact
            icon="clipboard"
            onClick={copyToClipboard}
            color={isCopied ? 'red' : undefined}
            disabled={disabled}
          />
        }
        disabled={disabled}
      >
        <b style={{ color: 'red' }}>Copied</b>
      </Popup>
    </Flex>
  );
};
