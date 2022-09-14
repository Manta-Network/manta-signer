import { Button} from 'semantic-ui-react';
import "../fonts/ibm-plex/css/styles.css";

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
    <Button className="button ui danger" onClick={onClickReset}>DELETE ACCOUNT</Button>
    <div>
      <Button className="button ui two" onClick={onClickCancel}>Cancel</Button>
    </div>
  </>);
}

export default Reset;