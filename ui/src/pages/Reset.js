import { Button } from 'semantic-ui-react';
import "../App.css";

const Reset = (props) => {

  const onClickReset = async () => {
    await props.endConnection();
    props.resetAccount();
  }

  const onClickCancel = async () => {

    if (props.isConnected) {
      props.hideWindow();
    } else {
      props.cancelReset();
    }
  }

  return (<>
    <div className='tightHeaderContainer'>
      <h1 className='mainheadline'>Delete Account</h1>
      <p className='subtext'>
        Caution: Without a your recovery phrase you will lose access to this account forever!
      </p>
    </div>
    <Button className="button ui danger" onClick={onClickReset}>DELETE ACCOUNT</Button>
    <div>
      <Button className="button ui two" onClick={onClickCancel}>Cancel</Button>
    </div>
  </>);
}

export default Reset;