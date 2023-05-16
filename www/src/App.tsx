// @ts-nocheck
import React from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import {
  faDiscord,
  faTwitter,
  faTelegram,
  faGithub,
} from '@fortawesome/free-brands-svg-icons';
import MantaLogo from './assets/manta.png';
import './App.css';


function App() {

  return (
    <div className='App bg-primary flex flex-grow flex-col'>
      <header className='h-16 flex flex-shrink-0 px-4 sm:px-6 justify-between items-center border border-gray-300 border-x-0 border-t-0'>
        <a
          className='flex gap-2 cursor-pointer'
          rel='noreferrer'
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
            rel='noreferrer'
            target='_blank'
            className='cursor-pointer'
          >
            <FontAwesomeIcon icon={faDiscord} />
          </a>
          <a
            href='https://twitter.com/mantanetwork'
            rel='noreferrer'
            target='_blank'
            className='cursor-pointer'
          >
            <FontAwesomeIcon icon={faTwitter} />
          </a>
          <a
            href='https://t.me/mantanetworkofficial'
            rel='noreferrer'
            target='_blank'
            className='cursor-pointer'
          >
            <FontAwesomeIcon icon={faTelegram} />
          </a>
          <a
            href='https://github.com/manta-network'
            rel='noreferrer'
            target='_blank'
            className='cursor-pointer'
          >
            <FontAwesomeIcon icon={faGithub} />
          </a>
        </div>
      </header>
      <div className='flex flex-col flex-grow justify-center items-center p-4'>
        <img src={MantaLogo} alt='Dolphin Network' className='sm:w-24 w-16' />
        <h1 className='mt-6 text-4xl sm:text-6xl'>
        The Manta Wallet: Your Gateway to Web3 Privacy
        </h1>
        <a
          className='px-6 py-4 text-white text-2xl bg-button rounded-full mt-12 cursor-pointer'
          onclick="window.fathom.trackGoal('KUBL03QU', 0);"
          href={'https://chrome.google.com/webstore/detail/manta-wallet/enabgbdfcbaehmbigakijjabdpdnimlg'}
          rel='noreferrer'
          target='_blank'
          title={'Download Manta Wallet'}
        >
          Download Manta Wallet
        </a>
        <p className='mt-10 text-secondary text-lg max-w-lg'>
        Use MantaPay on Calamari - MantaPay allows users to privatize public assets, transfer private assets, and convert private assets back into public assets.
        </p>
        <a
          href='https://app.manta.network'
          target='_blank'
          rel='noreferrer'
          className='text-2xl mt-6 text-thirdry'
        >
          Use MantaPay on Calamari
        </a>
      </div>
    </div>
  );
}

export default App;
