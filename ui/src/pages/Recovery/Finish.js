import { useOutletContext } from 'react-router-dom';
import { Button } from 'semantic-ui-react';
import "../../App.css";

const Finish = () => {

  const { goForward } = useOutletContext();

  return (<>
    <div className='header-container-fat'>
      <h1 className='main-headline'>You're all done!</h1>
      <h3 className='medium-sub-text'>It's time to start using the Manta Signer.</h3>
    </div>
    <Button className="button ui first wide" onClick={goForward}>
      Finish
    </Button>
  </>
  )
};

export default Finish;