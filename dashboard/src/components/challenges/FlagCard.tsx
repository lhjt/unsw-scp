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
  submissionDetails?: string;
  points: number;
}

const FlagCard: FunctionComponent<FlagCardProps> = ({
  displayName,
  submissionDetails,
  points,
}) => {
  return (
    <EuiCard
      css={{ margin: '1rem', flexBasis: 400 }}
      textAlign="left"
      title={displayName}>
      <EuiBadge>
        {points > 1 || points == 0 ? `${points} Points` : `1 Point`}
      </EuiBadge>
      {submissionDetails ? (
        <EuiBadge color="success">{submissionDetails}</EuiBadge>
      ) : (
        <EuiBadge color="danger">Unsubmitted</EuiBadge>
      )}
      <EuiSpacer />

      <EuiText size="s">
        <h4>Related Services</h4>
        <EuiCard
          onClick={() => {
            window.open('https://hello.ctf.local.host/', '_blank');
          }}
          css={{ marginBottom: '1rem', marginTop: '1rem' }}
          title="hello.ctf.local.host">
          <EuiLink>Visit this service</EuiLink>
        </EuiCard>
      </EuiText>
      <EuiSpacer />
      {!submissionDetails && (
        <EuiFieldText
          fullWidth
          placeholder="Submit flag"
          append={<EuiButtonEmpty size="xs">Submit</EuiButtonEmpty>}
        />
      )}
    </EuiCard>
  );
};

export default FlagCard;
