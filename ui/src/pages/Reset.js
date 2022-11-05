import { Button } from 'semantic-ui-react';
import "../App.css";

const Reset = ({
  isConnected,
  hideWindow,
  endConnection,
  restartServer,
  cancelReset
}) => {

  const onClickReset = async () => {
    await endConnection();
    restartServer(true,true);
  }

  const onClickCancel = async () => {

    if (isConnected) {
      hideWindow();
    } else {
      cancelReset();
    }
  }

  return (<>
    <div className='tightHeaderContainer'>
      <h1 className='mainheadline'>Delete Account</h1>
      <br/>
      <p className='subtext'>
        You are about to <strong>delete</strong> the account associated with this wallet.
      </p>
      <p className='subtext'>
        Without your recovery phrase saved you will lose access to this account and all linked funds <strong>forever</strong>!
      </p>
      <p className='subtext'>
      This process is irreversible, proceed at your own risk!
      </p>
    </div>
    <Button className="button ui danger" onClick={onClickReset}>DELETE ACCOUNT</Button>
    <div>
      <Button className="button ui two" onClick={onClickCancel}>Cancel</Button>
    </div>
  </>);
}

export default Reset;