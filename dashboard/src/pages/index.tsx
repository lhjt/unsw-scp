import { FunctionComponent } from 'react';
import Head from 'next/head';
import Wrapper from '../components/starter/wrapper';
import { EuiAccordion, EuiCallOut, EuiSpacer, EuiText } from '@elastic/eui';
import Header from '../components/starter/header';
import FlagCard from '../components/challenges/FlagCard';

const Index: FunctionComponent = () => {
  return (
    <>
      <Head>
        <title>UNSW SCP - Challenges</title>
      </Head>

      <Header />
      <div css={{ margin: '1rem' }}>
        <EuiText>
          <h1>Challenges</h1>
        </EuiText>
        <EuiSpacer />
        <EuiText>
          <p>
            The challenges that are visible are available for you to attempt to
            complete. Upon successfully completing a challenge, you will be
            supplied with a flag that you should submit to the relevant form on
            this page.
          </p>
        </EuiText>
        <EuiSpacer />
        <EuiCallOut size="m" title="Dynamic Flags" iconType="pin">
          <p>
            Some flags are dynamic. This means that they are independently
            generated for your account, and will not work for other people.
          </p>
        </EuiCallOut>
        <EuiSpacer />
        <EuiAccordion
          id="main-accord"
          className="euiAccordionForm"
          element="fieldset"
          buttonContent="Category 0"
          buttonClassName="euiAccordionForm__button"
          paddingSize="l">
          <div css={{ display: 'flex', flexWrap: 'wrap' }}>
            <FlagCard displayName="Welcome!" points={2} />
            <FlagCard
              points={1}
              displayName="Welcome!"
              submissionDetails="Submitted at 20220527T134302+10:00"
            />
          </div>
        </EuiAccordion>
      </div>
    </>
  );
};

export default Index;
