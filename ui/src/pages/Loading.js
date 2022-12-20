import { Header } from 'semantic-ui-react';
import loadingGif from '../icons/loading-gif.gif';
const Loading = ({
  isConnected
}) => {
  return (
    <>
      <Header as='h1'> Manta Signer </Header>
      {!isConnected ? <h3>Downloading Manta Proving Keys...</h3> : null}
      <img src={loadingGif} alt = "Loading ..." style={{width:"5rem"}}/>
    </>
  );
};

export default Loading;