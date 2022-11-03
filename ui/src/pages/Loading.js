import { Header } from 'semantic-ui-react';
import loadingGif from '../icons/loading-gif.gif';
const Loading = () => {
  return (
    <>
      <Header as='h1'> Manta Signer </Header>
      <img src={loadingGif} alt = "Loading ..." style={{width:"5rem"}}/>
    </>
  );
};

export default Loading;
