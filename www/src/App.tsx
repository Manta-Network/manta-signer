import React, { useEffect, useState } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import {
  faDiscord,
  faTwitter,
  faTelegram,
  faGithub,
} from '@fortawesome/free-brands-svg-icons';

import DolphinLogo from './assets/dolphin.svg';
import MantaLogo from './assets/manta.png';
import './App.css';

function App() {
  const [os, setOs] = useState<string | undefined>('');

  const getOS = () => {
    const os = ['Linux', 'Mac', 'Windows'];
    return os.find((v) => navigator.appVersion.contains(v));
  };

  useEffect(() => {
    setOs(getOS());
  }, []);

  return (
    <div className='App bg-primary h-screen flex flex-col'>
      <header className='h-16 flex flex-shrink-0 px-4 sm:px-6 justify-between items-center border border-gray-300 border-x-0 border-t-0'>
        <a
          className='flex gap-2 cursor-pointer'
          href='http://manta.network/'
          target='_blank'
        >
          <img src={MantaLogo} alt='Manta Network' className='w-6 h-6' />
          <div className='text-primary'>
            <strong>MANTA</strong>&nbsp;NETWORK
          </div>
        </a>
        <div className='flex items-center gap-3 text-primary pr-0 sm:px-4'>
          <a
            href='https://discord.gg/PRDBTChSsF'
            target='_blank'
            className='cursor-pointer'
          >
            <FontAwesomeIcon icon={faDiscord} />
          </a>
          <a
            href='https://twitter.com/mantanetwork'
            target='_blank'
            className='cursor-pointer'
          >
            <FontAwesomeIcon icon={faTwitter} />
          </a>
          <a
            href='https://t.me/mantanetworkofficial'
            target='_blank'
            className='cursor-pointer'
          >
            <FontAwesomeIcon icon={faTelegram} />
          </a>
          <a
            href='https://github.com/manta-network'
            target='_blank'
            className='cursor-pointer'
          >
            <FontAwesomeIcon icon={faGithub} />
          </a>
        </div>
      </header>
      <div className='flex flex-col flex-grow justify-center items-center p-4'>
        <img src={DolphinLogo} alt='Dolphin Network' className='sm:w-24 w-16' />
        <h1 className='mt-6 text-4xl sm:text-6xl'>
          Your Gateway to Web3 Privacy.
        </h1>
        <a className='px-6 py-4 text-white text-2xl bg-button rounded-full mt-12 cursor-pointer'
          href=
          {os === 'Mac'
            ? 'https://github.com/Manta-Network/manta-signer/releases/download/0.6.0/manta-signer-macos-latest_0.6.0-103_x64.dmg'
            : os === 'Windows'
              ? 'https://github.com/Manta-Network/manta-signer/releases/download/0.6.0/manta-signer-windows-2019_0.6.0.102_x64.msi'
              : 'https://github.com/Manta-Network/manta-signer/releases/download/0.6.0/manta-signer-ubuntu-18.04_0.6.0_amd64.deb'}
          title=
          {os === 'Mac'
            ? "Manta Signer for macOS 10.15 or later"
            : os === 'Windows'
              ? 'Manta Signer for Windows 10 or later'
              : 'Manta Signer for Ubuntu 18.04 or later'}
        >
          Download Manta Signer
        </a>
        <p className='text-center text-secondary text-2xl mt-3'>
          {os === 'Mac'
            ? 'For macOS 10.15 or later'
            : os === 'Windows'
              ? 'For Windows 10 or later'
              : 'For Ubuntu 18.04 or later'}
        </p>
        <p className='mt-10 text-secondary text-lg max-w-lg'>
          Manta Signer enables access to all privacy tools and services on Manta Network, Calamari Network, and Dolphin Testnet.
        </p>
        <a
          href='https://docs.manta.network/docs/concepts/Signer?utm_source=website&utm_id=manta-signer-landing-page'
          target='_blank'
          className='text-2xl mt-6 text-thirdry'
        >
          Learn More about Manta Signer
        </a>

        <p className='mt-10 text-secondary text-lg max-w-lg'>
          Try MantaPay on the Dolphin Testnet - MantaPay allows users to privatize public assets, transfer private assets, and convert private assets back into public assets.
        </p>
        <a
          href='https://app.dolphin.manta.network/?utm_source=website&utm_id=manta-signer-landing-page#/transact'
          target='_blank'
          className='text-2xl mt-6 text-thirdry'
        >
          Try Dolphin Testnet
        </a>

      </div>
    </div >
  );
}

export default App;
