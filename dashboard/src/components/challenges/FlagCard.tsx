import {
  EuiBadge,
  EuiButtonEmpty,
  EuiCard,
  EuiFieldText,
  EuiIcon,
  EuiLink,
  EuiSpacer,
  EuiText,
} from '@elastic/eui';
import { FunctionComponent } from 'react';

interface FlagCardProps {
  displayName: string;
}

const FlagCard: FunctionComponent<FlagCardProps> = ({ displayName }) => {
  return (
    <EuiCard
      css={{ margin: '1rem', flexBasis: 300 }}
      textAlign="left"
      title={displayName}>
      <EuiBadge>1 Point</EuiBadge>
      <EuiBadge color="danger">Unsubmitted</EuiBadge>
      <EuiSpacer />

      <EuiText size="s">
        <h4>Related Services</h4>
        <EuiCard
          onClick={() => {
            window.open('https://www.google.com/', '_blank');
          }}
          css={{ marginBottom: '1rem', marginTop: '1rem' }}
          title="hello.ctf.local.host">
          <EuiLink>Visit this service</EuiLink>
        </EuiCard>
      </EuiText>
      <EuiSpacer />
      <EuiFieldText
        placeholder="Submit flag"
        append={<EuiButtonEmpty size="xs">Submit</EuiButtonEmpty>}
      />
    </EuiCard>
  );
};

export default FlagCard;
