import { Header } from 'semantic-ui-react';
import loadingGif from '../icons/loading-gif.gif';
const Loading = ({
  isConnected
}) => {
  return (
    <>
      <Header as='h1'> Manta Signer </Header>
        <h3>Loading ...  Please Wait</h3>
      <img src={loadingGif} alt = "Loading ..." style={{width:"5rem"}}/>
    </>
  );
};

export default Loading;