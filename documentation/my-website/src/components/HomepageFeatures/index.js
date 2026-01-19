import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: '🔐 Passkey Authentication',
    Svg: require('@site/static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
        Use biometric authentication (TouchID, FaceID, Windows Hello) or hardware security keys 
        instead of seed phrases. Your private keys never leave your device's secure element.
      </>
    ),
  },
  {
    title: '🛡️ Policy Protection',
    Svg: require('@site/static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
        Configure on-chain policies to control spending limits, time locks, and transaction rules. 
        Policies are enforced on-chain and cannot be bypassed.
      </>
    ),
  },
  {
    title: '🔄 Easy Recovery',
    Svg: require('@site/static/img/undraw_docusaurus_react.svg').default,
    description: (
      <>
        Register multiple passkeys across devices for account recovery. Never lose access to your 
        funds with multi-passkey support and social recovery options.
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
